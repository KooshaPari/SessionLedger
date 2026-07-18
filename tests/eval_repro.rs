//! Hermetic `SelfCheck` for eval reproducibility manifest anchors (C08 L79).
//!
//! Local: `pwsh ./scripts/eval-repro-check.ps1 -SelfCheck`

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn eval_repro_manifest_self_check_validates_lockfile_and_fixtures() {
    let script = repo_root().join("scripts/eval-repro-check.ps1");
    assert!(
        script.is_file(),
        "expected eval repro check script at {}",
        script.display()
    );

    let output = Command::new("pwsh")
        .args(["-NoProfile", "-File", script.to_str().expect("utf-8 script path"), "-SelfCheck"])
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn pwsh for SelfCheck: {error}"));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "eval-repro-check.ps1 -SelfCheck failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("Eval reproducibility manifest SelfCheck passed"),
        "expected SelfCheck success line, got:\n{stdout}"
    );
    assert!(
        stdout.contains("cargo_lock_sha256="),
        "expected cargo_lock_sha256 echo, got:\n{stdout}"
    );
}
