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
//! ```
//!
//! * `--once` does a single deterministic sweep then exits (CI / cron-friendly).
//! * default mode watches forever until SIGINT (Ctrl-C).

mod etl;
mod watcher;

use std::path::PathBuf;

use clap::Parser;
use tokio::sync::mpsc;

/// Channel depth. Bounded so a slow consumer applies backpressure to the
/// watcher instead of letting an unbounded queue grow without limit.
const CHANNEL_CAPACITY: usize = 256;

#[derive(Parser, Debug)]
#[command(
    name = "sl-daemon",
    about = "Watch a session directory and export OKF documents"
)]
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
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (tx, mut rx) = mpsc::channel::<PathBuf>(CHANNEL_CAPACITY);
    let out_dir = args.out.clone();

    // Consumer: drain the channel, transforming each path. Errors are logged
    // (loudly, to stderr) but do not tear down the daemon — one malformed file
    // must not stop the pipeline.
    let consumer = tokio::spawn(async move {
        let mut total = 0usize;
        while let Some(path) = rx.recv().await {
            match etl::transform_file(&path, &out_dir) {
                Ok(written) => {
                    total += written.len();
                    eprintln!("[sl-daemon] {} → {} OKF doc(s)", path.display(), written.len());
                }
                Err(err) => eprintln!("[sl-daemon] ERROR {err}"),
            }
        }
        total
    });

    if args.once {
        // Deterministic single sweep, then close the channel so the consumer
        // finishes and we can report the total.
        let sent = watcher::scan_once(&args.watch, &tx).await?;
        eprintln!("[sl-daemon] --once: enqueued {sent} file(s)");
        drop(tx);
        let total = consumer.await?;
        eprintln!("[sl-daemon] --once: wrote {total} OKF doc(s)");
        return Ok(());
    }

    // Long-running mode: seed with an initial sweep, then hand off to the
    // event-driven watcher. `_watcher` must stay alive for the OS watch to hold.
    watcher::scan_once(&args.watch, &tx).await?;
    let _watcher = watcher::spawn_fs_watcher(&args.watch, tx.clone())?;
    eprintln!(
        "[sl-daemon] watching {} → {}",
        args.watch.display(),
        args.out.display()
    );

    // Drop our own extra sender so the channel can close once the watcher is
    // torn down on shutdown.
    drop(tx);

    tokio::signal::ctrl_c().await?;
    eprintln!("[sl-daemon] shutting down");
    drop(_watcher);
    let total = consumer.await?;
    eprintln!("[sl-daemon] wrote {total} OKF doc(s) total");
    Ok(())
}
