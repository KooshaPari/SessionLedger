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

/// Default number of full session payloads retained by the viewer at startup.
/// Discovery remains newest-first; the UI reports when older sessions are
/// outside the bounded in-memory view instead of silently dropping them.
pub const DEFAULT_MAX_DISCOVERED_SESSIONS: usize = 128;

/// Upper bound for the opt-in `SESSION_LEDGER_VIEWER_MAX_SESSIONS` override.
///
/// Keeping an absolute ceiling protects the desktop app from accidentally
/// loading an unbounded transcript corpus when a shell profile exports a bad
/// value. Larger histories remain available through the daemon export.
pub const MAX_CONFIGURED_DISCOVERED_SESSIONS: usize = 256;

const MAX_DISCOVERED_SESSIONS_ENV: &str = "SESSION_LEDGER_VIEWER_MAX_SESSIONS";

/// Result of loading a viewer corpus, including bounded-retention accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionLoadReport {
    pub sessions: Vec<Session>,
    /// Number of transcript files enumerated by native discovery.  Native
    /// discovery intentionally counts the filesystem index, not only the
    /// bounded payloads parsed below; older files remain on disk and are not
    /// decompressed during startup.
    pub discovered_count: usize,
    pub retained_count: usize,
}

impl SessionLoadReport {
    #[must_use]
    pub fn is_truncated(&self) -> bool {
        self.retained_count < self.discovered_count
    }
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
    Ok(load_sessions_report(source)?.sessions)
}

/// Load sessions and retain a bounded, newest-first in-memory view.
pub fn load_sessions_report(source: &DataSource) -> Result<SessionLoadReport, String> {
    let max_sessions = configured_max_discovered_sessions();
    match source {
        DataSource::Mock => report(sample_sessions(), max_sessions),
        DataSource::Auto => load_discovered_sessions(max_sessions),
        #[cfg(feature = "sqlite")]
        DataSource::ForgeDb(path) => {
            load_from_sqlite(path).map(|sessions| report(sessions, max_sessions))
        }
    }
}

fn configured_max_discovered_sessions() -> usize {
    parse_max_discovered_sessions(std::env::var(MAX_DISCOVERED_SESSIONS_ENV).ok().as_deref())
}

fn parse_max_discovered_sessions(raw: Option<&str>) -> usize {
    raw.and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .map(|value| value.min(MAX_CONFIGURED_DISCOVERED_SESSIONS))
        .unwrap_or(DEFAULT_MAX_DISCOVERED_SESSIONS)
}

fn report(mut sessions: Vec<Session>, max_sessions: usize) -> Result<SessionLoadReport, String> {
    let discovered_count = sessions.len();
    let retained_count = discovered_count.min(max_sessions);
    sessions.truncate(retained_count);
    Ok(SessionLoadReport { sessions, discovered_count, retained_count })
}

fn load_discovered_sessions(max_sessions: usize) -> Result<SessionLoadReport, String> {
    let home = std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| "HOME is not set; cannot discover local sessions".to_owned())?;
    let mut sessions = Vec::new();
    let mut discovered_count = 0;
    let mut discovered_roots = 0;
    discovered_roots += load_json_corpus(
        &home.join(".codex").join("sessions"),
        |path| session_ledger::CodexDir::new(path.to_path_buf()),
        &mut sessions,
        &mut discovered_count,
        max_sessions,
    )?;
    discovered_roots += load_json_corpus(
        &home.join(".claude").join("projects"),
        |path| session_ledger::ClaudeDir::new(path.to_path_buf()),
        &mut sessions,
        &mut discovered_count,
        max_sessions,
    )?;
    // Cursor stores exported conversation JSON/JSONL under its global data
    // directory on macOS. Only existing roots are scanned; caches and plans
    // that do not contain transcript-shaped files are ignored by the adapter.
    discovered_roots += load_json_corpus(
        &home.join(".cursor").join("projects"),
        |path| session_ledger::CursorDir::new(path.to_path_buf()),
        &mut sessions,
        &mut discovered_count,
        max_sessions,
    )?;
    // Cursor's native agent runner keeps transcripts outside the projects
    // tree. Discover this sibling root so automatic viewer discovery matches
    // the daemon's device-level Cursor coverage without traversing caches.
    discovered_roots += load_json_corpus(
        &home.join(".cursor").join("agent-transcripts"),
        |path| session_ledger::CursorDir::new(path.to_path_buf()),
        &mut sessions,
        &mut discovered_count,
        max_sessions,
    )?;
    #[cfg(feature = "sqlite")]
    if let Some(path) = resolve_forge_db_path(&home, std::env::var_os("FORGE_DB")) {
        for session in load_from_sqlite(&path)? {
            if !session.messages.is_empty() {
                discovered_count += 1;
                retain_session(&mut sessions, session, max_sessions);
            }
        }
        discovered_roots += 1;
    }
    if discovered_roots == 0 {
        return Err(
            "no supported local session stores found (Codex, Claude Code, Cursor, or Forge)".into(),
        );
    }
    sessions.sort_by_key(session_timestamp);
    sessions.reverse();
    Ok(SessionLoadReport { retained_count: sessions.len(), sessions, discovered_count })
}

fn session_timestamp(session: &Session) -> i64 {
    session.messages.iter().filter_map(|message| message.ts_ms).max().unwrap_or_default()
}

fn retain_session(sessions: &mut Vec<Session>, session: Session, max_sessions: usize) {
    sessions.push(session);
    if sessions.len() > max_sessions {
        if let Some((oldest, _)) =
            sessions.iter().enumerate().min_by_key(|(_, item)| session_timestamp(item))
        {
            sessions.swap_remove(oldest);
        }
    }
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
    discovered_count: &mut usize,
    max_sessions: usize,
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
    let listed_count = ids.len();
    // `list` is an index-only operation.  Rank the IDs from filesystem
    // metadata and load only the bounded newest window; in particular, this
    // prevents historical `.jsonl.zst` transcripts from being decompressed
    // just to discover that they will be evicted from the viewer anyway.
    for id in recent_ids(root, ids, max_sessions) {
        match source.load(&id) {
            Ok(session) if !session.messages.is_empty() => {
                retain_session(sessions, session, max_sessions);
            }
            Ok(_) => {}
            Err(error) => eprintln!("[sl-viewer] skipping {}:{}: {error}", root.display(), id),
        }
    }
    // This is deliberately the number of indexed transcript records, not the
    // number successfully parsed in the bounded window.  A malformed or empty
    // historical record is therefore still visible in the truncation notice
    // without claiming that it was loaded successfully.
    *discovered_count += listed_count;
    Ok(1)
}

/// Return the newest transcript IDs without opening or decompressing them.
///
/// Filesystem modification time is the best available recency signal for
/// adapters whose IDs are UUIDs (Claude/Cursor).  The ID is a deterministic
/// tie-breaker and also preserves Codex's date-prefixed path ordering when
/// archives share a timestamp.
fn recent_ids(root: &std::path::Path, ids: Vec<String>, limit: usize) -> Vec<String> {
    let mut ranked = ids
        .into_iter()
        .map(|id| {
            let modified_ns = root
                .join(&id)
                .metadata()
                .and_then(|metadata| metadata.modified())
                .ok()
                .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |duration| duration.as_nanos());
            (modified_ns, id)
        })
        .collect::<Vec<_>>();
    ranked.sort_unstable_by(|(left_time, left_id), (right_time, right_id)| {
        left_time.cmp(right_time).then_with(|| left_id.cmp(right_id))
    });
    ranked.into_iter().rev().take(limit).map(|(_, id)| id).collect()
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
    fn configured_session_limit_defaults_to_safe_memory_budget() {
        assert_eq!(parse_max_discovered_sessions(None), 128);
    }

    #[test]
    fn configured_session_limit_rejects_invalid_values_and_clamps_unsafe_values() {
        assert_eq!(parse_max_discovered_sessions(Some("0")), 128);
        assert_eq!(parse_max_discovered_sessions(Some("not-a-number")), 128);
        assert_eq!(parse_max_discovered_sessions(Some(" 32 ")), 32);
        assert_eq!(parse_max_discovered_sessions(Some("999999")), 256);
    }

    #[test]
    fn session_report_caps_retained_payload_and_exposes_count() {
        let sessions = (0..(DEFAULT_MAX_DISCOVERED_SESSIONS + 8))
            .map(|index| {
                let mut session = Session::new(
                    format!("session-{index}"),
                    session_ledger::domain::session::Corpus::Codex,
                );
                session.messages.push(session_ledger::domain::session::Message::new(
                    session_ledger::domain::session::Role::User,
                    "payload",
                ));
                session
            })
            .collect();

        let report = report(sessions, DEFAULT_MAX_DISCOVERED_SESSIONS).expect("bounded report");
        assert_eq!(report.discovered_count, DEFAULT_MAX_DISCOVERED_SESSIONS + 8);
        assert_eq!(report.retained_count, DEFAULT_MAX_DISCOVERED_SESSIONS);
        assert!(report.is_truncated());
        assert_eq!(report.sessions.len(), DEFAULT_MAX_DISCOVERED_SESSIONS);
        assert_eq!(report.sessions[0].id, "session-0");
    }

    #[test]
    fn bounded_discovery_retains_newest_sessions_while_iterating() {
        let mut retained = Vec::new();
        for index in 0..(DEFAULT_MAX_DISCOVERED_SESSIONS + 8) {
            let mut session = Session::new(
                format!("session-{index}"),
                session_ledger::domain::session::Corpus::Codex,
            );
            let mut message = session_ledger::domain::session::Message::new(
                session_ledger::domain::session::Role::User,
                "payload",
            );
            message.ts_ms = Some(index as i64);
            session.messages.push(message);
            retain_session(&mut retained, session, DEFAULT_MAX_DISCOVERED_SESSIONS);
        }

        assert_eq!(retained.len(), DEFAULT_MAX_DISCOVERED_SESSIONS);
        assert!(retained.iter().all(|session| session_timestamp(session) >= 8));
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
        let mut discovered_count = 0;
        let roots = load_json_corpus(
            root.path(),
            |path| session_ledger::ClaudeDir::new(path.to_path_buf()),
            &mut sessions,
            &mut discovered_count,
            DEFAULT_MAX_DISCOVERED_SESSIONS,
        )
        .expect("discover Claude transcripts");

        assert_eq!(roots, 1);
        assert_eq!(discovered_count, 1);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "claude-local-1");
    }

    #[test]
    fn auto_loader_accepts_cursor_agent_transcripts_root() {
        let root = tempfile::tempdir().expect("temp root");
        std::fs::write(
            root.path().join("agent-session.jsonl"),
            serde_json::json!({
                "conversationId": "cursor-agent-1",
                "role": "user",
                "content": "hello from the Cursor agent"
            })
            .to_string(),
        )
        .expect("write transcript");

        let mut sessions = Vec::new();
        let mut discovered_count = 0;
        let roots = load_json_corpus(
            root.path(),
            |path| session_ledger::CursorDir::new(path.to_path_buf()),
            &mut sessions,
            &mut discovered_count,
            DEFAULT_MAX_DISCOVERED_SESSIONS,
        )
        .expect("discover Cursor agent transcripts");

        assert_eq!(roots, 1);
        assert_eq!(discovered_count, 1);
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "cursor-agent-1");
        assert_eq!(sessions[0].messages[0].content, "hello from the Cursor agent");
    }

    #[test]
    fn bounded_window_indexes_all_files_but_loads_only_newest_ids() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        #[derive(Clone)]
        struct CountingSource {
            ids: Vec<String>,
            loads: Arc<AtomicUsize>,
        }

        impl session_ledger::ports::CorpusSource for CountingSource {
            fn list(&self) -> Result<Vec<String>, session_ledger::ports::PortError> {
                Ok(self.ids.clone())
            }

            fn load(&self, id: &str) -> Result<Session, session_ledger::ports::PortError> {
                self.loads.fetch_add(1, Ordering::SeqCst);
                let mut session = Session::new(id, session_ledger::domain::session::Corpus::Codex);
                session.messages.push(session_ledger::domain::session::Message::new(
                    session_ledger::domain::session::Role::User,
                    "payload",
                ));
                Ok(session)
            }
        }

        let root = tempfile::tempdir().expect("temp root");
        let ids = (0..8).map(|index| format!("session-{index:02}.jsonl")).collect::<Vec<_>>();
        for id in &ids {
            std::fs::write(root.path().join(id), b"indexed").expect("index fixture");
        }
        // A non-transcript file is ignored by real adapters' `list` method;
        // the count below therefore remains an honest transcript-file count.
        std::fs::write(root.path().join("notes.txt"), b"not a transcript").expect("non-transcript");

        let loads = Arc::new(AtomicUsize::new(0));
        let mut sessions = Vec::new();
        let mut discovered_count = 0;
        let ids_for_source = ids.clone();
        let loads_for_source = Arc::clone(&loads);
        load_json_corpus(
            root.path(),
            move |_| CountingSource { ids: ids_for_source, loads: loads_for_source },
            &mut sessions,
            &mut discovered_count,
            3,
        )
        .expect("bounded discovery");

        assert_eq!(discovered_count, ids.len());
        assert_eq!(loads.load(Ordering::SeqCst), 3);
        assert_eq!(sessions.len(), 3);
        assert!(sessions.iter().all(|session| session.id.ends_with(".jsonl")));
        assert!(sessions.iter().any(|session| session.id == "session-07.jsonl"));
    }

    #[test]
    fn recent_ids_is_deterministic_when_metadata_ties() {
        let root = tempfile::tempdir().expect("temp root");
        let ids = ["session-b.jsonl", "session-a.jsonl", "session-c.jsonl"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        for id in &ids {
            std::fs::write(root.path().join(id), b"indexed").expect("index fixture");
        }

        let first = recent_ids(root.path(), ids.clone(), ids.len());
        let second = recent_ids(root.path(), ids.clone(), ids.len());
        assert_eq!(first, second, "filesystem index ordering must be stable");
        assert_eq!(first.len(), 3);
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
