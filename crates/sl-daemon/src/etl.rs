//! ETL transform — the *consumer* half of the pipeline.
//!
//! For each `*.jsonl` path handed over by the watcher, run the full
//! session-ledger pipeline (ingest → compile → export) and write one
//! `<session-id>.okf.json` per session into the output directory.
//!
//! The heavy lifting lives in the root `session-ledger` crate; this module is a
//! thin, well-tested adapter that turns a file path into on-disk OKF documents.

use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use session_ledger::export::okf::export_to_okf;
use session_ledger::ports::{CorpusSource, MemoryStore};
use session_ledger::{
    compile_and_store, process_session, read_jsonl_sessions, ClaudeDir, CodexDir, CursorDir,
};

/// Errors surfaced while transforming one JSONL file into OKF documents.
#[derive(Debug, thiserror::Error)]
pub enum EtlError {
    #[error("ingestion failed for {path}: {source}")]
    Ingest {
        path: PathBuf,
        #[source]
        source: session_ledger::IngestionError,
    },
    #[error("writing OKF for {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("memory persistence failed for {path}: {source}")]
    Memory {
        path: PathBuf,
        #[source]
        source: session_ledger::ports::PortError,
    },
    #[error("serializing OKF: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Compile + export every session in `jsonl_path`, writing one
/// `<session-id>.okf.json` per session under `out_dir`.
///
/// When `memory_store` is set, distilled episodic facts are persisted through
/// the configured [`MemoryStore`] before OKF export.
///
/// Returns the paths written (stable order — same as the sessions in the file).
/// `out_dir` is created if missing.
pub fn transform_file(
    jsonl_path: &Path,
    out_dir: &Path,
    memory_store: Option<&dyn MemoryStore>,
) -> Result<Vec<PathBuf>, EtlError> {
    let sessions = read_sessions(jsonl_path)
        .map_err(|source| EtlError::Ingest { path: jsonl_path.to_path_buf(), source })?;

    std::fs::create_dir_all(out_dir)
        .map_err(|source| EtlError::Write { path: out_dir.to_path_buf(), source })?;

    let mut written = Vec::with_capacity(sessions.len());
    for session in &sessions {
        let doc = if let Some(store) = memory_store {
            let output = compile_and_store(session, store)
                .map_err(|source| EtlError::Memory { path: jsonl_path.to_path_buf(), source })?;
            export_to_okf(&output.bundle, session.corpus.as_str())
        } else {
            process_session(session)
        };
        let json = serde_json::to_string_pretty(&doc)?;
        let out_path = out_dir.join(format!("{}.okf.json", sanitize(&session.id)));
        write_okf_atomically(&out_path, &json)
            .map_err(|source| EtlError::Write { path: out_path.clone(), source })?;
        written.push(out_path);
    }
    Ok(written)
}

/// Publish an OKF document by renaming a fully-written sibling file into place.
/// Readers of `/api/bundles` therefore observe either the previous complete
/// bundle or the new complete bundle, never partially serialized JSON.
fn write_okf_atomically(path: &Path, json: &str) -> std::io::Result<()> {
    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("bundle.okf.json");
    let temp = path.with_file_name(format!(".{file_name}.tmp-{}-{sequence}", std::process::id()));

    std::fs::write(&temp, json)?;
    std::fs::rename(&temp, path)
}

fn read_sessions(
    path: &Path,
) -> Result<Vec<session_ledger::Session>, session_ledger::IngestionError> {
    // Auto-discovered Claude/Cursor roots contain native event records rather
    // than normalized `Session` JSONL. Route those paths through the existing
    // corpus adapters so automatic ingestion preserves corpus provenance.
    if let Some(source) = native_source(path) {
        return read_native_session(path, source);
    }

    let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    if name.ends_with(".jsonl.zst") || has_codex_session_metadata(path)? {
        return read_codex_session(path);
    }

    read_jsonl_sessions(path)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeSource {
    Claude,
    Cursor,
}

/// Identify native transcript roots without changing the explicit `--watch`
/// contract. Files outside these roots continue through strict normalized
/// JSONL parsing, while the native adapters own their tool-specific records.
fn native_source(path: &Path) -> Option<NativeSource> {
    if has_path_pair(path, ".claude", "projects") {
        return Some(NativeSource::Claude);
    }
    if has_path_pair(path, ".cursor", "projects")
        || has_path_pair(path, ".cursor", "agent-transcripts")
    {
        return Some(NativeSource::Cursor);
    }
    None
}

fn has_path_pair(path: &Path, first: &str, second: &str) -> bool {
    let mut previous = None;
    for component in path.components() {
        let current = component.as_os_str().to_str();
        if previous == Some(first) && current == Some(second) {
            return true;
        }
        previous = current;
    }
    false
}

fn read_native_session(
    path: &Path,
    source_kind: NativeSource,
) -> Result<Vec<session_ledger::Session>, session_ledger::IngestionError> {
    let source: Box<dyn CorpusSource> = match source_kind {
        NativeSource::Claude => Box::new(ClaudeDir::new(path)),
        NativeSource::Cursor => Box::new(CursorDir::new(path)),
    };
    let id = source
        .list()
        .map_err(|error| native_ingestion_error(path, error.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| native_ingestion_error(path, "native transcript is empty"))?;
    let session =
        source.load(&id).map_err(|error| native_ingestion_error(path, error.to_string()))?;
    // Native roots may contain JSON metadata/configuration files alongside
    // conversations. Never emit an empty synthetic bundle for those files.
    if session.messages.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![session])
}

fn native_ingestion_error(
    path: &Path,
    message: impl Into<String>,
) -> session_ledger::IngestionError {
    session_ledger::IngestionError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("native transcript {}: {}", path.display(), message.into()),
    ))
}

fn has_codex_session_metadata(path: &Path) -> Result<bool, session_ledger::IngestionError> {
    let file = std::fs::File::open(path)?;
    for line in std::io::BufReader::new(file).lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(record) = serde_json::from_str::<serde_json::Value>(trimmed) else {
            return Ok(false);
        };
        return Ok(record.get("type").and_then(serde_json::Value::as_str) == Some("session_meta"));
    }
    Ok(false)
}

fn read_codex_session(
    path: &Path,
) -> Result<Vec<session_ledger::Session>, session_ledger::IngestionError> {
    let source = CodexDir::new(path);
    let id = source
        .list()
        .map_err(|error| std::io::Error::other(error.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed transcript is empty")
        })?;
    let session = source.load(&id).map_err(|error| std::io::Error::other(error.to_string()))?;
    Ok(vec![session])
}

/// Make a session id safe to use as a filename (path separators → `_`).
fn sanitize(id: &str) -> String {
    id.chars().map(|c| if matches!(c, '/' | '\\' | ':') { '_' } else { c }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_ledger::domain::session::Corpus;
    use session_ledger::{Message, Role, Session};

    /// Build a JSONL file of `n` forge sessions in `dir`, return its path.
    ///
    /// Mirrors the on-disk shape `read_jsonl_sessions` expects: one serialized
    /// `Session` per line (see the root crate's `tests/skeleton.rs`).
    fn write_fixture(dir: &Path, n: usize) -> PathBuf {
        let mut buf = String::new();
        for i in 0..n {
            let mut s = Session::new(format!("sess-{i}"), Corpus::Forge);
            s.title = Some(format!("task {i}"));
            s.messages.push(Message::new(Role::User, "add pagination to the users endpoint"));
            s.messages.push(Message::new(Role::Assistant, "on it — adding a cursor param"));
            s.messages.push(Message::new(Role::User, "lgtm, ship it"));
            buf.push_str(&serde_json::to_string(&s).expect("serialize session"));
            buf.push('\n');
        }
        let path = dir.join("sessions.jsonl");
        std::fs::write(&path, buf).expect("write fixture");
        path
    }

    #[test]
    fn transform_file_writes_one_okf_per_session() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let jsonl = write_fixture(tmp.path(), 3);
        let out = tmp.path().join("out");

        let written = transform_file(&jsonl, &out, None).expect("transform");

        assert_eq!(written.len(), 3, "one OKF doc per session");
        for (i, path) in written.iter().enumerate() {
            assert!(path.exists(), "{path:?} should exist");
            let content = std::fs::read_to_string(path).expect("read okf");
            let doc: serde_json::Value = serde_json::from_str(&content).expect("okf is valid json");
            assert_eq!(doc["source_id"], format!("sess-{i}"));
            assert_eq!(doc["provenance"]["corpus"], "forge");
        }
    }

    #[test]
    fn transform_file_creates_missing_out_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let jsonl = write_fixture(tmp.path(), 1);
        let out = tmp.path().join("nested").join("deeper");
        assert!(!out.exists());

        let written = transform_file(&jsonl, &out, None).expect("transform");
        assert_eq!(written.len(), 1);
        assert!(out.is_dir(), "out dir auto-created");
    }

    #[test]
    fn atomic_okf_write_publishes_complete_document_without_temp_artifacts() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let out = tmp.path().join("out");
        std::fs::create_dir_all(&out).expect("out dir");
        let target = out.join("session.okf.json");

        write_okf_atomically(&target, "{\"okf\":\"1.0\"}").expect("atomic write");

        assert_eq!(std::fs::read_to_string(&target).unwrap(), "{\"okf\":\"1.0\"}");
        assert!(
            std::fs::read_dir(&out)
                .unwrap()
                .flatten()
                .all(|entry| !entry.file_name().to_string_lossy().contains(".tmp-")),
            "a bundle listing must never observe a temporary OKF artifact"
        );
    }

    #[test]
    fn transform_file_reads_compressed_codex_transcript() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("codex.jsonl.zst");
        let input = format!(
            "{}\n{}\n",
            serde_json::json!({"type":"session_meta","payload":{"id":"zst-session"}}),
            serde_json::json!({"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"ship compressed"}]}})
        );
        let compressed = zstd::stream::encode_all(input.as_bytes(), 3).expect("compress");
        std::fs::write(&path, compressed).expect("write compressed transcript");
        let written = transform_file(&path, &tmp.path().join("out"), None).expect("transform");
        assert_eq!(written.len(), 1);
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&written[0]).unwrap()).unwrap();
        assert_eq!(doc["source_id"], "zst-session");
    }

    #[test]
    fn transform_file_reads_plain_codex_session_meta_transcript() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("codex.jsonl");
        let input = format!(
            "{}\n{}\n",
            serde_json::json!({"type":"session_meta","payload":{"id":"plain-codex-session"}}),
            serde_json::json!({"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"ship plain codex"}]}})
        );
        std::fs::write(&path, input).expect("write plain transcript");

        let written = transform_file(&path, &tmp.path().join("out"), None)
            .expect("transform plain Codex transcript");

        assert_eq!(written.len(), 1);
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&written[0]).unwrap()).unwrap();
        assert_eq!(doc["source_id"], "plain-codex-session");
    }

    #[test]
    fn transform_file_routes_claude_projects_to_claude_adapter() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join(".claude/projects/demo/session.jsonl");
        std::fs::create_dir_all(path.parent().expect("project directory")).expect("mkdir");
        let input = serde_json::json!({
            "type": "assistant",
            "sessionId": "claude-etl-session",
            "message": {"role": "assistant", "content": [{"type": "text", "text": "done"}]}
        });
        std::fs::write(&path, format!("{input}\n")).expect("write Claude transcript");

        let written = transform_file(&path, &tmp.path().join("out"), None).expect("transform");
        assert_eq!(written.len(), 1);
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&written[0]).unwrap()).unwrap();
        assert_eq!(doc["source_id"], "claude-etl-session");
        assert_eq!(doc["provenance"]["corpus"], "claude-code");
    }

    #[test]
    fn transform_file_routes_cursor_agent_jsonl_to_cursor_adapter() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join(".cursor/agent-transcripts/session.jsonl");
        std::fs::create_dir_all(path.parent().expect("Cursor directory")).expect("mkdir");
        let input = serde_json::json!({
            "conversationId": "cursor-etl-session",
            "messages": [{"role": "user", "content": "hello"}]
        });
        std::fs::write(&path, format!("{{not json}}\n{input}\n"))
            .expect("write Cursor transcript");

        let written = transform_file(&path, &tmp.path().join("out"), None).expect("transform");
        assert_eq!(written.len(), 1);
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&written[0]).unwrap()).unwrap();
        assert_eq!(doc["source_id"], "cursor-etl-session");
        assert_eq!(doc["provenance"]["corpus"], "cursor");
    }

    #[test]
    fn transform_file_routes_cursor_project_json_to_cursor_adapter() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join(".cursor/projects/demo/conversation.json");
        std::fs::create_dir_all(path.parent().expect("Cursor directory")).expect("mkdir");
        let input = serde_json::json!({
            "conversationId": "cursor-json-etl-session",
            "title": "Cursor JSON",
            "messages": [{"role": "user", "content": "hello from Cursor JSON"}]
        });
        std::fs::write(&path, input.to_string()).expect("write Cursor JSON transcript");

        let written = transform_file(&path, &tmp.path().join("out"), None)
            .expect("transform Cursor JSON transcript");
        assert_eq!(written.len(), 1);
        let doc: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&written[0]).unwrap()).unwrap();
        assert_eq!(doc["source_id"], "cursor-json-etl-session");
        assert_eq!(doc["provenance"]["corpus"], "cursor");
    }

    #[test]
    fn native_source_requires_a_supported_root_pair() {
        assert_eq!(
            native_source(Path::new("/tmp/.claude/projects/demo/session.jsonl")),
            Some(NativeSource::Claude)
        );
        assert_eq!(
            native_source(Path::new("/tmp/.cursor/agent-transcripts/session.jsonl")),
            Some(NativeSource::Cursor)
        );
        assert_eq!(
            native_source(Path::new("/tmp/projects/.cursorish/session.jsonl")),
            None
        );
    }

    #[test]
    fn sanitize_replaces_path_separators() {
        assert_eq!(sanitize("a/b:c\\d"), "a_b_c_d");
        assert_eq!(sanitize("plain-id"), "plain-id");
    }

    #[cfg(feature = "sqlite")]
    #[test]
    fn transform_file_persists_distilled_facts_when_memory_store_configured() {
        use session_ledger::SqliteMemoryStore;

        let tmp = tempfile::tempdir().expect("tempdir");
        let jsonl = write_fixture(tmp.path(), 1);
        let out = tmp.path().join("out");
        let memory_path = tmp.path().join("memory.db");
        let store = SqliteMemoryStore::open(&memory_path).expect("open memory db");

        let written =
            transform_file(&jsonl, &out, Some(&store)).expect("transform with memory store");
        assert_eq!(written.len(), 1);

        let recalled = store.recall("pagination", 5).expect("recall distilled facts");
        assert!(!recalled.is_empty(), "distilled episodic facts should persist");
    }
}
