//! Contract checks for the enforced pipeline perf-budget gate (WBS-6.2 / C07).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn perf_baseline_exposes_enforced_budgets() {
    let path = repo_root().join("docs/ops/perf-baseline.json");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let value: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));

    assert_eq!(
        value.get("schema_version").and_then(serde_json::Value::as_u64),
        Some(2),
        "perf-baseline schema_version should be 2"
    );
    assert_eq!(
        value.pointer("/policy/enforced").and_then(serde_json::Value::as_bool),
        Some(true),
        "policy.enforced must be true for the blocking CI gate"
    );

    let threshold = value
        .pointer("/policy/threshold_percent")
        .and_then(serde_json::Value::as_f64)
        .expect("policy.threshold_percent");
    assert!(
        (threshold - 25.0).abs() < f64::EPSILON,
        "documented threshold is 25% (got {threshold})"
    );

    let benchmarks =
        value.get("benchmarks").and_then(serde_json::Value::as_object).expect("benchmarks object");
    let required = [
        "pipeline/distill_compile_200_messages",
        "pipeline/okf_export_200_messages",
        "pipeline/inject_render_200_messages",
    ];
    for key in required {
        let entry = benchmarks.get(key).unwrap_or_else(|| panic!("missing benchmark {key}"));
        let mean = entry
            .get("mean_ns")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or_else(|| panic!("missing {key}.mean_ns"));
        let budget = entry
            .get("budget_mean_ns")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or_else(|| panic!("missing {key}.budget_mean_ns"));
        assert!(mean > 0.0, "{key}.mean_ns must be positive");
        assert!(
            budget + f64::EPSILON >= mean * (1.0 + threshold / 100.0),
            "{key}.budget_mean_ns ({budget}) must cover mean+threshold ({})",
            mean * (1.0 + threshold / 100.0)
        );
    }
}

#[test]
fn bench_gate_self_check_validates_enforced_policy() {
    let script = repo_root().join("scripts/bench-gate.ps1");
    assert!(script.is_file(), "missing {}", script.display());

    let output = Command::new("pwsh")
        .args(["-NoProfile", "-File", script.to_str().expect("utf-8 script path"), "-SelfCheck"])
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn pwsh for SelfCheck: {error}"));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "bench-gate.ps1 -SelfCheck failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("SelfCheck passed"), "expected SelfCheck success line, got:\n{stdout}");
    assert!(stdout.contains("enforced=true"), "expected enforced=true echo, got:\n{stdout}");
}
