//! Shared `sl-daemon` HTTP base URL for viewer API calls.
//!
//! Override at runtime with `SL_DAEMON_URL` or `SL_DAEMON_URLS` (see `.env.example`).

/// Default daemon base URL when `SL_DAEMON_URL` is not set at compile time.
pub const DEFAULT_DAEMON_BASE: &str = "http://127.0.0.1:8080";

const FALLBACK_DAEMON_BASES: [&str; 2] = ["http://127.0.0.1:9001", "http://localhost:9001"];

fn normalize_base(base: &str) -> Option<String> {
    let base = base.trim().trim_end_matches('/');
    if base.is_empty() {
        return None;
    }
    Some(base.to_string())
}

fn collect_candidate_base(
    candidate: &str,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<String>,
) {
    if let Some(base) = normalize_base(candidate) {
        let key = base.to_lowercase();
        if seen.insert(key) {
            out.push(base);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn runtime_candidates_raw_urls() -> Vec<String> {
    let mut candidates = Vec::new();
    if let Ok(value) = std::env::var("SL_DAEMON_URL") {
        candidates.extend(
            value
                .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToString::to_string),
        );
    }
    if let Ok(value) = std::env::var("SL_DAEMON_URLS") {
        candidates.extend(
            value
                .split(|c: char| c == ',' || c == ';' || c.is_whitespace())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToString::to_string),
        );
    }
    candidates
}

#[cfg(target_arch = "wasm32")]
fn runtime_candidates_raw_urls() -> Vec<String> {
    Vec::new()
}

/// Candidate daemon base URLs (in priority order):
///   - explicit runtime `SL_DAEMON_URL`
///   - explicit runtime `SL_DAEMON_URLS`
///   - compile-time `SL_DAEMON_URL`
///   - fallback defaults (`8080` and `9001` endpoints)
pub fn daemon_base_url_candidates() -> Vec<String> {
    let mut candidates = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for candidate in runtime_candidates_raw_urls() {
        collect_candidate_base(&candidate, &mut candidates, &mut seen);
    }
    if let Some(url) = normalize_base(option_env!("SL_DAEMON_URL").unwrap_or("")) {
        collect_candidate_base(&url, &mut candidates, &mut seen);
    }
    collect_candidate_base(DEFAULT_DAEMON_BASE, &mut candidates, &mut seen);
    for fallback in FALLBACK_DAEMON_BASES {
        collect_candidate_base(fallback, &mut candidates, &mut seen);
    }

    if candidates.is_empty() {
        vec![DEFAULT_DAEMON_BASE.to_string()]
    } else {
        candidates
    }
}

/// Resolved daemon base URL (no trailing slash).
pub fn daemon_base_url() -> String {
    daemon_base_url_candidates()
        .into_iter()
        .next()
        .unwrap_or_else(|| DEFAULT_DAEMON_BASE.to_string())
}

fn base_to_api(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{base}/{path}")
}

/// Build a full URL for a daemon API path using the highest-priority candidate.
pub fn daemon_api_url(path: &str) -> String {
    base_to_api(&daemon_base_url(), path)
}

/// Build URLs for all daemon candidates for the same API path.
pub fn daemon_api_url_options(path: &str) -> Vec<String> {
    daemon_base_url_candidates().into_iter().map(|base| base_to_api(&base, path)).collect()
}

/// Human-readable host display for error messages (scheme stripped).
pub fn daemon_host_display() -> String {
    daemon_base_url_candidates()
        .into_iter()
        .map(|base| base.trim_start_matches("http://").trim_start_matches("https://").to_owned())
        .collect::<Vec<_>>()
        .join(" or ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_daemon_base_is_localhost_8080() {
        assert_eq!(DEFAULT_DAEMON_BASE, "http://127.0.0.1:8080");
    }

    #[test]
    fn daemon_api_url_joins_base_and_path() {
        let url = daemon_api_url("/api/stream");
        assert!(url.ends_with("/api/stream"), "got {url}");
        assert!(url.starts_with("http://"), "got {url}");
    }

    #[test]
    fn daemon_api_url_accepts_path_without_leading_slash() {
        let url = daemon_api_url("api/search");
        assert!(url.ends_with("/api/search"), "got {url}");
    }

    #[test]
    fn daemon_host_display_strips_scheme() {
        let display = daemon_host_display();
        assert!(!display.starts_with("http"), "got {display}");
        assert!(display.contains("8080"), "got {display}");
    }
}
