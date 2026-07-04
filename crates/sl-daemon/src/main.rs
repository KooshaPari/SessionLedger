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
//! | `sl export` | Export bundle metadata as CSV / Markdown / JSON            |
//! | `sl summary`| Print aggregate statistics across all bundles              |
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
mod export;
mod filter;
mod http;
mod tag;
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

    /// Export bundle metadata as CSV, Markdown, or JSON.
    ///
    /// When no bundle paths are given, all bundles are fetched from the daemon
    /// via GET /api/bundles.  Each bundle path must point to an OKF JSON file
    /// on the local filesystem.
    Export {
        /// Output format: csv | md | json  (default: csv).
        #[arg(long, default_value = "csv")]
        format: String,

        /// Write output to this file; defaults to stdout.
        #[arg(long)]
        out: Option<PathBuf>,

        /// OKF bundle file paths to export.  If omitted, fetches all from
        /// the daemon.
        bundles: Vec<PathBuf>,
    },

    /// Print aggregate statistics across all bundles.
    Summary {
        /// OKF bundle file paths to summarise.  If omitted, fetches all from
        /// the daemon.
        bundles: Vec<PathBuf>,
    },

    /// Manage tags on OKF bundle files.
    Tag {
        #[command(subcommand)]
        action: TagAction,
    },

    /// Search / filter bundles by date, model, token count, or tags.
    ///
    /// When no `--bundles` paths are given, all bundles are fetched from the
    /// daemon via GET /api/bundles.
    Search {
        /// Include only bundles created on or after this date (YYYY-MM-DD).
        #[arg(long)]
        since: Option<String>,

        /// Include only bundles created on or before this date (YYYY-MM-DD).
        #[arg(long)]
        until: Option<String>,

        /// Include only bundles whose model name contains this substring
        /// (case-insensitive).
        #[arg(long)]
        model: Option<String>,

        /// Include only bundles with at least this many tokens.
        #[arg(long)]
        min_tokens: Option<u64>,

        /// Include only bundles that carry this tag (repeat for AND logic).
        #[arg(long = "tag", action = clap::ArgAction::Append)]
        tags: Vec<String>,

        /// Maximum number of results to return (default: 50).
        #[arg(long, default_value = "50")]
        limit: usize,

        /// Output format: text | json | csv  (default: text).
        #[arg(long, default_value = "text")]
        format: String,

        /// OKF bundle file paths to search. If omitted, fetches all from the
        /// daemon.
        bundles: Vec<PathBuf>,
    },
}

/// Sub-actions for `sl tag`.
#[derive(Subcommand, Debug)]
enum TagAction {
    /// Add one or more tags to a bundle.
    Add {
        /// Path to the `.okf.json` bundle file.
        bundle: PathBuf,
        /// Tags to add.
        #[arg(required = true)]
        tags: Vec<String>,
    },

    /// Remove one or more tags from a bundle.
    Remove {
        /// Path to the `.okf.json` bundle file.
        bundle: PathBuf,
        /// Tags to remove.
        #[arg(required = true)]
        tags: Vec<String>,
    },

    /// List current tags on a bundle.
    List {
        /// Path to the `.okf.json` bundle file.
        bundle: PathBuf,
    },

    /// Search a directory for bundles that carry a specific tag.
    Search {
        /// Tag to search for.
        tag: String,
        /// Directory to scan (recursively) for `*.okf.json` files.
        /// Defaults to the current directory.
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
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
        Command::Export { format, out, bundles } => {
            run_export(&args.url, &format, out.as_deref(), &bundles).await;
        }
        Command::Summary { bundles } => {
            run_summary(&args.url, &bundles).await;
        }
        Command::Tag { action } => {
            run_tag(action);
        }
        Command::Search { since, until, model, min_tokens, tags, limit, format, bundles } => {
            run_search(&args.url, since, until, model, min_tokens, tags, limit, &format, &bundles)
                .await;
        }
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

// ---------------------------------------------------------------------------
// export
// ---------------------------------------------------------------------------

/// Load OKF JSON files from `paths` into [`export::BundleMeta`] objects.
///
/// Skips files that cannot be read or parsed, printing a warning to stderr.
fn load_metas(paths: &[PathBuf]) -> Vec<export::BundleMeta> {
    paths
        .iter()
        .filter_map(|p| match std::fs::read_to_string(p) {
            Err(e) => {
                eprintln!("warning: cannot read {}: {e}", p.display());
                None
            }
            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                Err(e) => {
                    eprintln!("warning: cannot parse {}: {e}", p.display());
                    None
                }
                Ok(v) => Some(export::BundleMeta::from_value(&v)),
            },
        })
        .collect()
}

async fn resolve_bundle_paths(base_url: &str, given: &[PathBuf]) -> Vec<PathBuf> {
    if !given.is_empty() {
        return given.to_vec();
    }
    let client = reqwest::Client::new();
    match cli::fetch_bundle_paths(&client, base_url).await {
        Ok(paths) => paths.into_iter().map(PathBuf::from).collect(),
        Err(e) => {
            eprintln!("error fetching bundle list: {e}");
            std::process::exit(2);
        }
    }
}

async fn run_export(
    base_url: &str,
    format_str: &str,
    out_path: Option<&std::path::Path>,
    given_bundles: &[PathBuf],
) {
    use std::str::FromStr as _;

    let fmt = match export::ExportFormat::from_str(format_str) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    };

    let paths = resolve_bundle_paths(base_url, given_bundles).await;
    let metas = load_metas(&paths);

    let rendered = match fmt {
        export::ExportFormat::Csv => export::render_csv(&metas),
        export::ExportFormat::Markdown => export::render_markdown(&metas),
        export::ExportFormat::Json => export::render_json(&metas),
    };

    match out_path {
        Some(p) => {
            if let Err(e) = std::fs::write(p, &rendered) {
                eprintln!("error writing to {}: {e}", p.display());
                std::process::exit(2);
            }
        }
        None => print!("{rendered}"),
    }
}

// ---------------------------------------------------------------------------
// tag
// ---------------------------------------------------------------------------

fn run_tag(action: TagAction) {
    match action {
        TagAction::Add { bundle, tags } => match tag::add(&bundle, &tags) {
            Ok(updated) => {
                eprintln!("tags updated: {:?}", updated);
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        },
        TagAction::Remove { bundle, tags } => match tag::remove(&bundle, &tags) {
            Ok(updated) => {
                eprintln!("tags updated: {:?}", updated);
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        },
        TagAction::List { bundle } => match tag::list(&bundle) {
            Ok(tags) => {
                for t in &tags {
                    println!("{t}");
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        },
        TagAction::Search { tag: search_tag, dir } => match tag::search_dir(&dir, &search_tag) {
            Ok(paths) => {
                for p in &paths {
                    println!("{}", p.display());
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(2);
            }
        },
    }
}

// ---------------------------------------------------------------------------
// summary
// ---------------------------------------------------------------------------

async fn run_summary(base_url: &str, given_bundles: &[PathBuf]) {
    let paths = resolve_bundle_paths(base_url, given_bundles).await;
    let metas = load_metas(&paths);
    let summary = export::compute_summary(&metas);
    print!("{}", export::render_summary(&summary));
}

// ---------------------------------------------------------------------------
// search
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
async fn run_search(
    base_url: &str,
    since: Option<String>,
    until: Option<String>,
    model: Option<String>,
    min_tokens: Option<u64>,
    tags: Vec<String>,
    limit: usize,
    format_str: &str,
    given_bundles: &[PathBuf],
) {
    use std::str::FromStr as _;

    let paths = resolve_bundle_paths(base_url, given_bundles).await;
    let metas = load_metas(&paths);

    let spec = filter::FilterSpec { since, until, model, min_tokens, tags, limit };
    let matched: Vec<&export::BundleMeta> = filter::apply_filters(&metas, &spec);

    if matched.is_empty() {
        eprintln!("no bundles matched the given filters");
        return;
    }

    let owned: Vec<export::BundleMeta> = matched.into_iter().cloned().collect();

    let rendered = match export::ExportFormat::from_str(format_str) {
        Ok(export::ExportFormat::Csv) => export::render_csv(&owned),
        Ok(export::ExportFormat::Markdown) => export::render_markdown(&owned),
        Ok(export::ExportFormat::Json) => export::render_json(&owned),
        // Default / "text": one session_id  created_at  model  token_count  tags line each
        _ => {
            let mut out = String::new();
            for m in &owned {
                out.push_str(&format!(
                    "{}\t{}\t{}\t{}\t[{}]\n",
                    m.session_id,
                    m.created_at,
                    m.model,
                    m.token_count,
                    m.tags.join(", ")
                ));
            }
            out
        }
    };

    print!("{rendered}");
}
