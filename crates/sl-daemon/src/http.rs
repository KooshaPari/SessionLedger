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

use axum::body::to_bytes;
use axum::extract::Request;
use axum::extract::{Path as AxumPath, Query, State};
use axum::http::{header::AUTHORIZATION, header::CONTENT_TYPE, HeaderMap, HeaderValue};
use axum::middleware::{self, Next};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::{broadcast, Semaphore};

use crate::audit::{self, AuditSink};
use crate::export::BundleMeta;
use crate::filter::{apply_filters, FilterSpec};
use crate::metrics::{compute_metrics, HttpMetrics};
use crate::validation::{validate_okf_bundle, PostBundle};
#[cfg(feature = "otel")]
use opentelemetry::trace::{
    SpanContext, SpanId, TraceContextExt as _, TraceFlags, TraceId, TraceState,
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn, Instrument as _};
#[cfg(feature = "otel")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

const DEFAULT_INGEST_MAX_BODY_BYTES: usize = 1_048_576;
const DEFAULT_INGEST_MAX_CONCURRENCY: usize = 8;
const SL_API_KEY: &str = "SL_API_KEY";
const X_API_KEY: &str = "x-api-key";

/// Optional shared-secret authentication for mutating HTTP routes.
#[derive(Clone, Debug, Default)]
pub(crate) struct ApiKeyAuth {
    expected: Option<Arc<str>>,
}

impl ApiKeyAuth {
    pub(crate) fn from_env() -> Self {
        Self::from_value(std::env::var(SL_API_KEY).ok())
    }

    fn from_value(value: Option<String>) -> Self {
        let expected = value
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .map(Arc::<str>::from);
        Self { expected }
    }

    fn allows(&self, headers: &HeaderMap) -> bool {
        let Some(expected) = self.expected.as_deref() else {
            return true;
        };
        bearer_token_matches(headers, expected) || x_api_key_matches(headers, expected)
    }
}

/// Process-local admission controls for `POST /api/ingest`.
#[derive(Clone)]
pub(crate) struct IngestAdmission {
    max_body_bytes: usize,
    semaphore: Arc<Semaphore>,
}

impl IngestAdmission {
    pub(crate) fn from_env() -> Result<Self, String> {
        Self::from_values(
            std::env::var("SL_INGEST_MAX_BODY_BYTES").ok(),
            std::env::var("SL_INGEST_MAX_CONCURRENCY").ok(),
        )
    }

    fn from_values(
        max_body_bytes: Option<String>,
        max_concurrency: Option<String>,
    ) -> Result<Self, String> {
        let max_body_bytes = parse_positive_limit(
            "SL_INGEST_MAX_BODY_BYTES",
            max_body_bytes.as_deref(),
            DEFAULT_INGEST_MAX_BODY_BYTES,
        )?;
        let max_concurrency = parse_positive_limit(
            "SL_INGEST_MAX_CONCURRENCY",
            max_concurrency.as_deref(),
            DEFAULT_INGEST_MAX_CONCURRENCY,
        )?;
        Ok(Self { max_body_bytes, semaphore: Arc::new(Semaphore::new(max_concurrency)) })
    }
}

fn parse_positive_limit(name: &str, value: Option<&str>, default: usize) -> Result<usize, String> {
    let Some(value) = value else {
        return Ok(default);
    };
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("{name} must be a positive integer, got {value:?}"))?;
    if parsed == 0 {
        return Err(format!("{name} must be greater than zero"));
    }
    Ok(parsed)
}

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
    /// Body-size and in-flight request limits for ingest.
    pub ingest_admission: IngestAdmission,
    /// Optional shared-secret authentication for mutating routes.
    pub api_key_auth: ApiKeyAuth,
    /// Durable local audit sink for structured actor/action events.
    pub audit_sink: Arc<AuditSink>,
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
        .fallback(not_found)
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
        api_error(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "not_ready",
            format!("out_dir not ready: {}", path.display()),
        )
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
            api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "bundle_read_failed",
                format!("failed to read bundles: {e}"),
            )
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
#[cfg(feature = "otel")]
const TRACESTATE: &str = "tracestate";
const X_REQUEST_ID: &str = "x-request-id";

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

fn request_id_from_headers(headers: &HeaderMap) -> String {
    headers
        .get(X_REQUEST_ID)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            headers
                .get(TRACEPARENT)
                .and_then(|value| value.to_str().ok())
                .and_then(parse_traceparent)
                .map(|parsed| parsed.trace_id)
        })
        .unwrap_or_else(audit::local_request_id)
}

#[cfg(feature = "otel")]
fn remote_parent_context(
    headers: &HeaderMap,
    traceparent: &TraceParent,
) -> Option<opentelemetry::Context> {
    let trace_id = TraceId::from_hex(&traceparent.trace_id).ok()?;
    let span_id = SpanId::from_hex(&traceparent.parent_id).ok()?;
    let flags = u8::from_str_radix(&traceparent.flags, 16).ok()?;
    let trace_flags =
        if flags & 0x01 == 0x01 { TraceFlags::SAMPLED } else { TraceFlags::default() };
    let trace_state = headers
        .get(TRACESTATE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<TraceState>().ok())
        .unwrap_or_default();
    let span_context = SpanContext::new(trace_id, span_id, trace_flags, true, trace_state);

    Some(opentelemetry::Context::current().with_remote_span_context(span_context))
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
    #[cfg(feature = "otel")]
    if let Some((_, parsed)) = propagated.as_ref() {
        if let Some(parent_context) = remote_parent_context(request.headers(), parsed) {
            if let Err(error) = span.set_parent(parent_context) {
                warn!(?error, "failed to attach remote OTel parent context");
            }
        }
    }
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
            return api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "bundle_read_failed",
                format!("failed to read bundles: {e}"),
            );
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
#[tracing::instrument(skip(state, request))]
async fn ingest_bundle(State(state): State<AppState>, request: Request) -> Response {
    let request_id = request_id_from_headers(request.headers());
    if !state.api_key_auth.allows(request.headers()) {
        audit_event(&state.audit_sink, "ingest", "rejected", "unauthorized", &request_id);
        return api_error(
            axum::http::StatusCode::UNAUTHORIZED,
            "unauthorized",
            "missing or invalid API key",
        );
    }
    let Ok(_permit) = state.ingest_admission.semaphore.clone().try_acquire_owned() else {
        audit_event(&state.audit_sink, "ingest", "rejected", "concurrency_limit", &request_id);
        return api_error(
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            "ingest_busy",
            "too many concurrent ingest requests",
        );
    };
    let bytes = match to_bytes(request.into_body(), state.ingest_admission.max_body_bytes).await {
        Ok(bytes) => bytes,
        Err(_) => {
            audit_event(&state.audit_sink, "ingest", "rejected", "body_too_large", &request_id);
            return api_error(
                axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                format!("ingest payload exceeds {} bytes", state.ingest_admission.max_body_bytes),
            );
        }
    };
    let payload: PostBundle = match serde_json::from_slice(&bytes) {
        Ok(payload) => payload,
        Err(error) => {
            audit_event(&state.audit_sink, "ingest", "rejected", "invalid_json", &request_id);
            return api_error(
                axum::http::StatusCode::BAD_REQUEST,
                "invalid_json",
                format!("invalid JSON payload: {error}"),
            );
        }
    };
    let result = validate_okf_bundle(&payload);
    if result.valid {
        audit_event(&state.audit_sink, "ingest", "accepted", "validation", &request_id);
        Json(&result).into_response()
    } else {
        audit_event(&state.audit_sink, "ingest", "rejected", "validation", &request_id);
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

fn bearer_token_matches(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| token.trim() == expected)
}

fn x_api_key_matches(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get(X_API_KEY)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.trim() == expected)
}

#[derive(Serialize)]
struct ApiErrorEnvelope {
    error: ApiError,
}

#[derive(Serialize)]
struct ApiError {
    code: &'static str,
    message: String,
}

fn api_error(
    status: axum::http::StatusCode,
    code: &'static str,
    message: impl Into<String>,
) -> Response {
    (status, Json(ApiErrorEnvelope { error: ApiError { code, message: message.into() } }))
        .into_response()
}

async fn not_found() -> Response {
    api_error(axum::http::StatusCode::NOT_FOUND, "not_found", "route not found")
}

fn audit_event(
    sink: &AuditSink,
    action: &'static str,
    outcome: &'static str,
    reason: &'static str,
    request_id: &str,
) {
    info!(
        target: "sl_daemon::audit",
        event_kind = "audit",
        actor = audit::LOCAL_ACTOR,
        action,
        outcome,
        request_id,
        reason,
        "local operation"
    );
    let event = audit::AuditEvent {
        timestamp: audit::timestamp_unix_ms(),
        actor: audit::LOCAL_ACTOR,
        action,
        outcome,
        request_id,
        reason: Some(reason),
        resource: None,
    };
    if let Err(error) = sink.append(&event) {
        warn!(error = %error, path = %sink.path().display(), "failed to append audit event");
    }
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
        return api_error(
            axum::http::StatusCode::BAD_REQUEST,
            "invalid_bundle_id",
            "invalid bundle_id",
        );
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
            return api_error(
                axum::http::StatusCode::NOT_FOUND,
                "bundle_not_found",
                format!("bundle {bundle_id:?} not found"),
            );
        }
        Err(e) => {
            return api_error(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "bundle_read_failed",
                format!("failed to read bundle: {e}"),
            );
        }
    };

    let doc: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(e) => {
            return api_error(
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_bundle_json",
                format!("invalid OKF JSON: {e}"),
            );
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
    use tracing_test::traced_test;

    fn test_state(out_dir: &Path) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(1);
        AppState {
            out_dir: Arc::new(out_dir.to_owned()),
            broadcast_tx,
            http_metrics: Arc::new(HttpMetrics::default()),
            ingest_admission: IngestAdmission::from_values(None, None).unwrap(),
            api_key_auth: ApiKeyAuth::default(),
            audit_sink: Arc::new(AuditSink::new(out_dir)),
        }
    }

    fn test_state_with_api_key(out_dir: &Path, api_key: &str) -> AppState {
        let mut state = test_state(out_dir);
        state.api_key_auth = ApiKeyAuth::from_value(Some(api_key.to_owned()));
        state
    }

    fn valid_ingest_body() -> &'static str {
        r#"{
            "bundle_id": "bundle-auth-test",
            "created_at": "2026-07-13T21:40:00Z",
            "messages": [{"role": "user", "content": "hello"}],
            "token_count": 1
        }"#
    }

    async fn start_test_server(state: AppState) -> (SocketAddr, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, router(state)).await.unwrap();
        });
        (addr, server)
    }

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

    #[cfg(feature = "otel")]
    #[test]
    fn builds_remote_otel_parent_context_from_trace_context_headers() {
        use opentelemetry::trace::TraceContextExt as _;

        let traceparent = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
        let parsed = parse_traceparent(traceparent).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(TRACEPARENT, HeaderValue::from_static(traceparent));
        headers.insert(TRACESTATE, HeaderValue::from_static("rojo=00f067aa0ba902b7"));

        let context = remote_parent_context(&headers, &parsed).unwrap();
        let span_context = context.span().span_context().clone();

        assert!(span_context.is_remote());
        assert_eq!(span_context.trace_id().to_string(), parsed.trace_id);
        assert_eq!(span_context.span_id().to_string(), parsed.parent_id);
        assert!(span_context.is_sampled());
        assert_eq!(span_context.trace_state().header(), "rojo=00f067aa0ba902b7");
    }

    #[test]
    fn ingest_admission_defaults_and_rejects_invalid_values() {
        let defaults = IngestAdmission::from_values(None, None).unwrap();
        assert_eq!(defaults.max_body_bytes, DEFAULT_INGEST_MAX_BODY_BYTES);
        assert_eq!(defaults.semaphore.available_permits(), DEFAULT_INGEST_MAX_CONCURRENCY);
        assert!(IngestAdmission::from_values(Some("0".into()), None).is_err());
        assert!(IngestAdmission::from_values(None, Some("many".into())).is_err());
    }

    #[test]
    fn api_key_auth_ignores_unset_or_empty_env_values() {
        assert!(ApiKeyAuth::from_value(None).allows(&HeaderMap::new()));
        assert!(ApiKeyAuth::from_value(Some("  ".into())).allows(&HeaderMap::new()));
    }

    #[test]
    #[traced_test]
    fn audit_event_contains_actor_action_and_outcome() {
        let dir = tempfile::TempDir::new().unwrap();
        let sink = AuditSink::new(dir.path());
        audit_event(&sink, "ingest", "accepted", "validation", "req-test");
        assert!(logs_contain("actor=\"local\"") || logs_contain("actor=local"));
        assert!(logs_contain("action=\"ingest\""));
        assert!(logs_contain("outcome=\"accepted\""));
    }

    #[tokio::test]
    async fn ingest_without_api_key_keeps_loopback_trust_model() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let (addr, server) = start_test_server(test_state(out_dir.path())).await;

        let response = reqwest::Client::new()
            .post(format!("http://{addr}/api/ingest"))
            .header(CONTENT_TYPE, "application/json")
            .body(valid_ingest_body())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body: Value = response.json().await.unwrap();
        assert_eq!(body["valid"], true);
        server.abort();
    }

    #[tokio::test]
    async fn configured_api_key_allows_bearer_and_x_api_key_headers() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let (addr, server) =
            start_test_server(test_state_with_api_key(out_dir.path(), "secret-token")).await;
        let client = reqwest::Client::new();

        let bearer = client
            .post(format!("http://{addr}/api/ingest"))
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, "Bearer secret-token")
            .body(valid_ingest_body())
            .send()
            .await
            .unwrap();
        assert_eq!(bearer.status(), axum::http::StatusCode::OK);

        let x_api_key = client
            .post(format!("http://{addr}/api/ingest"))
            .header(CONTENT_TYPE, "application/json")
            .header(X_API_KEY, "secret-token")
            .body(valid_ingest_body())
            .send()
            .await
            .unwrap();
        assert_eq!(x_api_key.status(), axum::http::StatusCode::OK);
        server.abort();
    }

    #[tokio::test]
    async fn configured_api_key_denies_missing_or_invalid_credentials() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let (addr, server) =
            start_test_server(test_state_with_api_key(out_dir.path(), "secret-token")).await;
        let client = reqwest::Client::new();

        for request in [
            client
                .post(format!("http://{addr}/api/ingest"))
                .header(CONTENT_TYPE, "application/json")
                .body(valid_ingest_body()),
            client
                .post(format!("http://{addr}/api/ingest"))
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, "Bearer wrong-token")
                .body(valid_ingest_body()),
        ] {
            let response = request.send().await.unwrap();
            assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
            let body: Value = response.json().await.unwrap();
            assert_eq!(body["error"]["code"], "unauthorized");
            assert_eq!(body["error"]["message"], "missing or invalid API key");
        }
        server.abort();
    }

    #[tokio::test]
    async fn oversized_ingest_returns_json_error_envelope() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let mut state = test_state(out_dir.path());
        state.ingest_admission = IngestAdmission::from_values(Some("16".into()), None).unwrap();
        let (addr, server) = start_test_server(state).await;

        let response = reqwest::Client::new()
            .post(format!("http://{addr}/api/ingest"))
            .header(CONTENT_TYPE, "application/json")
            .body(r#"{"payload":"this is larger than sixteen bytes"}"#)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
        let body: Value = response.json().await.unwrap();
        assert_eq!(body["error"]["code"], "payload_too_large");
        assert!(body["error"]["message"].as_str().unwrap().contains("16 bytes"));
        server.abort();
    }

    #[tokio::test]
    async fn saturated_ingest_returns_json_too_many_requests() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let mut state = test_state(out_dir.path());
        state.ingest_admission = IngestAdmission {
            max_body_bytes: DEFAULT_INGEST_MAX_BODY_BYTES,
            semaphore: Arc::new(Semaphore::new(0)),
        };
        let (addr, server) = start_test_server(state).await;

        let response = reqwest::Client::new()
            .post(format!("http://{addr}/api/ingest"))
            .header(CONTENT_TYPE, "application/json")
            .body("{}")
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::TOO_MANY_REQUESTS);
        let body: Value = response.json().await.unwrap();
        assert_eq!(body["error"]["code"], "ingest_busy");
        server.abort();
    }

    #[tokio::test]
    async fn http_observability_middleware_propagates_and_counts() {
        let out_dir = tempfile::TempDir::new().unwrap();
        let state = test_state(out_dir.path());
        let (addr, server) = start_test_server(state).await;
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
