//! Soft stub for OTLP metrics export (C05 L43 / L42).
//!
//! Default builds keep dependency-free Prometheus text at `GET /metrics`.
//! Enable `--features otel-metrics` to compile this acknowledgment stub.
//! Set `SL_OTLP_METRICS=1` to log intent; no OTLP metrics push is performed yet.

/// Env gate that acknowledges the unpaid OTLP metrics push path.
pub const ENV_OTLP_METRICS: &str = "SL_OTLP_METRICS";

/// Returns whether the soft OTLP metrics stub is operator-acknowledged.
#[must_use]
pub fn stub_acknowledged() -> bool {
    std::env::var(ENV_OTLP_METRICS).is_ok_and(|value| {
        let trimmed = value.trim();
        trimmed == "1"
            || trimmed.eq_ignore_ascii_case("true")
            || trimmed.eq_ignore_ascii_case("yes")
    })
}

/// Emit a one-shot info line when the stub is acknowledged.
///
/// Does not open sockets, register instruments, or alter Prometheus `/metrics`.
pub fn maybe_log_stub_status() {
    if stub_acknowledged() {
        tracing::info!(
            target: "sl_daemon::otel_metrics",
            metrics_export = "stub",
            prometheus_http = "/metrics",
            "OTLP metrics export stub acknowledged (SL_OTLP_METRICS); \
             push unpaid — Prometheus /metrics unchanged"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_constant_matches_ops_ssot() {
        assert_eq!(ENV_OTLP_METRICS, "SL_OTLP_METRICS");
    }

    #[test]
    fn stub_acknowledged_parses_truthy_values() {
        // Isolate from ambient process env for hermetic unit proof.
        std::env::remove_var(ENV_OTLP_METRICS);
        assert!(!stub_acknowledged());

        std::env::set_var(ENV_OTLP_METRICS, "1");
        assert!(stub_acknowledged());

        std::env::set_var(ENV_OTLP_METRICS, "true");
        assert!(stub_acknowledged());

        std::env::set_var(ENV_OTLP_METRICS, "0");
        assert!(!stub_acknowledged());

        std::env::remove_var(ENV_OTLP_METRICS);
    }
}
