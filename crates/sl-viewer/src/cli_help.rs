//! CLI `--help` / `--version` text for `sl-viewer` (C01/C09 DX).
//!
//! SSOT companion: `docs/ops/sl-viewer-help.md`.

use crate::daemon_url::{daemon_base_url, DEFAULT_DAEMON_BASE};

pub const HELP_HEADING: &str = "SessionLedger desktop viewer (sl-viewer)";

/// Multi-line `--version` output (package version + resolved daemon URL).
pub fn version_text() -> String {
    format!(
        "sl-viewer {} (SessionLedger)\ndaemon: {}\nhelp: docs/ops/sl-viewer-help.md",
        env!("CARGO_PKG_VERSION"),
        daemon_base_url()
    )
}

/// Multi-line `--help` output (usage, env vars, doc cross-links).
pub fn help_text() -> String {
    format!(
        r#"{HELP_HEADING}

USAGE:
    sl-viewer [--help | -h] [--version | -V]

ENVIRONMENT:
    SL_DAEMON_URL   Runtime daemon base URL override (default: {DEFAULT_DAEMON_BASE}).
                    Set one URL to force endpoint resolution.
    SL_DAEMON_URLS  Runtime URL set (comma/semicolon/space-separated), tried in order.
                    Use for port failover, eg: http://127.0.0.1:9001,http://127.0.0.1:8080.
    FORGE_DB        Runtime path to a Forge SQLite corpus (requires --features sqlite).
    SL_VIEWER_DEMO  Set to 1 for in-memory demo data (desktop only).

IN-VIEWER:
    ?               Toggle keyboard help overlay
    Cmd/Ctrl+K      Command palette

DOCS:
    docs/ops/sl-viewer-help.md              CLI + env SSOT (C09)
    docs/HELP.md                            In-viewer shortcuts
    docs/guides/quick-start/QUICKSTART.md   First-run stack setup
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_documents_sl_daemon_url_and_forge_db() {
        let help = help_text();
        assert!(help.contains("SL_DAEMON_URL"), "missing SL_DAEMON_URL in help");
        assert!(help.contains("FORGE_DB"), "missing FORGE_DB in help");
        assert!(help.contains("sl-viewer-help.md"), "missing sl-viewer-help doc link");
        assert!(help.contains("QUICKSTART.md"), "missing quick-start doc link");
    }

    #[test]
    fn version_includes_package_version_and_daemon() {
        let version = version_text();
        assert!(version.contains(env!("CARGO_PKG_VERSION")));
        assert!(version.contains("daemon:"));
        assert!(version.contains("sl-viewer-help.md"));
    }
}
