//! Consumer pool — reads JSONL session paths off a shared channel, runs the
//! compile → OKF export pipeline, and writes results to an output directory.

use crate::{DaemonError, is_jsonl};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};
use tracing::{error, info, info_span, Instrument};

/// Run a pool of `num_workers` consumers that receive session paths from `rx`,
/// compile each session into a [`ContinuationBundle`], export as OKF, and write
/// the resulting JSON document into `out_dir`.
///
/// Each worker runs in its own `tokio::spawn` task. File I/O and the
/// (CPU-bound) compile/export pipeline are executed on blocking threads via
/// `tokio::task::spawn_blocking`. The function returns when the channel closes
/// (all senders dropped) and all workers finish their current work.
#[tracing::instrument(skip(rx), fields(out_dir = %out_dir.display(), num_workers))]
pub async fn run_worker_pool(
    rx: Arc<Mutex<UnboundedReceiver<PathBuf>>>,
    out_dir: PathBuf,
    num_workers: usize,
) {
    info!("starting worker pool");
    let mut handles = Vec::with_capacity(num_workers);
    for i in 0..num_workers {
        let rx = Arc::clone(&rx);
        let out = out_dir.clone();
        let worker_span = info_span!("worker", worker_id = i);
        handles.push(tokio::spawn(
            async move {
                loop {
                    let path = {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    };
                    let Some(path) = path else {
                        // Channel closed — shut down this worker.
                        info!(worker_id = i, "worker shutting down");
                        break;
                    };
                    if let Err(e) = process_session_file(&path, &out).await {
                        error!(worker_id = i, path = %path.display(), error = %e, "session processing failed");
                    }
                }
            }
            .instrument(worker_span),
        ));
    }

    for h in handles {
        let _ = h.await;
    }
    info!("worker pool drained");
}

/// Read a JSONL file, compile each session in it, export to OKF, and write
/// each export as `{session_id}.okf.json` into `out_dir`.
#[tracing::instrument(skip(out_dir), fields(path = %path.display(), out_dir = %out_dir.display()))]
async fn process_session_file(path: &Path, out_dir: &Path) -> Result<(), DaemonError> {
    // Reject non-JSONL files early.
    if !is_jsonl(path) {
        return Ok(());
    }

    let path = path.to_path_buf();
    let out = out_dir.to_path_buf();

    tokio::task::spawn_blocking(move || -> Result<(), DaemonError> {
        let sessions = session_ledger::read_jsonl_sessions(&path)?;
        info!(sessions = sessions.len(), "compiling sessions");

        for session in &sessions {
            let doc = session_ledger::process_session(session);
            let filename = format!("{}.okf.json", session.id);
            let out_path = out.join(&filename);
            let file = std::fs::File::create(&out_path)?;
            serde_json::to_writer_pretty(file, &doc)?;
            info!(session_id = %session.id, out = %out_path.display(), "wrote OKF");
        }

        Ok(())
    })
    .await
    .map_err(std::io::Error::other)?
}
