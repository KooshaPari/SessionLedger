//! Hermetic `SelfCheck` for docs/ops/otlp-metrics.md anchors (C05 L43/L42).
//!
//! Local: `pwsh ./scripts/otlp-metrics-check.ps1 -SelfCheck`

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn otlp_metrics_doc_self_check_validates_anchors() {
    let script = repo_root().join("scripts/otlp-metrics-check.ps1");
    assert!(script.is_file(), "expected OTLP metrics check script at {}", script.display());

    let output = match Command::new("pwsh").arg("-NoProfile").arg("-Command").arg("exit 0").output() {
        Ok(_) => Command::new("pwsh")
        .args(["-NoProfile", "-File", script.to_str().expect("utf-8 script path"), "-SelfCheck"])
        .output()
        .expect("pwsh self-check failed to start"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => { eprintln!("skipping PowerShell self-check: pwsh is not installed"); return; },
        Err(error) => panic!("failed to probe pwsh for SelfCheck: {error}"),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "otlp-metrics-check.ps1 -SelfCheck failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("OTLP metrics SelfCheck passed"),
        "expected SelfCheck success line, got:\n{stdout}"
    );
}
