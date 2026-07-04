//! sl-daemon — SessionLedger ETL daemon.
//!
//! Watches a directory of `*.jsonl` session transcripts, and for every
//! new/changed file runs the session-ledger pipeline (ingest → compile →
//! export), writing one `<session-id>.okf.json` per session into an output
//! directory.
//!
//! Architecture is a classic producer/consumer split over a bounded
//! [`tokio::sync::mpsc`] channel:
//!
//! ```text
//!   watcher (notify / scan) ──Sender<PathBuf>──▶ channel ──▶ consumer (ETL)
//!                                                                   │
//!                                                   broadcast::Sender<PathBuf>
//!                                                                   │
//!                                                         HTTP SSE subscribers
//! ```
//!
//! * `--once` does a single deterministic sweep then exits (CI / cron-friendly).
//! * default mode watches forever until SIGINT (Ctrl-C).
//! * `--http-bind` (default `127.0.0.1:8080`) starts an HTTP server exposing:
//!     - `GET /healthz` — liveness probe
//!     - `GET /api/bundles` — current OKF documents as JSON array
//!     - `GET /api/stream` — SSE stream of new `*.okf.json` paths
//!       Pass `off` to disable the HTTP server entirely.

mod etl;
mod http;
mod watcher;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use tokio::sync::{broadcast, mpsc};

/// Channel depth. Bounded so a slow consumer applies backpressure to the
/// watcher instead of letting an unbounded queue grow without limit.
const CHANNEL_CAPACITY: usize = 256;

/// Broadcast channel capacity for SSE notifications.
const BROADCAST_CAPACITY: usize = 256;

#[derive(Parser, Debug)]
#[command(name = "sl-daemon", about = "Watch a session directory and export OKF documents")]
struct Args {
    /// Directory to watch for `*.jsonl` session transcripts.
    #[arg(long)]
    watch: PathBuf,

    /// Directory to write `<session-id>.okf.json` files into.
    #[arg(long)]
    out: PathBuf,

    /// Do a single sweep of `--watch` then exit (no long-running watcher).
    #[arg(long)]
    once: bool,

    /// Address to bind the HTTP server on (e.g. `127.0.0.1:8080`).
    ///
    /// Set to `off` to disable the HTTP server entirely.
    /// When absent the default `127.0.0.1:8080` is used.
    #[arg(long, default_value = "127.0.0.1:8080")]
    http_bind: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Broadcast channel: ETL consumer publishes every written path; HTTP SSE
    // handler subscribes one receiver per connected client.
    let (bcast_tx, _bcast_rx) = broadcast::channel::<PathBuf>(BROADCAST_CAPACITY);

    let (tx, mut rx) = mpsc::channel::<PathBuf>(CHANNEL_CAPACITY);
    let out_dir = args.out.clone();

    // Consumer: drain the channel, transforming each path.
    let bcast_for_consumer = bcast_tx.clone();
    let consumer = tokio::spawn(async move {
        let mut total = 0usize;
        while let Some(path) = rx.recv().await {
            match etl::transform_file(&path, &out_dir, Some(&bcast_for_consumer)) {
                Ok(written) => {
                    total += written.len();
                    eprintln!("[sl-daemon] {} → {} OKF doc(s)", path.display(), written.len());
                }
                Err(err) => eprintln!("[sl-daemon] ERROR {err}"),
            }
        }
        total
    });

    // HTTP server (optional).
    let http_handle = if args.http_bind.eq_ignore_ascii_case("off") {
        None
    } else {
        let addr: SocketAddr = args
            .http_bind
            .parse()
            .map_err(|e| format!("invalid --http-bind address {:?}: {e}", args.http_bind))?;
        let state =
            http::AppState { out_dir: Arc::new(args.out.clone()), broadcast_tx: bcast_tx.clone() };
        // Use a one-shot channel so the server shuts down cleanly on Ctrl-C.
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            if let Err(e) = http::serve(addr, state, async move {
                let _ = shutdown_rx.await;
            })
            .await
            {
                eprintln!("[sl-daemon] HTTP server error: {e}");
            }
        });
        eprintln!("[sl-daemon] HTTP server listening on http://{addr}");
        Some((handle, shutdown_tx))
    };

    if args.once {
        let sent = watcher::scan_once(&args.watch, &tx).await?;
        eprintln!("[sl-daemon] --once: enqueued {sent} file(s)");
        drop(tx);
        let total = consumer.await?;
        eprintln!("[sl-daemon] --once: wrote {total} OKF doc(s)");
        if let Some((handle, shutdown_tx)) = http_handle {
            let _ = shutdown_tx.send(());
            let _ = handle.await;
        }
        return Ok(());
    }

    // Long-running mode.
    watcher::scan_once(&args.watch, &tx).await?;
    let _watcher = watcher::spawn_fs_watcher(&args.watch, tx.clone())?;
    eprintln!("[sl-daemon] watching {} → {}", args.watch.display(), args.out.display());

    drop(tx);

    tokio::signal::ctrl_c().await?;
    eprintln!("[sl-daemon] shutting down");
    drop(_watcher);

    if let Some((handle, shutdown_tx)) = http_handle {
        let _ = shutdown_tx.send(());
        let _ = handle.await;
    }

    let total = consumer.await?;
    eprintln!("[sl-daemon] wrote {total} OKF doc(s) total");
    Ok(())
}
