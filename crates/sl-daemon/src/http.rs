//! HTTP bridge for the sl-daemon OKF pipeline.
//!
//! Exposes three endpoints:
//!
//! * `GET /healthz` — liveness probe; returns `200 ok`.
//! * `GET /readyz` — readiness probe; returns `200` when `out_dir` is usable.
//! * `GET /api/bundles` — reads all `*.okf.json` files currently in the output
//!   directory and returns them as a JSON array. Each element is the parsed
//!   document as a [`serde_json::Value`] so the response is decoupled from any
//!   viewer-side type definitions.
//! * `GET /api/stream` — SSE endpoint (`text/event-stream`) that emits a
//!   `data: <path>\n\n` event whenever a new `*.okf.json` is written. Driven by
//!   a [`tokio::sync::broadcast`] channel whose sender is populated by the ETL
//!   consumer after each successful write.
//!
//! CORS is enabled with a permissive policy so WASM-based viewers (e.g. Dioxus)
//! running on a different origin can call these endpoints.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use axum::extract::Request;
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{header::CONTENT_TYPE, HeaderValue};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_core::Stream;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::broadcast;

use crate::export::BundleMeta;
use crate::filter::{apply_filters, FilterSpec};
use crate::metrics::{compute_metrics, HttpMetrics};
use crate::validation::{validate_okf_bundle, PostBundle};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn, Instrument as _};

/// Shared state threaded into every handler.
#[derive(Clone)]
pub(crate) struct AppState {
    /// Directory that the ETL consumer writes `*.okf.json` files into.
    pub out_dir: Arc<PathBuf>,
    /// Broadcast receiver factory: each SSE connection subscribes to a fresh
    /// receiver. The sender lives in the ETL consumer task.
    pub broadcast_tx: broadcast::Sender<PathBuf>,
    /// Process-local RED counters for the Prometheus scrape endpoint.
    pub http_metrics: Arc<HttpMetrics>,
}

/// Build the axum [`Router`].
pub(crate) fn router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let http_metrics = state.http_metrics.clone();

    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/bundles", get(list_bundles))
        .route("/api/search", get(search_bundles))
        .route("/api/stream", get(sse_stream))
        .route("/api/replay/{bundle_id}", get(replay_bundle))
        .route("/api/ingest", post(ingest_bundle))
        .route("/api/metrics", get(metrics_handler))
        .route("/metrics", get(prometheus_metrics))
        .with_state(state)
        .layer(middleware::from_fn_with_state(http_metrics, observe_request))
        .layer(cors)
}

/// Run the axum HTTP server on `addr` until `shutdown` resolves.
///
/// Returns immediately if binding fails (caller logs the error).
#[tracing::instrument(skip(state, shutdown), fields(addr = %addr))]
pub async fn serve(
    addr: SocketAddr,
    state: AppState,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(%addr, "HTTP server bound");
    axum::serve(listener, app).with_graceful_shutdown(shutdown).await.map_err(std::io::Error::other)
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// `GET /healthz` — liveness probe.
#[tracing::instrument]
async fn healthz() -> Response {
    "ok".into_response()
}

/// `GET /readyz` — readiness probe (output directory must exist and be a dir).
#[tracing::instrument(skip(state), fields(out_dir = %state.out_dir.display()))]
async fn readyz(State(state): State<AppState>) -> Response {
    let path = state.out_dir.as_path();
    if path.is_dir() {
        "ready".into_response()
    } else {
        warn!(out_dir = %path.display(), "readyz: out_dir not ready");
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            format!("out_dir not ready: {}", path.display()),
        )
            .into_response()
    }
}

/// `GET /api/bundles` — return all `*.okf.json` documents as a JSON array.
#[tracing::instrument(skip(state), fields(out_dir = %state.out_dir.display()))]
async fn list_bundles(State(state): State<AppState>) -> Response {
    match read_all_bundles(&state.out_dir) {
        Ok(values) => {
            info!(count = values.len(), "list_bundles");
            Json(values).into_response()
        }
        Err(e) => {
            error!(error = %e, "failed to read bundles");
            let msg = format!("failed to read bundles: {e}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        }
    }
}

/// `GET /api/metrics` — aggregate token/model stats over the output directory.
#[tracing::instrument(skip(state), fields(out_dir = %state.out_dir.display()))]
async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    Json(compute_metrics(&state.out_dir))
}

/// `GET /metrics` — process-local HTTP RED metrics in Prometheus text format.
async fn prometheus_metrics(State(state): State<AppState>) -> Response {
    (
        [(CONTENT_TYPE, HeaderValue::from_static("text/plain; version=0.0.4; charset=utf-8"))],
        state.http_metrics.render_prometheus(),
    )
        .into_response()
}

const TRACEPARENT: &str = "traceparent";

#[derive(Debug, PartialEq, Eq)]
struct TraceParent {
    trace_id: String,
    parent_id: String,
    flags: String,
}

/// Parse the commonly deployed W3C trace-context version (`00`).
fn parse_traceparent(value: &str) -> Option<TraceParent> {
    if value.len() != 55 || !value.is_ascii() {
        return None;
    }
    let bytes = value.as_bytes();
    if &bytes[0..3] != b"00-" || bytes[35] != b'-' || bytes[52] != b'-' {
        return None;
    }
    let trace_id = &value[3..35];
    let parent_id = &value[36..52];
    let flags = &value[53..55];
    let is_lower_hex = |part: &str| {
        part.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    };
    if !is_lower_hex(trace_id)
        || !is_lower_hex(parent_id)
        || !is_lower_hex(flags)
        || trace_id.bytes().all(|byte| byte == b'0')
        || parent_id.bytes().all(|byte| byte == b'0')
    {
        return None;
    }
    Some(TraceParent {
        trace_id: trace_id.to_owned(),
        parent_id: parent_id.to_owned(),
        flags: flags.to_owned(),
    })
}

/// Count and trace every HTTP request while preserving valid upstream context.
async fn observe_request(
    State(metrics): State<Arc<HttpMetrics>>,
    request: Request,
    next: Next,
) -> Response {
    metrics.request_started();
    let started = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let propagated = request
        .headers()
        .get(TRACEPARENT)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| parse_traceparent(value).map(|parsed| (value.to_owned(), parsed)));
    let (traceparent, trace_id, parent_span_id, trace_flags) =
        propagated.as_ref().map_or((None, "", "", ""), |(value, parsed)| {
            (
                Some(value.clone()),
                parsed.trace_id.as_str(),
                parsed.parent_id.as_str(),
                parsed.flags.as_str(),
            )
        });
    let span = tracing::info_span!(
        "http.request",
        http.method = %method,
        http.route = %path,
        trace_id = %trace_id,
        parent_span_id = %parent_span_id,
        trace_flags = %trace_flags,
    );
    let mut response = next.run(request).instrument(span).await;
    if let Some(value) = traceparent.and_then(|value| HeaderValue::from_str(&value).ok()) {
        response.headers_mut().insert(TRACEPARENT, value);
    }
    let is_error = response.status().is_client_error() || response.status().is_server_error();
    let elapsed = started.elapsed().as_micros().try_into().unwrap_or(u64::MAX);
    metrics.request_completed(is_error, elapsed);
    response
}

/// `GET /api/stream` — SSE; one event per newly-written `*.okf.json` path.
#[tracing::instrument(skip(state))]
async fn sse_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("SSE client subscribed");
    let rx = state.broadcast_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| {
        result
            .ok()
            .map(|path| Ok(Event::default().event("bundle").data(path.display().to_string())))
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Query parameters accepted by `GET /api/search`.
///
/// All fields are optional; omitted fields mean "no constraint on that axis".
/// `tags` is a comma-separated string (e.g. `?tags=rust,ml`) that is split on
/// the server side before being handed to [`apply_filters`].
#[derive(Debug, Deserialize)]
pub(crate) struct SearchParams {
    /// ISO date lower bound, e.g. `2024-01-01`.
    pub since: Option<String>,
    /// ISO date upper bound, e.g. `2024-12-31`.
    pub until: Option<String>,
    /// Case-insensitive model substring, e.g. `claude`.
    pub model: Option<String>,
    /// Minimum token count.
    pub min_tokens: Option<u64>,
    /// Comma-separated tags; ALL must be present (AND logic).
    pub tags: Option<String>,
    /// Maximum results (default 50).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    50
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            since: None,
            until: None,
            model: None,
            min_tokens: None,
            tags: None,
            limit: default_limit(),
        }
    }
}

/// Build a [`FilterSpec`] from HTTP query parameters.
///
/// This is a pure function so it can be unit-tested without a running server.
pub(crate) fn params_to_spec(p: &SearchParams) -> FilterSpec {
    let tags: Vec<String> = p
        .tags
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();

    FilterSpec {
        since: p.since.clone(),
        until: p.until.clone(),
        model: p.model.clone(),
        min_tokens: p.min_tokens,
        tags,
        limit: p.limit,
    }
}

/// `GET /api/search` — filter bundles by date, model, tokens, and tags.
///
/// Returns a JSON array of [`BundleMeta`] objects matching the query.
#[tracing::instrument(skip(state, params), fields(out_dir = %state.out_dir.display(), limit = params.limit))]
async fn search_bundles(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Response {
    let raw = match read_all_bundles(&state.out_dir) {
        Ok(v) => v,
        Err(e) => {
            error!(error = %e, "failed to read bundles for search");
            let msg = format!("failed to read bundles: {e}");
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
        }
    };

    let metas: Vec<BundleMeta> = raw.iter().map(BundleMeta::from_value).collect();
    let spec = params_to_spec(&params);
    let matched: Vec<BundleMeta> = apply_filters(&metas, &spec).into_iter().cloned().collect();
    info!(matched = matched.len(), scanned = metas.len(), "search_bundles");

    Json(matched).into_response()
}

/// `POST /api/ingest` — validate an OKF bundle payload before accepting it.
///
/// Returns `200 OK` with the [`crate::validation::ValidationResult`] JSON when the
/// bundle passes all structural checks. Returns `422 Unprocessable Entity` with
/// the same JSON body when one or more validation errors are found. This allows
/// clients to distinguish a transport-level failure (4xx/5xx from the proxy or
/// server) from a business-logic rejection (422 with actionable error details).
#[tracing::instrument(skip(payload))]
async fn ingest_bundle(Json(payload): Json<PostBundle>) -> Response {
    let result = validate_okf_bundle(&payload);
    if result.valid {
        info!("ingest accepted");
        Json(&result).into_response()
    } else {
        warn!("ingest rejected by validation");
        (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(&result)).into_response()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read every `*.okf.json` file in `out_dir` and parse each as
/// [`serde_json::Value`].  Files that fail to parse are silently skipped so
/// a single corrupt file does not poison the entire listing.
fn read_all_bundles(out_dir: &Path) -> std::io::Result<Vec<Value>> {
    let mut results = Vec::new();

    let rd = match std::fs::read_dir(out_dir) {
        Ok(rd) => rd,
        // If the directory doesn't exist yet, return an empty list.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e),
    };

    for entry in rd {
        let entry = entry?;
        let path = entry.path();
        let is_okf =
            path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".okf.json"));
        if !is_okf {
            continue;
        }
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(value) = serde_json::from_str::<Value>(&contents) {
                results.push(value);
            }
        }
    }

    Ok(results)
}

// ---------------------------------------------------------------------------
// replay
// ---------------------------------------------------------------------------

/// Query parameters for `GET /api/replay/:bundle_id`.
#[derive(Debug, Deserialize)]
pub(crate) struct ReplayParams {
    /// Playback speed multiplier (default 1.0).  `speed=2.0` replays at 2×
    /// real-time by halving the inter-event delay.
    #[serde(default = "default_speed")]
    pub speed: f64,
}

fn default_speed() -> f64 {
    1.0
}

/// Base inter-event delay in milliseconds at speed = 1.0.
const BASE_DELAY_MS: f64 = 200.0;

/// Calculate the inter-event delay for a given speed multiplier.
///
/// Returns at least 1 ms so we never produce a zero-duration sleep.
pub(crate) fn delay_ms_for_speed(speed: f64) -> u64 {
    let speed = if speed <= 0.0 { 1.0 } else { speed };
    let ms = (BASE_DELAY_MS / speed).round() as u64;
    ms.max(1)
}

/// `GET /api/replay/:bundle_id` — SSE stream of OKF entities in order.
///
/// Opens `<out_dir>/<bundle_id>.okf.json`, reads all entities, and streams
/// each one as an SSE event with the following shape:
///
/// ```text
/// data: {"entity_index":0,"total_entities":5,"entity":{...}}\n\n
/// ```
///
/// After the last entity it sends:
/// ```text
/// event: done\ndata: {}\n\n
/// ```
///
/// The optional `?speed=<f>` query parameter (default 1.0) controls how
/// quickly entities are emitted: `speed=2.0` halves the inter-event delay.
#[tracing::instrument(skip(state, params), fields(bundle_id = %bundle_id, speed = params.speed, out_dir = %state.out_dir.display()))]
async fn replay_bundle(
    AxumPath(bundle_id): AxumPath<String>,
    Query(params): Query<ReplayParams>,
    State(state): State<AppState>,
) -> Response {
    // Sanitise the bundle_id: reject any path traversal.
    if bundle_id.contains('/') || bundle_id.contains('\\') || bundle_id.contains("..") {
        warn!(%bundle_id, "invalid bundle_id");
        return (axum::http::StatusCode::BAD_REQUEST, "invalid bundle_id").into_response();
    }

    let filename = if bundle_id.ends_with(".okf.json") {
        bundle_id.clone()
    } else {
        format!("{bundle_id}.okf.json")
    };

    let path = state.out_dir.join(&filename);
    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return (axum::http::StatusCode::NOT_FOUND, format!("bundle {bundle_id:?} not found"))
                .into_response();
        }
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read bundle: {e}"),
            )
                .into_response();
        }
    };

    let doc: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(e) => {
            return (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                format!("invalid OKF JSON: {e}"),
            )
                .into_response();
        }
    };

    // Extract the entities array; fall back to empty if absent.
    let entities: Vec<serde_json::Value> =
        doc.get("entities").and_then(|v| v.as_array()).cloned().unwrap_or_default();

    let delay = delay_ms_for_speed(params.speed);
    let total = entities.len();
    info!(entities = total, delay_ms = delay, "replay starting");

    let stream = async_stream::stream! {
        for (idx, entity) in entities.into_iter().enumerate() {
            if idx > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
            let payload = serde_json::json!({
                "event_index": idx,
                "total_events": total,
                "entity": entity,
            });
            let data = payload.to_string();
            yield Ok::<Event, Infallible>(Event::default().event("entity").data(data));
        }
        // Final done sentinel.
        yield Ok(Event::default().event("done").data("{}"));
    };

    Sse::new(stream).keep_alive(KeepAlive::default()).into_response()
}

// ---------------------------------------------------------------------------
// Unit tests for params_to_spec
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_params_produce_default_spec() {
        let p = SearchParams::default();
        let spec = params_to_spec(&p);
        assert!(spec.since.is_none());
        assert!(spec.until.is_none());
        assert!(spec.model.is_none());
        assert!(spec.min_tokens.is_none());
        assert!(spec.tags.is_empty());
        assert_eq!(spec.limit, 50);
    }

    #[test]
    fn since_until_model_min_tokens_propagate() {
        let p = SearchParams {
            since: Some("2024-01-01".into()),
            until: Some("2024-12-31".into()),
            model: Some("claude".into()),
            min_tokens: Some(1000),
            tags: None,
            limit: 10,
        };
        let spec = params_to_spec(&p);
        assert_eq!(spec.since.as_deref(), Some("2024-01-01"));
        assert_eq!(spec.until.as_deref(), Some("2024-12-31"));
        assert_eq!(spec.model.as_deref(), Some("claude"));
        assert_eq!(spec.min_tokens, Some(1000));
        assert_eq!(spec.limit, 10);
    }

    #[test]
    fn tags_comma_separated_parsed_correctly() {
        let p = SearchParams { tags: Some("rust, ml, perf".into()), ..SearchParams::default() };
        let spec = params_to_spec(&p);
        assert_eq!(spec.tags, vec!["rust", "ml", "perf"]);
    }

    #[test]
    fn empty_tags_string_yields_empty_vec() {
        let p = SearchParams { tags: Some(String::new()), ..SearchParams::default() };
        let spec = params_to_spec(&p);
        assert!(spec.tags.is_empty());
    }

    #[test]
    fn single_tag_no_trailing_comma() {
        let p = SearchParams { tags: Some("rust".into()), ..SearchParams::default() };
        let spec = params_to_spec(&p);
        assert_eq!(spec.tags, vec!["rust"]);
    }

    // --- replay helpers ---

    #[test]
    fn delay_speed_1_is_base() {
        assert_eq!(delay_ms_for_speed(1.0), 200);
    }

    #[test]
    fn delay_speed_2_is_half_base() {
        assert_eq!(delay_ms_for_speed(2.0), 100);
    }

    #[test]
    fn delay_speed_10_is_clamped_to_1ms_min() {
        // BASE_DELAY_MS(200) / 10 = 20ms — still above minimum.
        assert_eq!(delay_ms_for_speed(10.0), 20);
    }

    #[test]
    fn delay_zero_or_negative_defaults_to_base() {
        // Non-positive speed treated as 1.0.
        assert_eq!(delay_ms_for_speed(0.0), 200);
        assert_eq!(delay_ms_for_speed(-5.0), 200);
    }

    #[test]
    fn delay_very_high_speed_at_least_1ms() {
        // 200 / 1000 = 0.2 → rounds to 0 → clamped to 1.
        assert!(delay_ms_for_speed(1000.0) >= 1);
    }

    #[test]
    fn parses_valid_traceparent() {
        let parsed =
            parse_traceparent("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01").unwrap();
        assert_eq!(parsed.trace_id, "4bf92f3577b34da6a3ce929d0e0e4736");
        assert_eq!(parsed.parent_id, "00f067aa0ba902b7");
        assert_eq!(parsed.flags, "01");
    }

    #[test]
    fn rejects_malformed_or_zero_traceparent() {
        assert!(parse_traceparent("not-a-traceparent").is_none());
        assert!(
            parse_traceparent("00-00000000000000000000000000000000-00f067aa0ba902b7-01").is_none()
        );
        assert!(
            parse_traceparent("00-4bf92f3577b34da6a3ce929d0e0e4736-0000000000000000-01").is_none()
        );
    }

    #[tokio::test]
    async fn http_observability_middleware_propagates_and_counts() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let (broadcast_tx, _) = broadcast::channel(1);
        let state = AppState {
            out_dir: Arc::new(out_dir.path().to_owned()),
            broadcast_tx,
            http_metrics: Arc::new(HttpMetrics::default()),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, router(state)).await.unwrap();
        });
        let base_url = format!("http://{addr}");
        let traceparent = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
        let client = reqwest::Client::new();

        let health = client
            .get(format!("{base_url}/healthz"))
            .header(TRACEPARENT, traceparent)
            .send()
            .await
            .unwrap();
        assert_eq!(health.headers().get(TRACEPARENT).unwrap(), traceparent);

        let metrics =
            client.get(format!("{base_url}/metrics")).send().await.unwrap().text().await.unwrap();
        assert!(metrics.contains("sl_http_requests_total 2"));
        assert!(metrics.contains("sl_http_errors_total 0"));
        server.abort();
    }
}
