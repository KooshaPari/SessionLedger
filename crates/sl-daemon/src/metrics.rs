//! GET /api/metrics — aggregated session statistics.
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use tracing::info;

/// Process-local HTTP RED counters exposed in Prometheus text format.
#[derive(Debug, Default)]
pub struct HttpMetrics {
    requests: AtomicU64,
    errors: AtomicU64,
    completed: AtomicU64,
    duration_micros: AtomicU64,
}

impl HttpMetrics {
    pub fn request_started(&self) {
        self.requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn request_completed(&self, is_error: bool, duration_micros: u64) {
        if is_error {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
        self.completed.fetch_add(1, Ordering::Relaxed);
        self.duration_micros.fetch_add(duration_micros, Ordering::Relaxed);
    }

    /// Render a dependency-free Prometheus/OpenMetrics-compatible text snapshot.
    pub fn render_prometheus(&self) -> String {
        let requests = self.requests.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        let completed = self.completed.load(Ordering::Relaxed);
        let duration_seconds = self.duration_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;

        format!(
            "# HELP sl_http_requests_total Total HTTP requests received.\n\
             # TYPE sl_http_requests_total counter\n\
             sl_http_requests_total {requests}\n\
             # HELP sl_http_errors_total HTTP responses with a 4xx or 5xx status.\n\
             # TYPE sl_http_errors_total counter\n\
             sl_http_errors_total {errors}\n\
             # HELP sl_http_request_duration_seconds Request duration summary.\n\
             # TYPE sl_http_request_duration_seconds summary\n\
             sl_http_request_duration_seconds_sum {duration_seconds:.6}\n\
             sl_http_request_duration_seconds_count {completed}\n"
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub total_bundles: u64,
    pub total_tokens: u64,
    pub avg_tokens: u64,
    pub model_counts: HashMap<String, u64>,
    pub daily_counts: HashMap<String, u64>,
}

#[tracing::instrument(skip_all, fields(out_dir = %out_dir.display()))]
pub fn compute_metrics(out_dir: &Path) -> MetricsSummary {
    let mut s = MetricsSummary::default();
    let Ok(rd) = std::fs::read_dir(out_dir) else { return s };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(&path) else { continue };
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) else { continue };
        s.total_bundles += 1;
        if let Some(t) = val.get("total_tokens").and_then(|v| v.as_u64()) {
            s.total_tokens += t;
        }
        if let Some(m) = val.get("model").and_then(|v| v.as_str()) {
            *s.model_counts.entry(m.to_string()).or_default() += 1;
        }
        if let Some(d) = val.get("created_at").and_then(|v| v.as_str()) {
            *s.daily_counts.entry(d[..10.min(d.len())].to_string()).or_default() += 1;
        }
    }
    if let Some(avg) = s.total_tokens.checked_div(s.total_bundles) {
        s.avg_tokens = avg;
    }
    info!(
        total_bundles = s.total_bundles,
        total_tokens = s.total_tokens,
        avg_tokens = s.avg_tokens,
        "metrics computed"
    );
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn mkbundle(dir: &std::path::Path, name: &str, model: &str, tokens: u64, date: &str) {
        let mut f = std::fs::File::create(dir.join(format!("{name}.json"))).unwrap();
        write!(
            f,
            r#"{{"model":"{model}","total_tokens":{tokens},"created_at":"{date}T00:00:00Z"}}"#
        )
        .unwrap();
    }
    #[test]
    fn empty_dir() {
        let d = tempfile::TempDir::new().unwrap();
        let m = compute_metrics(d.path());
        assert_eq!(m.total_bundles, 0);
    }
    #[test]
    fn counts_bundles() {
        let d = tempfile::TempDir::new().unwrap();
        mkbundle(d.path(), "a", "m", 100, "2026-07-01");
        mkbundle(d.path(), "b", "m", 200, "2026-07-01");
        let m = compute_metrics(d.path());
        assert_eq!(m.total_bundles, 2);
        assert_eq!(m.avg_tokens, 150);
    }
    #[test]
    fn model_breakdown() {
        let d = tempfile::TempDir::new().unwrap();
        mkbundle(d.path(), "a", "gpt-4", 10, "2026-07-01");
        mkbundle(d.path(), "b", "claude", 10, "2026-07-01");
        let m = compute_metrics(d.path());
        assert_eq!(m.model_counts["gpt-4"], 1);
    }
    #[test]
    fn daily_histogram() {
        let d = tempfile::TempDir::new().unwrap();
        mkbundle(d.path(), "a", "m", 10, "2026-07-01");
        mkbundle(d.path(), "b", "m", 10, "2026-07-02");
        let m = compute_metrics(d.path());
        assert_eq!(m.daily_counts["2026-07-01"], 1);
        assert_eq!(m.daily_counts["2026-07-02"], 1);
    }
    #[test]
    fn ignores_non_json() {
        let d = tempfile::TempDir::new().unwrap();
        std::fs::write(d.path().join("r.md"), "x").unwrap();
        assert_eq!(compute_metrics(d.path()).total_bundles, 0);
    }
    #[test]
    fn missing_dir() {
        assert_eq!(compute_metrics(std::path::Path::new("/nonexistent/xyz")).total_bundles, 0);
    }
    #[test]
    fn handles_malformed() {
        let d = tempfile::TempDir::new().unwrap();
        std::fs::write(d.path().join("bad.json"), "bad").unwrap();
        assert_eq!(compute_metrics(d.path()).total_bundles, 0);
    }
    #[test]
    fn zero_tokens_bundle() {
        let d = tempfile::TempDir::new().unwrap();
        std::fs::write(d.path().join("min.json"), "{}").unwrap();
        let m = compute_metrics(d.path());
        assert_eq!(m.total_bundles, 1);
        assert_eq!(m.total_tokens, 0);
    }

    #[test]
    fn prometheus_snapshot_contains_red_counters() {
        let metrics = HttpMetrics::default();
        metrics.request_started();
        metrics.request_completed(true, 250_000);
        let text = metrics.render_prometheus();
        assert!(text.contains("sl_http_requests_total 1"));
        assert!(text.contains("sl_http_errors_total 1"));
        assert!(text.contains("sl_http_request_duration_seconds_sum 0.250000"));
        assert!(text.contains("sl_http_request_duration_seconds_count 1"));
    }
}
