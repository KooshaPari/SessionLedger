//! ETL transform — the *consumer* half of the pipeline.
//!
//! For each `*.jsonl` path handed over by the watcher, run the full
//! session-ledger pipeline (ingest → compile → export) and write one
//! `<session-id>.okf.json` per session into the output directory.
//!
//! The heavy lifting lives in the root `session-ledger` crate; this module is a
//! thin, well-tested adapter that turns a file path into on-disk OKF documents.

use std::path::{Path, PathBuf};

use session_ledger::export::okf::export_to_okf;
use session_ledger::ports::MemoryStore;
use session_ledger::{compile_and_store, process_session, read_jsonl_sessions};

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
    let sessions = read_jsonl_sessions(jsonl_path)
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
        std::fs::write(&out_path, json)
            .map_err(|source| EtlError::Write { path: out_path.clone(), source })?;
        written.push(out_path);
    }
    Ok(written)
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
