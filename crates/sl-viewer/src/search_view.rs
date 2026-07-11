//! Search / filter view for the sl-viewer.
//!
//! Renders a form that calls `GET /api/search` on the sl-daemon HTTP API
//! (`127.0.0.1:8080`) and displays results in the same card style used by the
//! Bundles tab.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::async_states::{ErrorState, LoadingState};

/// Slim bundle metadata returned by `GET /api/search`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SearchResult {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub token_count: u64,
    #[serde(default)]
    pub message_count: u64,
    #[serde(default)]
    pub duration_ms: u64,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Daemon base URL used for all HTTP calls.
const DAEMON_BASE: &str = "http://127.0.0.1:8080";

/// Build the `/api/search` query string from form values.
///
/// This is a pure function exposed for unit-testing.
pub fn build_query(
    since: &str,
    until: &str,
    model: &str,
    min_tokens: &str,
    tags: &str,
    limit: &str,
) -> String {
    let mut parts: Vec<String> = Vec::new();

    let since = since.trim();
    if !since.is_empty() {
        parts.push(format!("since={}", urlencoding(since)));
    }
    let until = until.trim();
    if !until.is_empty() {
        parts.push(format!("until={}", urlencoding(until)));
    }
    let model = model.trim();
    if !model.is_empty() {
        parts.push(format!("model={}", urlencoding(model)));
    }
    let min_tokens = min_tokens.trim();
    if !min_tokens.is_empty() {
        parts.push(format!("min_tokens={}", urlencoding(min_tokens)));
    }
    let tags = tags.trim();
    if !tags.is_empty() {
        // Comma-separated tags; passed as single `tags` query param and split
        // by the daemon.
        parts.push(format!("tags={}", urlencoding(tags)));
    }
    let limit = limit.trim();
    let limit_val = limit.parse::<usize>().unwrap_or(50);
    parts.push(format!("limit={limit_val}"));

    parts.join("&")
}

/// Minimal percent-encoder — encodes the characters that break query strings.
fn urlencoding(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            ' ' => vec!['%', '2', '0'],
            ',' => vec!['%', '2', 'C'],
            '#' => vec!['%', '2', '3'],
            '&' => vec!['%', '2', '6'],
            '=' => vec!['%', '3', 'D'],
            '+' => vec!['%', '2', 'B'],
            _ => vec![c],
        })
        .collect()
}

/// Search / filter panel.
///
/// Input fields: `since`, `until`, `model`, `min_tokens`, `tags` (comma-
/// separated), `limit`.  Clicking **Search** fires `GET /api/search` on the
/// sl-daemon and renders results as bundle cards.  **Clear** resets all fields.
#[component]
pub fn SearchView() -> Element {
    let mut since = use_signal(String::new);
    let mut until = use_signal(String::new);
    let mut model = use_signal(String::new);
    let mut min_tokens = use_signal(String::new);
    let mut tags = use_signal(String::new);
    let mut limit = use_signal(|| "50".to_string());

    let mut results: Signal<Vec<SearchResult>> = use_signal(Vec::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let loading = use_signal(|| false);
    let mut selected_idx: Signal<Option<usize>> = use_signal(|| None);

    let mut search_tick: Signal<u32> = use_signal(|| 0u32);

    // Shared Search / Retry path — bump `search_tick` to fire a fetch.
    use_effect(move || {
        let tick = search_tick();
        if tick == 0 {
            return;
        }
        let qs = build_query(&since(), &until(), &model(), &min_tokens(), &tags(), &limit());
        let url = format!("{DAEMON_BASE}/api/search?{qs}");
        let mut results = results;
        let mut error = error;
        let mut loading = loading;

        loading.set(true);
        error.set(None);

        spawn(async move {
            #[cfg(feature = "web")]
            {
                use_web_fetch(url, results, error, loading).await;
            }
            #[cfg(not(feature = "web"))]
            {
                match reqwest_search(&url).await {
                    Ok(data) => {
                        results.set(data);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            }
        });
    });

    let on_clear = move |_| {
        since.set(String::new());
        until.set(String::new());
        model.set(String::new());
        min_tokens.set(String::new());
        tags.set(String::new());
        limit.set("50".to_string());
        results.set(Vec::new());
        error.set(None);
        selected_idx.set(None);
    };

    let result_count = results.len();
    let plural = if result_count == 1 { "" } else { "s" };

    rsx! {
        div { class: "search-view",
            // ---- Filter form ----
            div { class: "search-form",
                h2 { style: "padding: 16px 20px; margin: 0; font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: #8b8fa3; border-bottom: 1px solid #2a2d35;",
                    "Search Bundles"
                }
                div { class: "search-fields",
                    label { class: "search-label", "Since (YYYY-MM-DD)" }
                    input {
                        class: "search-input",
                        placeholder: "2024-01-01",
                        value: "{since}",
                        oninput: move |e| since.set(e.value()),
                    }
                    label { class: "search-label", "Until (YYYY-MM-DD)" }
                    input {
                        class: "search-input",
                        placeholder: "2024-12-31",
                        value: "{until}",
                        oninput: move |e| until.set(e.value()),
                    }
                    label { class: "search-label", "Model (substring)" }
                    input {
                        class: "search-input",
                        placeholder: "claude",
                        value: "{model}",
                        oninput: move |e| model.set(e.value()),
                    }
                    label { class: "search-label", "Min Tokens" }
                    input {
                        class: "search-input",
                        r#type: "number",
                        placeholder: "1000",
                        value: "{min_tokens}",
                        oninput: move |e| min_tokens.set(e.value()),
                    }
                    label { class: "search-label", "Tags (comma-separated)" }
                    input {
                        class: "search-input",
                        placeholder: "rust, ml",
                        value: "{tags}",
                        oninput: move |e| tags.set(e.value()),
                    }
                    label { class: "search-label", "Limit" }
                    input {
                        class: "search-input",
                        r#type: "number",
                        value: "{limit}",
                        oninput: move |e| limit.set(e.value()),
                    }
                }
                div { class: "search-actions",
                    button {
                        class: "search-btn search-btn-primary",
                        onclick: move |_| search_tick.with_mut(|t| *t += 1),
                        if loading() { "Searching…" } else { "Search" }
                    }
                    button {
                        class: "search-btn",
                        onclick: on_clear,
                        "Clear"
                    }
                }
            }

            // ---- Results ----
            div { class: "search-results",
                if let Some(ref msg) = error() {
                    ErrorState {
                        message: msg.clone(),
                        retryable: true,
                        on_retry: move |_| search_tick.with_mut(|t| *t += 1),
                    }
                }
                if loading() {
                    LoadingState { message: "Searching bundles…".to_string() }
                }
                if !results.is_empty() && !loading() {
                    div { class: "session-count",
                        "{result_count} result{plural}"
                    }
                }
                for (idx, result) in results.iter().enumerate() {
                    {
                        let is_selected = selected_idx() == Some(idx);
                        let cls = if is_selected { "session-item selected" } else { "session-item" };
                        let r = result.clone();
                        let tags_display = r.tags.join(", ");
                        rsx! {
                            div {
                                key: "{r.session_id}-{idx}",
                                class: "{cls}",
                                onclick: move |_| selected_idx.set(Some(idx)),
                                div { class: "session-source", "{r.session_id}" }
                                div { class: "session-goal", "model: {r.model}" }
                                div { class: "session-meta",
                                    span { class: "meta-bundles", "{r.token_count} tokens" }
                                    span { style: "color: #8b8fa3;", "{r.created_at}" }
                                    if !tags_display.is_empty() {
                                        span { class: "badge badge-ok", "{tags_display}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Desktop HTTP fetch via a simple blocking-in-spawn call.
///
/// Returns the parsed results or an error string.
#[cfg(not(feature = "web"))]
async fn reqwest_search(url: &str) -> Result<Vec<SearchResult>, String> {
    // Dioxus desktop bundles tokio; we can do async HTTP directly.
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.map_err(|e| format!("daemon not reachable: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("daemon returned {status}: {body}"));
    }

    resp.json::<Vec<SearchResult>>().await.map_err(|e| format!("failed to parse response: {e}"))
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_query_all_fields() {
        let q = build_query("2024-01-01", "2024-12-31", "claude", "500", "rust,ml", "10");
        assert!(q.contains("since=2024-01-01"), "since present");
        assert!(q.contains("until=2024-12-31"), "until present");
        assert!(q.contains("model=claude"), "model present");
        assert!(q.contains("min_tokens=500"), "min_tokens present");
        assert!(q.contains("tags=rust"), "tags present");
        assert!(q.contains("limit=10"), "limit present");
    }

    #[test]
    fn build_query_empty_fields_omitted() {
        let q = build_query("", "", "", "", "", "50");
        assert!(!q.contains("since="), "since absent");
        assert!(!q.contains("until="), "until absent");
        assert!(!q.contains("model="), "model absent");
        assert!(!q.contains("min_tokens="), "min_tokens absent");
        assert!(!q.contains("tags="), "tags absent");
        assert!(q.contains("limit=50"), "default limit present");
    }

    #[test]
    fn build_query_invalid_limit_defaults_to_50() {
        let q = build_query("", "", "", "", "", "not_a_number");
        assert!(q.contains("limit=50"), "falls back to 50");
    }

    #[test]
    fn build_query_spaces_encoded() {
        let q = build_query("", "", "claude 3", "", "", "50");
        assert!(q.contains("model=claude%203"), "space encoded");
    }

    #[test]
    fn build_query_comma_in_tags_encoded() {
        let q = build_query("", "", "", "", "rust,ml", "50");
        // Comma is encoded as %2C in the tag list.
        assert!(q.contains("tags=rust%2Cml"), "comma encoded in tags");
    }
}
