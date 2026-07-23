//! Corpus loader — bridges the Forge ingestion adapter to the viewer's data model.
//!
//! When the `sqlite` feature is enabled and a DB path is provided, loads real
//! sessions from a Forge SQLite corpus via [`ForgeDb`].  Falls back to
//! [`mock_data::sample_sessions`] when no path is given (development / demo mode).
//!
//! The data-layer is intentionally decoupled from Dioxus so it can be unit-tested
//! without a UI runtime.

use session_ledger::domain::session::Session;

use crate::mock_data::sample_sessions;

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
    if discovered_roots == 0 {
        return Err(
            "no supported local session stores found (Codex, Claude Code, or Cursor)".into()
        );
    }
    sessions.sort_by_key(|session| {
        session.messages.iter().filter_map(|message| message.ts_ms).max().unwrap_or_default()
    });
    sessions.reverse();
    Ok(sessions)
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
    if !root.is_dir() {
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
        let root = std::env::var_os("HOME").map(std::path::PathBuf::from).unwrap_or_default();
        if !root.join(".codex/sessions").exists()
            && !root.join(".claude/projects").exists()
            && !root.join(".cursor/projects").exists()
        {
            assert!(load_sessions(&DataSource::Auto).is_err());
        }
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

    // ── SQLite source ─────────────────────────────────────────────────────────

    #[cfg(feature = "sqlite")]
    mod sqlite_tests {
        use std::path::Path;

        use rusqlite::Connection;

        use super::*;

        /// Build a minimal Forge SQLite fixture DB at `path` with `rows` rows.
        fn write_fixture_db(
            path: &Path,
            rows: &[(&str, Option<&str>, Option<&str>, Option<Vec<u8>>, Option<&str>)],
        ) {
            let conn = Connection::open(path).expect("create fixture db");
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS conversations (
                    id            TEXT PRIMARY KEY,
                    title         TEXT,
                    cwd           TEXT,
                    context_zstd  BLOB,
                    context       TEXT
                 );",
            )
            .expect("create table");
            for (id, title, cwd, blob, plain) in rows {
                conn.execute(
                    "INSERT OR IGNORE INTO conversations \
                     (id, title, cwd, context_zstd, context) VALUES (?1,?2,?3,?4,?5)",
                    rusqlite::params![id, title, cwd, blob, plain],
                )
                .expect("insert row");
            }
        }

        fn zstd_compress(s: &str) -> Vec<u8> {
            zstd::stream::encode_all(s.as_bytes(), 3).expect("compress")
        }

        #[test]
        fn sqlite_source_loads_real_sessions() {
            let tmp = tempfile::NamedTempFile::new().expect("tempfile");
            let ctx = serde_json::json!([
                {"role": "user", "content": "implement the thing"},
                {"role": "assistant", "content": "done"}
            ])
            .to_string();
            let blob = zstd_compress(&ctx);

            write_fixture_db(
                tmp.path(),
                &[
                    (
                        "forge-real-001",
                        Some("Real session"),
                        Some("/code/project"),
                        Some(blob.clone()),
                        None,
                    ),
                    (
                        "forge-real-002",
                        Some("Another real session"),
                        Some("/code/other"),
                        Some(blob),
                        None,
                    ),
                ],
            );

            let source = DataSource::ForgeDb(tmp.path().to_owned());
            let sessions = load_sessions(&source).expect("sqlite load");

            assert_eq!(sessions.len(), 2);
            assert_eq!(sessions[0].id, "forge-real-001");
            assert_eq!(sessions[0].title.as_deref(), Some("Real session"));
            assert_eq!(sessions[0].cwd.as_deref(), Some("/code/project"));
            assert_eq!(sessions[0].messages.len(), 2);
        }

        #[test]
        fn sqlite_source_skips_corrupt_rows_and_returns_clean_rows() {
            let tmp = tempfile::NamedTempFile::new().expect("tempfile");
            let ctx = serde_json::json!([
                {"role": "user", "content": "hello"}
            ])
            .to_string();
            let good_blob = zstd_compress(&ctx);
            let bad_blob = vec![0xDE, 0xAD, 0xBE, 0xEF];

            write_fixture_db(
                tmp.path(),
                &[
                    ("corrupt-row", None, None, Some(bad_blob), None),
                    ("clean-row", Some("ok"), Some("/ok"), Some(good_blob), None),
                ],
            );

            let source = DataSource::ForgeDb(tmp.path().to_owned());
            let sessions = load_sessions(&source).expect("sqlite load");

            // Only the clean row should appear.
            assert_eq!(sessions.len(), 1);
            assert_eq!(sessions[0].id, "clean-row");
        }

        #[test]
        fn sqlite_source_plain_text_fallback() {
            let tmp = tempfile::NamedTempFile::new().expect("tempfile");
            let ctx = serde_json::json!([
                {"role": "user", "content": "plain text fallback"}
            ])
            .to_string();

            write_fixture_db(
                tmp.path(),
                &[("plain-row", Some("Plain"), Some("/plain"), None, Some(ctx.as_str()))],
            );

            let source = DataSource::ForgeDb(tmp.path().to_owned());
            let sessions = load_sessions(&source).expect("sqlite load");

            assert_eq!(sessions.len(), 1);
            assert_eq!(sessions[0].messages[0].content, "plain text fallback");
        }

        #[test]
        fn sqlite_source_error_on_nonexistent_file() {
            let source = DataSource::ForgeDb(std::path::PathBuf::from("/tmp/does_not_exist_sl.db"));
            let result = load_sessions(&source);
            assert!(result.is_err(), "should fail on missing file");
            let err = result.unwrap_err();
            assert!(err.contains("cannot open"), "error should describe open failure, got: {err}");
        }

        // ── Committed fixture DB integration test ────────────────────────────

        /// Build (or re-use if already present) the committed fixture DB at
        /// `tests/fixtures/forge_fixture.db`, then load it through the real
        /// corpus loader path and assert the expected session count.
        #[test]
        fn committed_fixture_db_loads_expected_sessions() {
            let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("fixtures")
                .join("forge_fixture.db");

            // Re-create the fixture each run so the test is hermetic.
            if fixture_path.exists() {
                std::fs::remove_file(&fixture_path).expect("remove old fixture");
            }
            std::fs::create_dir_all(fixture_path.parent().expect("parent")).expect("mkdir");

            let ctx_a = serde_json::json!([
                {"role": "user", "content": "fix the login timeout"},
                {"role": "assistant", "content": "bumped TTL to 1800s"},
                {"role": "user", "content": "looks good, ship it"}
            ])
            .to_string();
            let ctx_b = serde_json::json!([
                {"role": "user", "content": "add billing to the API"},
                {"role": "assistant", "content": "stripe integration done"},
                {"role": "user", "content": "approved"}
            ])
            .to_string();

            let blob_a = zstd_compress(&ctx_a);
            let blob_b = zstd_compress(&ctx_b);

            write_fixture_db(
                &fixture_path,
                &[
                    (
                        "fixture-session-001",
                        Some("Login timeout fix"),
                        Some("/code/auth-service"),
                        Some(blob_a),
                        None,
                    ),
                    (
                        "fixture-session-002",
                        Some("API billing"),
                        Some("/code/api-gateway"),
                        Some(blob_b),
                        None,
                    ),
                ],
            );

            let source = DataSource::ForgeDb(fixture_path.clone());
            let sessions = load_sessions(&source).expect("load committed fixture");

            assert_eq!(sessions.len(), 2, "fixture must contain exactly 2 sessions");

            let s1 = sessions.iter().find(|s| s.id == "fixture-session-001").expect("s1");
            assert_eq!(s1.title.as_deref(), Some("Login timeout fix"));
            assert_eq!(s1.messages.len(), 3);
            assert_eq!(s1.messages[0].role, session_ledger::domain::session::Role::User);

            let s2 = sessions.iter().find(|s| s.id == "fixture-session-002").expect("s2");
            assert_eq!(s2.title.as_deref(), Some("API billing"));
            assert_eq!(s2.messages.len(), 3);
        }
    }
}
