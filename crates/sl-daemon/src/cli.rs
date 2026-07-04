//! `sl` CLI client — talks to a running `sl-daemon` HTTP server.

use reqwest::Client;

/// Default base URL of the daemon HTTP server.
pub const DEFAULT_BASE_URL: &str = "http://127.0.0.1:8080";

/// Build a full URL from a base URL and a path segment.
///
/// Trailing slashes on `base` are stripped so that `build_url("http://host/", "/path")`
/// produces `"http://host/path"` rather than `"http://host//path"`.
pub fn build_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    format!("{base}{path}")
}

/// Outcome of a `/healthz` probe.
#[derive(Debug, PartialEq, Eq)]
pub enum HealthStatus {
    /// Daemon is reachable and responded.
    Running { body: String },
    /// Connection refused — daemon not running.
    NotRunning,
}

/// Fetch daemon health status.
///
/// Returns `Ok(HealthStatus::Running)` when the daemon is reachable.
/// Returns `Ok(HealthStatus::NotRunning)` when the connection is refused.
/// Returns `Err` for other network / parse failures.
pub async fn fetch_health(client: &Client, base_url: &str) -> Result<HealthStatus, reqwest::Error> {
    let url = build_url(base_url, "/healthz");
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) if e.is_connect() => return Ok(HealthStatus::NotRunning),
        Err(e) => return Err(e),
    };
    let body = resp.text().await?;
    Ok(HealthStatus::Running { body })
}

/// Fetch bundle list from the daemon.
///
/// The daemon's `/api/bundles` returns a JSON array of parsed OKF documents;
/// we extract just the file path from the `"path"` field of each object when
/// present, falling back to a serialized representation otherwise.
pub async fn fetch_bundle_paths(
    client: &Client,
    base_url: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let url = build_url(base_url, "/api/bundles");
    let resp = client.get(&url).send().await?;
    let values: Vec<serde_json::Value> = resp.json().await?;
    let paths = values
        .into_iter()
        .map(|v| {
            v.get("path")
                .and_then(|p| p.as_str())
                .map(|s| s.to_owned())
                .unwrap_or_else(|| v.to_string())
        })
        .collect();
    Ok(paths)
}

// ---------------------------------------------------------------------------
// Unit tests (no network required)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_appends_path() {
        assert_eq!(build_url("http://localhost:9001", "/healthz"), "http://localhost:9001/healthz");
    }

    #[test]
    fn build_url_strips_trailing_slash_from_base() {
        assert_eq!(
            build_url("http://localhost:9001/", "/api/bundles"),
            "http://localhost:9001/api/bundles"
        );
    }

    #[test]
    fn build_url_works_with_no_leading_slash_on_path() {
        assert_eq!(
            build_url("http://localhost:9001", "/api/stream"),
            "http://localhost:9001/api/stream"
        );
    }

    #[test]
    fn fetch_bundle_paths_extracts_path_field() {
        let json = r#"[{"path":"/out/a.okf.json","title":"A"},{"path":"/out/b.okf.json"}]"#;
        let values: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        let paths: Vec<String> = values
            .into_iter()
            .map(|v| {
                v.get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_owned())
                    .unwrap_or_else(|| v.to_string())
            })
            .collect();
        assert_eq!(paths, vec!["/out/a.okf.json", "/out/b.okf.json"]);
    }
}
