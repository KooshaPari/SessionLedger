//! Consumer pool — reads JSONL session paths off a shared channel, runs the
//! compile → OKF export pipeline, and writes results to an output directory.

use crate::{DaemonError, is_jsonl};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};

/// Run a pool of `num_workers` consumers that receive session paths from `rx`,
/// compile each session into a [`ContinuationBundle`], export as OKF, and write
/// the resulting JSON document into `out_dir`.
///
/// Each worker runs in its own `tokio::spawn` task. File I/O and the
/// (CPU-bound) compile/export pipeline are executed on blocking threads via
/// `tokio::task::spawn_blocking`. The function returns when the channel closes
/// (all senders dropped) and all workers finish their current work.
pub async fn run_worker_pool(
    rx: Arc<Mutex<UnboundedReceiver<PathBuf>>>,
    out_dir: PathBuf,
    num_workers: usize,
) {
    let mut handles = Vec::with_capacity(num_workers);
    for i in 0..num_workers {
        let rx = Arc::clone(&rx);
        let out = out_dir.clone();
        handles.push(tokio::spawn(async move {
            loop {
                let path = {
                    let mut guard = rx.lock().await;
                    guard.recv().await
                };
                let Some(path) = path else {
                    // Channel closed — shut down this worker.
                    break;
                };
                if let Err(e) = process_session_file(&path, &out).await {
                    eprintln!("[worker {i}] error processing {:?}: {e}", path);
                }
            }
        }));
    }

    for h in handles {
        let _ = h.await;
    }
}

/// Read a JSONL file, compile each session in it, export to OKF, and write
/// each export as `{session_id}.okf.json` into `out_dir`.
async fn process_session_file(
    path: &PathBuf,
    out_dir: &PathBuf,
) -> Result<(), DaemonError> {
    // Reject non-JSONL files early.
    if !is_jsonl(path) {
        return Ok(());
    }

    let path = path.clone();
    let out = out_dir.clone();

    tokio::task::spawn_blocking(move || -> Result<(), DaemonError> {
        let sessions = session_ledger::read_jsonl_sessions(&path)?;

        for session in &sessions {
            let doc = session_ledger::process_session(session);
            let filename = format!("{}.okf.json", session.id);
            let out_path = out.join(&filename);
            let file = std::fs::File::create(&out_path)?;
            serde_json::to_writer_pretty(file, &doc)?;
        }

        Ok(())
    })
    .await
    .map_err(|join_err| std::io::Error::new(std::io::ErrorKind::Other, join_err))?
}
