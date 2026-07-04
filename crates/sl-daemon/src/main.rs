//! `sl` — SessionLedger daemon and CLI companion.
//!
//! ## Subcommands
//!
//! | Command     | Description                                               |
//! |-------------|-----------------------------------------------------------|
//! | `sl serve`  | Start the file-watcher daemon (long-running or `--once`)  |
//! | `sl status` | Check daemon liveness (`GET /healthz`)                    |
//! | `sl list`   | List compiled OKF bundle paths (`GET /api/bundles`)       |
//! | `sl tail`   | Stream new bundle paths as they arrive (`GET /api/stream`)|
//!
//! ## HTTP API (exposed by `sl serve`)
//!
//! * `GET /healthz` — liveness probe
//! * `GET /api/bundles` — current OKF documents as JSON array
//! * `GET /api/stream` — SSE stream of new `*.okf.json` paths
//!
//! ## Exit codes
//!
//! * `0` — success (or daemon running, for `status`)
//! * `1` — daemon not running (for `status` / `tail` when daemon absent)
//! * `2` — general / unexpected error

mod cli;
mod etl;
mod http;
mod watcher;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::sync::{broadcast, mpsc};

/// Channel depth. Bounded so a slow consumer applies backpressure to the
/// watcher instead of letting an unbounded queue grow without limit.
const CHANNEL_CAPACITY: usize = 256;

/// Broadcast channel capacity for SSE notifications.
const BROADCAST_CAPACITY: usize = 256;

/// SessionLedger — daemon and CLI companion.
#[derive(Parser, Debug)]
#[command(name = "sl", version, about)]
struct Args {
    /// Base URL of the daemon HTTP server (used by status / list / tail).
    #[arg(long, global = true, default_value = cli::DEFAULT_BASE_URL)]
    url: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the file-watcher daemon.
    Serve {
        /// Directory to watch for `*.jsonl` session transcripts.
        #[arg(long)]
        watch: PathBuf,

        /// Directory to write `<session-id>.okf.json` files into.
        #[arg(long)]
        out: PathBuf,

        /// Do a single sweep of `--watch` then exit (CI / cron-friendly).
        #[arg(long)]
        once: bool,

        /// Address to bind the HTTP server on (e.g. `127.0.0.1:8080`).
        /// Pass `off` to disable the HTTP server entirely.
        #[arg(long, default_value = "127.0.0.1:8080")]
        http_bind: String,
    },

    /// Check daemon status (exit 0 = running, exit 1 = not running).
    Status,

    /// List compiled OKF bundle paths (one per line).
    List,

    /// Stream new bundle paths as they arrive (SSE). Press Ctrl+C to stop.
    Tail,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Serve { watch, out, once, http_bind } => {
            run_serve(watch, out, once, http_bind).await?;
        }
        Command::Status => run_status(&args.url).await,
        Command::List => run_list(&args.url).await,
        Command::Tail => run_tail(&args.url).await,
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// serve
// ---------------------------------------------------------------------------

async fn run_serve(
    watch: PathBuf,
    out: PathBuf,
    once: bool,
    http_bind: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Broadcast channel: ETL consumer publishes every written path; HTTP SSE
    // handler subscribes one receiver per connected client.
    let (bcast_tx, _bcast_rx) = broadcast::channel::<PathBuf>(BROADCAST_CAPACITY);

    let (tx, mut rx) = mpsc::channel::<PathBuf>(CHANNEL_CAPACITY);
    let out_dir = out.clone();

    // Consumer: drain the channel, transforming each path.
    let bcast_for_consumer = bcast_tx.clone();
    let consumer = tokio::spawn(async move {
        let mut total = 0usize;
        while let Some(path) = rx.recv().await {
            match etl::transform_file(&path, &out_dir) {
                Ok(written) => {
                    total += written.len();
                    for w in &written {
                        let _ = bcast_for_consumer.send(w.clone());
                    }
                    eprintln!("[sl-daemon] {} → {} OKF doc(s)", path.display(), written.len());
                }
                Err(err) => eprintln!("[sl-daemon] ERROR {err}"),
            }
        }
        total
    });

    // HTTP server (optional).
    let http_handle = if http_bind.eq_ignore_ascii_case("off") {
        None
    } else {
        let addr: SocketAddr = http_bind
            .parse()
            .map_err(|e| format!("invalid --http-bind address {http_bind:?}: {e}"))?;
        let state =
            http::AppState { out_dir: Arc::new(out.clone()), broadcast_tx: bcast_tx.clone() };
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

    if once {
        let sent = watcher::scan_once(&watch, &tx).await?;
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
    watcher::scan_once(&watch, &tx).await?;
    let _watcher = watcher::spawn_fs_watcher(&watch, tx.clone())?;
    eprintln!("[sl-daemon] watching {} → {}", watch.display(), out.display());

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

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

async fn run_status(base_url: &str) {
    let client = reqwest::Client::new();
    match cli::fetch_health(&client, base_url).await {
        Ok(cli::HealthStatus::Running { body }) => {
            println!("daemon running — {body}");
            std::process::exit(0);
        }
        Ok(cli::HealthStatus::NotRunning) => {
            println!("daemon not running");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

async fn run_list(base_url: &str) {
    let client = reqwest::Client::new();
    match cli::fetch_bundle_paths(&client, base_url).await {
        Ok(paths) => {
            for p in &paths {
                println!("{p}");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// tail
// ---------------------------------------------------------------------------

async fn run_tail(base_url: &str) {
    use futures_util::TryStreamExt as _;
    use tokio::io::{AsyncBufReadExt, BufReader};
    use tokio_util::io::StreamReader;

    let url = cli::build_url(base_url, "/api/stream");
    let client = reqwest::Client::new();

    let resp = match client.get(&url).header("Accept", "text/event-stream").send().await {
        Ok(r) => r,
        Err(e) if e.is_connect() => {
            eprintln!("daemon not running at {base_url}");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    };

    let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
    let stream_reader = StreamReader::new(byte_stream);
    let mut lines = BufReader::new(stream_reader).lines();

    tokio::select! {
        _ = async {
            while let Ok(Some(line)) = lines.next_line().await {
                // SSE data lines: "data: <payload>"
                if let Some(payload) = line.strip_prefix("data: ") {
                    println!("{payload}");
                }
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            println!("\nstopped.");
        }
    }
}
