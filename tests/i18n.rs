//! Hermetic `SelfCheck` for C01 L16 i18n scaffold (en+es catalogs + helper + docs).
//!
//! Local: `pwsh ./scripts/i18n-check.ps1 -SelfCheck`
//!
//! Proves the English + Spanish soft JSON catalogs, `src/i18n.rs` lookup API
//! (`SL_LOCALE` / `t_locale`), and Phase-0 / Fluent-ICU future-hooks doc anchors.
//! Does not claim full Fluent localization.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn i18n_scaffold_self_check_validates_anchors() {
    let script = repo_root().join("scripts/i18n-check.ps1");
    assert!(script.is_file(), "expected i18n check script at {}", script.display());

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
        "i18n-check.ps1 -SelfCheck failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("i18n SelfCheck passed"),
        "expected SelfCheck success line, got:\n{stdout}"
    );
}
