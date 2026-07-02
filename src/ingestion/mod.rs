//! Ingestion adapters — one per corpus. Each normalizes a raw transcript into a
//! [`crate::domain::session::Session`] and implements [`crate::ports::CorpusSource`].
//!
//! Phase 1 ships the corpus enumeration + the forge adapter contract; the
//! remaining adapters (codex/claude-code/cursor) follow the same trait. See
//! `docs/DESIGN.md` for each corpus's on-disk shape.

pub mod claude_code;
pub mod codex;
pub mod cursor;
pub mod forge;

use crate::domain::session::Session;
use std::io::BufRead;
use std::path::Path;

/// Errors that can occur during ingestion.
#[derive(Debug, thiserror::Error)]
pub enum IngestionError {
    /// Underlying I/O error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// JSON parse error on a line.
    #[error("JSON parse error at line {line}: {msg}")]
    Json { line: usize, msg: String },
}

/// Parse a JSONL reader into a [`Vec<Session>`].
///
/// Each non-empty line in the reader must be a JSON-serialized [`Session`].
/// Lines are numbered from 1 for error reporting.
///
/// # Errors
///
/// Returns [`IngestionError::Io`] on read failures or
/// [`IngestionError::Json`] if a line is not valid session JSON.
pub fn parse_jsonl_sessions<R: std::io::Read>(reader: R) -> Result<Vec<Session>, IngestionError> {
    let mut sessions: Vec<Session> = Vec::new();
    for (line_num, line) in std::io::BufReader::new(reader).lines().enumerate() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let session: Session = serde_json::from_str(trimmed).map_err(|e| IngestionError::Json {
            line: line_num + 1,
            msg: e.to_string(),
        })?;
        sessions.push(session);
    }
    Ok(sessions)
}

/// Read a JSONL file and parse each line as a [`Session`].
///
/// This is a convenience wrapper over [`parse_jsonl_sessions`] that opens the
/// file at `path` and delegates to the reader-based parser.
///
/// # Errors
///
/// Returns [`IngestionError::Io`] if the file cannot be opened or read, or
/// [`IngestionError::Json`] if a line is not valid session JSON.
pub fn read_jsonl_sessions<P: AsRef<Path>>(path: P) -> Result<Vec<Session>, IngestionError> {
    let file = std::fs::File::open(path)?;
    parse_jsonl_sessions(file)
}
