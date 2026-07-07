//! # `sl-daemon` — ETL producer-consumer pipeline for SessionLedger.
//!
//! A file-system watcher (producer) tails a sessions directory for new/changed
//! JSONL session files, feeds paths through a `tokio::mpsc` channel, and a
//! configurable consumer pool runs the existing session-ledger compile → OKF
//! export pipeline, writing `.okf.json` documents into an output directory.
//!
//! # Architecture
//!
//! ```text
//! notify (file events)
//!   └─→ SessionWatcher (producer)
//!         └─→ tokio::mpsc (unbounded channel)
//!               └─→ Worker Pool (N consumers)
//!                     └─→ read_jsonl → compile → export_to_okf → write .okf.json
//! ```
//!
//! # Example
//!
//! ```no_run
//! use std::path::PathBuf;
//! use tokio::sync::mpsc;
//! use sl_daemon::{spawn_fs_watcher, run_worker_pool, DaemonError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), DaemonError> {
//!     let (tx, rx) = mpsc::unbounded_channel();
//!     let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
//!
//!     let _handle = spawn_fs_watcher(PathBuf::from("./sessions"), tx)?;
//!     run_worker_pool(rx, PathBuf::from("./out"), 4).await;
//!     Ok(())
//! }
//! ```

pub mod watcher;
pub mod worker;

pub use watcher::list_jsonl;
pub use watcher::spawn_fs_watcher;
pub use worker::run_worker_pool;

use std::path::Path;

/// Errors that can occur during daemon operation.
#[derive(Debug, thiserror::Error)]
pub enum DaemonError {
    /// Underlying I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Notify (file-watcher) error.
    #[error("file watcher error: {0}")]
    Notify(#[from] notify::Error),

    /// SessionLedger ingestion error.
    #[error("ingestion error: {0}")]
    Ingestion(#[from] session_ledger::IngestionError),

    /// JSON serialization error.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Determine whether a path has a `.jsonl` extension.
pub(crate) fn is_jsonl(path: &Path) -> bool {
    path.extension().map_or(false, |e| e == "jsonl")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_jsonl_accepts_jsonl_extension() {
        assert!(is_jsonl(Path::new("session.jsonl")));
    }

    #[test]
    fn is_jsonl_rejects_non_jsonl() {
        assert!(!is_jsonl(Path::new("session.txt")));
        assert!(!is_jsonl(Path::new("session.json")));
        assert!(!is_jsonl(Path::new("session")));
    }
}
