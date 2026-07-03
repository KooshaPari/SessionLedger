//! Watcher — the *producer* half of the daemon's ETL pipeline.
//!
//! Two source modes feed the same `mpsc::Sender<PathBuf>`:
//!
//! * [`spawn_fs_watcher`] — event-driven via the `notify` crate (FSEvents on
//!   macOS). Best for a long-running daemon: near-zero idle cost, sub-second
//!   latency on new/modified `*.jsonl` files.
//! * [`scan_once`] — a single deterministic sweep of the directory. Used by
//!   `--once` mode and by tests, where event timing must not be relied on.
//!
//! Keeping the poll path deterministic (no sleeps, no wall-clock) is what lets
//! the consumer be unit-tested without flakiness.

use std::path::{Path, PathBuf};

use notify::{Event, EventKind, RecursiveMode, Watcher};
use tokio::sync::mpsc;

/// Return every `*.jsonl` file directly under `dir`, sorted for determinism.
///
/// Non-recursive by design: session corpora are flat directories of transcript
/// files. Sorting makes the emitted order stable so tests can assert on it.
pub fn list_jsonl(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

/// Perform one deterministic sweep, sending each discovered path downstream.
///
/// Returns the number of paths emitted. Errors from `read_dir` propagate; a
/// closed receiver ends the sweep early (returns what was sent so far).
pub async fn scan_once(dir: &Path, tx: &mpsc::Sender<PathBuf>) -> std::io::Result<usize> {
    let paths = list_jsonl(dir)?;
    let mut sent = 0;
    for path in paths {
        if tx.send(path).await.is_err() {
            break;
        }
        sent += 1;
    }
    Ok(sent)
}

/// Spawn an event-driven `notify` watcher on `dir`.
///
/// New/modified `*.jsonl` paths are forwarded on `tx`. The returned
/// [`notify::RecommendedWatcher`] MUST be kept alive by the caller — dropping it
/// tears down the OS watch. Runs until the receiver is dropped.
pub fn spawn_fs_watcher(
    dir: &Path,
    tx: mpsc::Sender<PathBuf>,
) -> notify::Result<notify::RecommendedWatcher> {
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        let Ok(event) = res else { return };
        if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
            return;
        }
        for path in event.paths {
            if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
                // `blocking_send` is correct here: the notify callback runs on
                // its own OS thread, not inside the tokio runtime.
                if tx.blocking_send(path).is_err() {
                    return;
                }
            }
        }
    })?;
    watcher.watch(dir, RecursiveMode::NonRecursive)?;
    Ok(watcher)
}
