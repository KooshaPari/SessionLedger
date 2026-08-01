//! Native local session-store discovery for the daemon.
//!
//! The daemon should work out of the box on a developer machine.  These roots
//! mirror the viewer's automatic corpus resolver; an explicit `--watch` still
//! takes precedence for CI and custom stores.

use std::path::PathBuf;

/// Return existing native transcript roots in deterministic order.
pub fn local_watch_roots(home: Option<PathBuf>) -> Vec<PathBuf> {
    let home = home.or_else(|| std::env::var_os("HOME").map(PathBuf::from));
    let Some(home) = home else { return Vec::new() };
    [
        home.join(".codex").join("sessions"),
        home.join(".claude").join("projects"),
        home.join(".cursor").join("projects"),
        // Cursor has shipped both the global exported-project layout and the
        // live agent transcript layout.  They contain the same JSON/JSONL
        // transcript shapes, so both can flow through CursorDir without
        // browser access or credentials.
        home.join(".cursor").join("agent-transcripts"),
    ]
    .into_iter()
    .filter(|root| root.is_dir())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_only_existing_supported_roots() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".codex/sessions")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor/projects")).unwrap();
        assert_eq!(local_watch_roots(Some(dir.path().to_path_buf())).len(), 2);
    }

    #[test]
    fn discovers_cursor_agent_transcripts_alongside_project_exports() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor/projects")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor/agent-transcripts")).unwrap();

        assert_eq!(
            local_watch_roots(Some(dir.path().to_path_buf())),
            vec![dir.path().join(".cursor/projects"), dir.path().join(".cursor/agent-transcripts"),]
        );
    }

    #[test]
    fn absent_home_yields_empty_roots() {
        let dir = tempfile::tempdir().unwrap();
        assert!(local_watch_roots(Some(dir.path().to_path_buf())).is_empty());
    }
}
