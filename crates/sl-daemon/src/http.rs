//! HTTP bridge for the sl-daemon OKF pipeline.
//!
//! Exposes three endpoints:
//!
//! * `GET /healthz` — liveness probe; returns `200 ok`.
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

use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use futures_core::Stream;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::broadcast;

use crate::export::BundleMeta;
use crate::filter::{apply_filters, FilterSpec};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;
use tower_http::cors::{Any, CorsLayer};

/// Shared state threaded into every handler.
#[derive(Clone)]
pub(crate) struct AppState {
    /// Directory that the ETL consumer writes `*.okf.json` files into.
    pub out_dir: Arc<PathBuf>,
    /// Broadcast receiver factory: each SSE connection subscribes to a fresh
    /// receiver. The sender lives in the ETL consumer task.
    pub broadcast_tx: broadcast::Sender<PathBuf>,
}

/// Build the axum [`Router`].
pub(crate) fn router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    Router::new()
        .route("/healthz", get(healthz))
        .route("/api/bundles", get(list_bundles))
        .route("/api/search", get(search_bundles))
        .route("/api/stream", get(sse_stream))
        .with_state(state)
        .layer(cors)
}

/// Run the axum HTTP server on `addr` until `shutdown` resolves.
///
/// Returns immediately if binding fails (caller logs the error).
pub async fn serve(
    addr: SocketAddr,
    state: AppState,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> std::io::Result<()> {
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown).await.map_err(std::io::Error::other)
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// `GET /healthz` — liveness probe.
async fn healthz() -> Response {
    "ok".into_response()
}

/// `GET /api/bundles` — return all `*.okf.json` documents as a JSON array.
async fn list_bundles(State(state): State<AppState>) -> Response {
    match read_all_bundles(&state.out_dir) {
        Ok(values) => Json(values).into_response(),
        Err(e) => {
            let msg = format!("failed to read bundles: {e}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        }
    }
}

/// `GET /api/stream` — SSE; one event per newly-written `*.okf.json` path.
async fn sse_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
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
async fn search_bundles(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Response {
    let raw = match read_all_bundles(&state.out_dir) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("failed to read bundles: {e}");
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
        }
    };

    let metas: Vec<BundleMeta> = raw.iter().map(BundleMeta::from_value).collect();
    let spec = params_to_spec(&params);
    let matched: Vec<BundleMeta> = apply_filters(&metas, &spec).into_iter().cloned().collect();

    Json(matched).into_response()
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
}
