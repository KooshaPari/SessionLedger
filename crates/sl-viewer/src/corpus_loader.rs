//! Corpus loader — bridges the Forge ingestion adapter to the viewer's data model.
//!
//! When the `sqlite` feature is enabled and a DB path is provided, loads real
//! sessions from a Forge SQLite corpus via [`ForgeDb`].  Falls back to
//! [`mock_data::sample_sessions`] when no path is given (development / demo mode).
//!
//! The data-layer is intentionally decoupled from Dioxus so it can be unit-tested
//! without a UI runtime.

use crate::mock_data::sample_sessions;
use session_ledger::domain::session::Session;
use std::collections::HashSet;

mod web_exports;

#[cfg(test)]
use web_exports::WebExportProvider;
use web_exports::{load_web_export_corpus, web_export_roots_with_env};

/// Source configuration for the viewer's session list.
#[derive(Debug, Clone, Default)]
pub enum DataSource {
    /// Discover native session stores on the local device.
    #[default]
    Auto,
    /// In-memory mock data (explicit demo mode only).
    Mock,
    /// Load from a Forge SQLite database at the given path.
    #[cfg(feature = "sqlite")]
    ForgeDb(std::path::PathBuf),
}

/// Load sessions from the configured source.
///
/// On `Mock`: returns the hard-coded sample sessions.
/// On `ForgeDb`: opens the DB read-only, ingests all conversations, returns
/// the successfully-parsed sessions.  Rows that fail decompression or JSON
/// parsing are skipped and logged to stderr rather than aborting.
///
/// # Errors
///
/// Returns an error string only if the database itself cannot be opened or
/// queried (e.g. file not found, not a SQLite database).  Per-row failures are
/// surfaced on stderr as warnings and do not cause an error return.
pub fn load_sessions(source: &DataSource) -> Result<Vec<Session>, String> {
    match source {
        DataSource::Mock => Ok(sample_sessions()),
        DataSource::Auto => load_discovered_sessions(),
        #[cfg(feature = "sqlite")]
        DataSource::ForgeDb(path) => load_from_sqlite(path),
    }
}

fn load_discovered_sessions() -> Result<Vec<Session>, String> {
    let home = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| "HOME is not set; cannot discover local sessions".to_owned())?;
    load_discovered_sessions_with_home(&home, std::env::var_os("SESSIONLEDGER_WEB_EXPORT_ROOTS"))
}

fn load_discovered_sessions_with_home(
    home: &std::path::Path,
    explicit_web_roots: Option<std::ffi::OsString>,
) -> Result<Vec<Session>, String> {
    let mut sessions = Vec::new();
    let mut discovered_roots = 0;
    discovered_roots += load_json_corpus(
        &home.join(".codex").join("sessions"),
        |path| session_ledger::CodexDir::new(path.to_path_buf()),
        &mut sessions,
    )?;
    discovered_roots += load_json_corpus(
        &home.join(".claude").join("projects"),
        |path| session_ledger::ClaudeDir::new(path.to_path_buf()),
        &mut sessions,
    )?;
    // Cursor stores exported conversation JSON/JSONL under its global data
    // directory on macOS. Only existing roots are scanned; caches and plans
    // that do not contain transcript-shaped files are ignored by the adapter.
    discovered_roots += load_json_corpus(
        &home.join(".cursor").join("projects"),
        |path| session_ledger::CursorDir::new(path.to_path_buf()),
        &mut sessions,
    )?;
    for (provider, root) in web_export_roots_with_env(home, explicit_web_roots) {
        discovered_roots += load_web_export_corpus(&root, provider, &mut sessions)?;
    }
    #[cfg(feature = "sqlite")]
    if let Some(path) = resolve_forge_db_path(home, std::env::var_os("FORGE_DB")) {
        sessions.extend(load_from_sqlite(&path)?);
        discovered_roots += 1;
    }
    if discovered_roots == 0 {
        return Err(
            "no supported local session stores found (Codex, Claude, Cursor, web exports, or Forge)"
                .into(),
        );
    }
    let mut seen = HashSet::new();
    sessions.retain(|session| seen.insert((session.corpus, session.id.clone())));
    sessions.sort_by_key(|session| {
        session.messages.iter().filter_map(|message| message.ts_ms).max().unwrap_or_default()
    });
    sessions.reverse();
    Ok(sessions)
}

/// Resolve the Forge database used by automatic discovery.
///
/// An explicit `FORGE_DB` value always wins, including when it points at a
/// missing file (the subsequent open produces a useful error).  Without the
/// override, the native Forge location is used when present.
#[cfg(feature = "sqlite")]
fn resolve_forge_db_path(
    home: &std::path::Path,
    explicit: Option<std::ffi::OsString>,
) -> Option<std::path::PathBuf> {
    if let Some(path) = explicit {
        let path = std::path::PathBuf::from(path);
        return (!path.as_os_str().is_empty()).then_some(path);
    }
    let native = home.join(".forge").join(".forge.db");
    native.is_file().then_some(native)
}

fn load_json_corpus<F, S>(
    root: &std::path::Path,
    make_source: F,
    sessions: &mut Vec<Session>,
) -> Result<usize, String>
where
    F: FnOnce(&std::path::Path) -> S,
    S: session_ledger::ports::CorpusSource,
{
    if !root.is_dir() && !root.is_file() {
        return Ok(0);
    }
    let source = make_source(root);
    let ids = source.list().map_err(|e| format!("discover {}: {e}", root.display()))?;
    for id in ids {
        match source.load(&id) {
            Ok(session) if !session.messages.is_empty() => sessions.push(session),
            Ok(_) => {}
            Err(error) => eprintln!("[sl-viewer] skipping {}:{}: {error}", root.display(), id),
        }
    }
    Ok(1)
}

/// Open a Forge SQLite DB at `path` and ingest all conversations.
#[cfg(feature = "sqlite")]
fn load_from_sqlite(path: &std::path::Path) -> Result<Vec<Session>, String> {
    use session_ledger::ingestion::forge::ForgeDb;

    let db = ForgeDb::open(path)
        .map_err(|e| format!("cannot open forge DB at {}: {e}", path.display()))?;

    let (sessions, report) =
        db.ingest_all().map_err(|e| format!("forge ingest_all failed: {e}"))?;

    if !report.is_clean() {
        eprintln!("[sl-viewer] forge ingestion: {} skipped rows:", report.skipped.len());
        for (id, reason) in &report.skipped {
            eprintln!("  skip {id}: {reason}");
        }
    }

    eprintln!(
        "[sl-viewer] forge ingestion: {} sessions loaded from {}",
        sessions.len(),
        path.display()
    );

    Ok(sessions)
}

#[cfg(all(test, feature = "sqlite"))]
#[path = "corpus_loader/sqlite_tests.rs"]
mod sqlite_tests;

#[cfg(test)]
mod tests {
    use super::*;

    // ── Mock source ───────────────────────────────────────────────────────────

    #[test]
    fn mock_source_returns_non_empty_sessions() {
        let sessions = load_sessions(&DataSource::Mock).expect("mock load");
        assert!(!sessions.is_empty(), "mock data must contain at least one session");
    }

    #[test]
    fn auto_source_missing_store_is_an_explicit_error() {
        let home = tempfile::tempdir().expect("temp home");
        assert!(load_discovered_sessions_with_home(home.path(), None).is_err());
    }

    #[test]
    fn web_export_roots_include_expected_defaults() {
        let home = tempfile::tempdir().expect("temp home");
        for base in ["Downloads", "Documents", "Desktop"] {
            let base = home.path().join(base);
            std::fs::create_dir_all(base.join("ChatGPT")).expect("chatgpt dir");
            std::fs::create_dir_all(base.join("Claude")).expect("claude dir");
            std::fs::create_dir_all(base.join("Gemini")).expect("gemini dir");
        }
        std::fs::create_dir_all(home.path().join(".sessionledger").join("imports").join("ChatGPT"))
            .expect("sessionledger import provider dir");
        std::fs::create_dir_all(
            home.path()
                .join("Library")
                .join("Application Support")
                .join("SessionLedger")
                .join("imports"),
        )
        .expect("imports");

        let roots = web_export_roots_with_env(home.path(), None);
        assert!(roots.contains(&(
            WebExportProvider::ChatGpt,
            home.path().join("Downloads").join("ChatGPT")
        )));
        assert!(roots
            .contains(&(WebExportProvider::Claude, home.path().join("Downloads").join("Claude"))));
        assert!(roots
            .contains(&(WebExportProvider::Gemini, home.path().join("Downloads").join("Gemini"))));
        assert!(roots.contains(&(
            WebExportProvider::ChatGpt,
            home.path().join(".sessionledger").join("imports").join("ChatGPT")
        )));
    }

    #[test]
    fn web_export_roots_prefers_explicit_override() {
        let home = tempfile::tempdir().expect("temp home");
        let chat_root = home.path().join("chatgpt_export");
        let claude_root = home.path().join("claude_export");
        std::fs::create_dir_all(&chat_root).expect("chat export root");
        std::fs::create_dir_all(&claude_root).expect("claude export root");
        let explicit = std::env::join_paths([&chat_root, &claude_root]).unwrap();
        let roots = web_export_roots_with_env(home.path(), Some(explicit));
        assert_eq!(
            roots,
            vec![(WebExportProvider::ChatGpt, chat_root), (WebExportProvider::Claude, claude_root)]
        );
    }

    #[test]
    fn invalid_explicit_web_root_does_not_fall_back_to_defaults() {
        let home = tempfile::tempdir().expect("temp home");
        std::fs::create_dir_all(home.path().join("Downloads/ChatGPT"))
            .expect("default provider root");
        let ambiguous = tempfile::tempdir().expect("ambiguous explicit root");
        let explicit = std::env::join_paths([ambiguous.path()]).expect("encode explicit root");

        assert!(web_export_roots_with_env(home.path(), Some(explicit)).is_empty());
    }

    #[test]
    fn explicit_provider_export_file_is_discovered_and_loaded() {
        let home = tempfile::tempdir().expect("temp home");
        let export = home.path().join("chatgpt-export.json");
        std::fs::write(
            &export,
            serde_json::json!({
                "conversation_id": "web-file-1",
                "mapping": {
                    "message": {
                        "author": {"role": "user"},
                        "content": {"parts": ["hello"]}
                    }
                }
            })
            .to_string(),
        )
        .expect("write explicit export file");
        let explicit = std::env::join_paths([&export]).expect("encode export file");

        let roots = web_export_roots_with_env(home.path(), Some(explicit));
        let mut sessions = Vec::new();
        let discovered =
            load_web_export_corpus(&roots[0].1, roots[0].0, &mut sessions).expect("load export");

        assert_eq!(roots, vec![(WebExportProvider::ChatGpt, export)]);
        assert_eq!(discovered, 1);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "web-file-1");
        assert_eq!(sessions[0].corpus, session_ledger::Corpus::ChatGptWeb);
    }

    #[test]
    fn mock_sessions_have_valid_ids() {
        let sessions = load_sessions(&DataSource::Mock).expect("mock load");
        for s in &sessions {
            assert!(!s.id.is_empty(), "session id must be non-empty");
        }
    }

    #[test]
    fn mock_sessions_have_messages() {
        let sessions = load_sessions(&DataSource::Mock).expect("mock load");
        for s in &sessions {
            assert!(!s.messages.is_empty(), "mock session {} has no messages", s.id);
        }
    }

    #[test]
    fn auto_loader_accepts_claude_projects_root() {
        let root = tempfile::tempdir().expect("temp root");
        let project = root.path().join("-Users-demo-repo");
        std::fs::create_dir_all(&project).expect("project root");
        std::fs::write(
            project.join("session.jsonl"),
            serde_json::json!({
                "type": "user",
                "sessionId": "claude-local-1",
                "message": {"role": "user", "content": "hello"}
            })
            .to_string(),
        )
        .expect("write transcript");

        let mut sessions = Vec::new();
        let roots = load_json_corpus(
            root.path(),
            |path| session_ledger::ClaudeDir::new(path.to_path_buf()),
            &mut sessions,
        )
        .expect("discover Claude transcripts");

        assert_eq!(roots, 1);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "claude-local-1");
    }

    #[test]
    fn load_web_export_corpus_loads_chatgpt_claude_and_gemini_exports() {
        let root = tempfile::tempdir().expect("temp root");
        std::fs::write(
            root.path().join("export.json"),
            serde_json::json!({
                "conversation_id": "web-1",
                "title": "Export",
                "mapping": {
                    "a": {
                        "message": {
                            "author": {"role": "user"},
                            "content": {"parts": ["hello"]}
                        }
                    },
                    "b": {
                        "message": {
                            "author": {"role": "assistant"},
                            "content": {"parts": ["hi"]}
                        }
                    }
                }
            })
            .to_string(),
        )
        .expect("write web transcript");

        let mut sessions = Vec::new();
        let discovered =
            load_web_export_corpus(root.path(), WebExportProvider::ChatGpt, &mut sessions)
                .expect("load chatgpt exports")
                + load_web_export_corpus(root.path(), WebExportProvider::Claude, &mut sessions)
                    .expect("load claude exports")
                + load_web_export_corpus(root.path(), WebExportProvider::Gemini, &mut sessions)
                    .expect("load gemini exports");

        assert_eq!(discovered, 3);
        assert_eq!(sessions.len(), 3);
        assert!(sessions
            .iter()
            .any(|s| s.corpus == session_ledger::domain::session::Corpus::ChatGptWeb));
        assert!(sessions
            .iter()
            .any(|s| s.corpus == session_ledger::domain::session::Corpus::ClaudeWeb));
        assert!(sessions
            .iter()
            .any(|s| s.corpus == session_ledger::domain::session::Corpus::GeminiWeb));
    }

    #[test]
    fn automatic_web_discovery_does_not_duplicate_a_provider_export() {
        let home = tempfile::tempdir().expect("temp home");
        let root = home.path().join("Downloads/ChatGPT");
        std::fs::create_dir_all(&root).expect("ChatGPT export root");
        std::fs::write(
            root.join("conversation.json"),
            serde_json::json!({
                "conversation_id": "chatgpt-only-1",
                "mapping": {
                    "message": {
                        "author": {"role": "user"},
                        "content": {"parts": ["hello"]}
                    }
                }
            })
            .to_string(),
        )
        .expect("write export");

        let sessions =
            load_discovered_sessions_with_home(home.path(), None).expect("discover web export");

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "chatgpt-only-1");
        assert_eq!(sessions[0].corpus, session_ledger::Corpus::ChatGptWeb);
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn forge_auto_resolution_prefers_explicit_override() {
        let home = tempfile::tempdir().expect("home");
        let native = home.path().join(".forge/.forge.db");
        std::fs::create_dir_all(native.parent().expect("parent")).expect("mkdir");
        std::fs::write(&native, b"fixture").expect("native db");

        let explicit = std::ffi::OsString::from("/custom/forge.db");
        assert_eq!(
            resolve_forge_db_path(home.path(), Some(explicit)),
            Some(std::path::PathBuf::from("/custom/forge.db"))
        );
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn forge_auto_resolution_discovers_native_database() {
        let home = tempfile::tempdir().expect("home");
        let native = home.path().join(".forge/.forge.db");
        std::fs::create_dir_all(native.parent().expect("parent")).expect("mkdir");
        std::fs::write(&native, b"fixture").expect("native db");

        assert_eq!(resolve_forge_db_path(home.path(), None), Some(native));
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn forge_auto_resolution_ignores_absent_native_database() {
        let home = tempfile::tempdir().expect("home");
        assert_eq!(resolve_forge_db_path(home.path(), None), None);
    }
}
