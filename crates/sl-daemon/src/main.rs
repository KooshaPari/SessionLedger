//! `sl-daemon` — SessionLedger daemon and CLI companion.
//!
//! ## Subcommands
//!
//! | Command     | Description                                               |
//! |-------------|-----------------------------------------------------------|
//! | `sl-daemon serve`  | Start the file-watcher daemon (long-running or `--once`)  |
//! | `sl-daemon status` | Check daemon liveness (`GET /healthz`)                    |
//! | `sl-daemon list`   | List compiled OKF bundle paths (`GET /api/bundles`)       |
//! | `sl-daemon tail`   | Stream new bundle paths as they arrive (`GET /api/stream`)|
//! | `sl-daemon export` | Export bundle metadata as CSV / Markdown / JSON            |
//! | `sl-daemon summary`| Print aggregate statistics across all bundles              |
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

mod archive;
mod audit;
mod cli;
mod etl;
mod export;
mod filter;
mod http;
mod metrics;
#[cfg(feature = "otel")]
mod otel;
mod tag;
mod validation;
mod watcher;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::path::Path;
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info, info_span, warn, Instrument};

/// Channel depth. Bounded so a slow consumer applies backpressure to the
/// watcher instead of letting an unbounded queue grow without limit.
const CHANNEL_CAPACITY: usize = 256;

/// Broadcast channel capacity for SSE notifications.
const BROADCAST_CAPACITY: usize = 256;

const CLI_AFTER_HELP: &str = r#"Examples:
  sl-daemon serve --watch ~/.cursor/agent-transcripts --out ./okf-out
  sl-daemon serve --watch ./sessions --out ./okf-out --once --http-bind 127.0.0.1:8080
  curl -X POST http://127.0.0.1:8080/api/ingest \
    -H 'content-type: application/json' --data-binary @bundle.json
  sl-daemon completions zsh > _sl-daemon
"#;

const SERVE_AFTER_HELP: &str = r#"Examples:
  sl-daemon serve --watch ~/.cursor/agent-transcripts --out ./okf-out
  sl-daemon serve --watch ./sessions --out ./okf-out --once
  sl-daemon serve --watch ./sessions --out ./okf-out --http-bind off

The HTTP listener is local-only. Use --http-bind off for batch ingest/export jobs
that do not need /healthz, /api/bundles, /api/stream, or /api/ingest.
"#;

/// SessionLedger — daemon and CLI companion.
#[derive(Parser, Debug)]
#[command(name = "sl-daemon", version, about, after_help = CLI_AFTER_HELP)]
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
    #[command(after_help = SERVE_AFTER_HELP)]
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

    /// Archive bundles older than a given date by gzipping them.
    Archive {
        /// Archive bundles with created_at strictly before this date (YYYY-MM-DD).
        #[arg(long)]
        before: String,

        /// Directory containing the bundle JSON files (and where the archive
        /// sub-tree will be created).
        #[arg(long, default_value = ".")]
        data_dir: std::path::PathBuf,

        /// Print what would be archived without touching the filesystem.
        #[arg(long)]
        dry_run: bool,
    },

    /// Restore a previously archived bundle by decompressing it.
    Restore {
        /// Bundle ID (without extension) to restore from the archive.
        bundle_id: String,

        /// Directory that contains the `archive/` sub-tree.
        #[arg(long, default_value = ".")]
        data_dir: std::path::PathBuf,

        /// Directory to write the restored `.okf.json` file into.
        /// Defaults to `<data_dir>`.
        #[arg(long)]
        out: Option<PathBuf>,
    },

    /// Replay a compiled OKF bundle, streaming its entities in chronological
    /// order.  Connects to the running daemon's SSE endpoint unless
    /// `--bundle` points to a local file.
    Replay {
        /// Bundle ID (e.g. `sess-abc`) or path to a `.okf.json` file.
        bundle_id: String,

        /// Playback speed multiplier (default 1.0).  `--speed 2.0` replays
        /// at 2× real-time.
        #[arg(long, default_value = "1.0")]
        speed: f64,

        /// Print all entities at once without any delay.
        #[arg(long)]
        no_stream: bool,
    },

    /// Validate an OKF bundle by sending it to the daemon's ingest endpoint.
    ///
    /// Reads the bundle file at `<data_dir>/<bundle_id>.okf.json`, re-packages
    /// the metadata as a `PostBundle`, and calls `POST /api/ingest`.  Exits 0
    /// when valid, 1 when invalid (errors printed to stdout as JSON), 2 on error.
    Validate {
        /// Bundle ID (filename stem, without `.okf.json`).
        bundle_id: String,

        /// Directory containing the `.okf.json` files.
        #[arg(long, default_value = ".")]
        data_dir: PathBuf,
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

    /// Generate shell completion scripts to stdout.
    ///
    /// Supported shells: bash, zsh, fish, powershell.
    Completions {
        /// Shell to generate completions for.
        #[arg(value_enum)]
        shell: Shell,
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
    #[cfg(feature = "otel")]
    let otel_provider = init_tracing();
    #[cfg(not(feature = "otel"))]
    init_tracing();
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
        Command::Archive { before, data_dir, dry_run } => {
            run_archive(&before, &data_dir, dry_run);
        }
        Command::Restore { bundle_id, data_dir, out } => {
            run_restore(&bundle_id, &data_dir, out.as_deref());
        }
        Command::Replay { bundle_id, speed, no_stream } => {
            run_replay(&args.url, &bundle_id, speed, no_stream).await;
        }
        Command::Validate { bundle_id, data_dir } => {
            run_validate(&bundle_id, &data_dir);
        }
        Command::Search { since, until, model, min_tokens, tags, limit, format, bundles } => {
            run_search(&args.url, since, until, model, min_tokens, tags, limit, &format, &bundles)
                .await;
        }
        Command::Completions { shell } => {
            run_completions(shell);
        }
    }

    #[cfg(feature = "otel")]
    if let Some(provider) = otel_provider {
        let _ = provider.shutdown();
    }

    Ok(())
}

fn run_completions(shell: Shell) {
    let mut command = Args::command();
    generate(shell, &mut command, "sl-daemon", &mut std::io::stdout());
}

/// Install a `tracing` subscriber filtered by `RUST_LOG`.
///
/// Default when unset: `sl_daemon=info` (matches docs/ops/observability.md).
#[cfg(not(feature = "otel"))]
fn init_tracing() {
    use tracing_subscriber::EnvFilter;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sl_daemon=info"));

    #[cfg(feature = "json-logs")]
    if json_logs_requested() {
        tracing_subscriber::fmt().json().with_env_filter(filter.clone()).with_target(true).init();
        return;
    }

    tracing_subscriber::fmt().with_env_filter(filter).with_target(true).init();
}

#[cfg(feature = "json-logs")]
fn json_logs_requested() -> bool {
    std::env::var("SL_LOG_FORMAT").is_ok_and(|value| value.eq_ignore_ascii_case("json"))
}

/// Install the formatting subscriber and, when configured, OTLP trace export.
#[cfg(feature = "otel")]
fn init_tracing() -> Option<opentelemetry_sdk::trace::SdkTracerProvider> {
    use tracing_subscriber::EnvFilter;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sl_daemon=info"));
    let endpoint = std::env::var("SL_OTLP_ENDPOINT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .ok()
                .filter(|value| !value.trim().is_empty())
        });

    if let Some(endpoint) = endpoint {
        #[cfg(feature = "json-logs")]
        let json_logs = json_logs_requested();
        #[cfg(not(feature = "json-logs"))]
        let json_logs = false;
        match otel::init(filter, &endpoint, json_logs) {
            Ok(provider) => return Some(provider),
            Err(error) => {
                eprintln!(
                    "warning: failed to initialize OTLP export for {endpoint:?}: {error}; \
                     continuing with local logs"
                );
            }
        }
    }

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("sl_daemon=info"));
    #[cfg(feature = "json-logs")]
    if json_logs_requested() {
        tracing_subscriber::fmt().json().with_env_filter(filter.clone()).with_target(true).init();
        return None;
    }

    tracing_subscriber::fmt().with_env_filter(filter).with_target(true).init();
    None
}

fn parse_local_http_addr(value: &str) -> Result<SocketAddr, String> {
    let addr: SocketAddr =
        value.parse().map_err(|error| format!("invalid --http-bind address {value:?}: {error}"))?;
    if !addr.ip().is_loopback() {
        return Err(format!(
            "--http-bind must use a loopback address (127.0.0.0/8 or ::1), got {addr}"
        ));
    }
    Ok(addr)
}

fn audit_event(
    sink: &audit::AuditSink,
    action: &'static str,
    outcome: &'static str,
    resource: &dyn std::fmt::Display,
) {
    let request_id = audit::local_request_id();
    let resource = resource.to_string();
    info!(
        target: "sl_daemon::audit",
        event_kind = "audit",
        actor = audit::LOCAL_ACTOR,
        action,
        outcome,
        request_id,
        resource = %resource,
        "local operation"
    );
    let event = audit::AuditEvent {
        timestamp: audit::timestamp_unix_ms(),
        actor: audit::LOCAL_ACTOR,
        action,
        outcome,
        request_id: &request_id,
        reason: None,
        resource: Some(resource),
    };
    if let Err(error) = sink.append(&event) {
        warn!(error = %error, path = %sink.path().display(), "failed to append audit event");
    }
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
    let data_dir = audit::data_dir_from_env_or(&out);
    let audit_sink = Arc::new(audit::AuditSink::new(&data_dir));

    // Consumer: drain the channel, transforming each path.
    let bcast_for_consumer = bcast_tx.clone();
    let audit_sink_for_consumer = audit_sink.clone();
    let consumer = tokio::spawn(
        async move {
            let mut total = 0usize;
            while let Some(path) = rx.recv().await {
                let span = info_span!("etl.transform", path = %path.display());
                async {
                    match etl::transform_file(&path, &out_dir) {
                        Ok(written) => {
                            total += written.len();
                            for w in &written {
                                let _ = bcast_for_consumer.send(w.clone());
                                audit_event(
                                    &audit_sink_for_consumer,
                                    "export",
                                    "succeeded",
                                    &w.display(),
                                );
                            }
                            info!(
                                path = %path.display(),
                                okf_docs = written.len(),
                                "ETL transform ok"
                            );
                        }
                        Err(err) => {
                            audit_event(
                                &audit_sink_for_consumer,
                                "export",
                                "failed",
                                &path.display(),
                            );
                            error!(path = %path.display(), error = %err, "ETL transform failed");
                        }
                    }
                }
                .instrument(span)
                .await;
            }
            total
        }
        .instrument(info_span!("etl.consumer")),
    );

    // HTTP server (optional).
    let http_handle = if http_bind.eq_ignore_ascii_case("off") {
        None
    } else {
        let addr = parse_local_http_addr(&http_bind)?;
        let ingest_admission = http::IngestAdmission::from_env()?;
        let api_key_auth = http::ApiKeyAuth::from_env();
        let state = http::AppState {
            out_dir: Arc::new(out.clone()),
            broadcast_tx: bcast_tx.clone(),
            http_metrics: Arc::new(metrics::HttpMetrics::default()),
            ingest_admission,
            api_key_auth,
            audit_sink: audit_sink.clone(),
            idempotency_cache: http::IngestIdempotencyCache::default(),
        };
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            if let Err(e) = http::serve(addr, state, async move {
                let _ = shutdown_rx.await;
            })
            .await
            {
                error!(error = %e, "HTTP server error");
            }
        });
        info!(%addr, "HTTP server listening");
        Some((handle, shutdown_tx))
    };

    if once {
        let sent = watcher::scan_once(&watch, &tx).await?;
        info!(enqueued = sent, "once: scan complete");
        drop(tx);
        let total = consumer.await?;
        info!(okf_docs = total, "once: ETL complete");
        if let Some((handle, shutdown_tx)) = http_handle {
            let _ = shutdown_tx.send(());
            let _ = handle.await;
        }
        return Ok(());
    }

    // Long-running mode.
    watcher::scan_once(&watch, &tx).await?;
    let _watcher = watcher::spawn_fs_watcher(&watch, tx.clone())?;
    info!(watch = %watch.display(), out = %out.display(), "watching for sessions");

    drop(tx);

    tokio::signal::ctrl_c().await?;
    info!("shutting down");
    drop(_watcher);

    if let Some((handle, shutdown_tx)) = http_handle {
        let _ = shutdown_tx.send(());
        let _ = handle.await;
    }

    let total = consumer.await?;
    info!(okf_docs = total, "ETL consumer finished");
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
// archive
// ---------------------------------------------------------------------------

fn run_archive(before_str: &str, data_dir: &Path, dry_run: bool) {
    let before = match chrono::NaiveDate::parse_from_str(before_str, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: invalid --before date {before_str:?}: {e}");
            std::process::exit(2);
        }
    };

    if dry_run {
        println!("[dry-run] bundles that would be archived (before {before}):");
    }

    match archive::archive_bundles(data_dir, before, dry_run) {
        Ok(stats) => {
            let audit_sink = audit::AuditSink::new(data_dir);
            audit_event(
                &audit_sink,
                "archive",
                if dry_run { "dry_run" } else { "succeeded" },
                &stats.archive_dir.display(),
            );
            let mb_saved = stats.bytes_saved as f64 / 1_048_576.0;
            println!(
                "Archived {} bundle(s), saved {:.2} MB  (archive: {})",
                stats.archived_count,
                mb_saved,
                stats.archive_dir.display()
            );
        }
        Err(e) => {
            let audit_sink = audit::AuditSink::new(data_dir);
            audit_event(&audit_sink, "archive", "failed", &data_dir.display());
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// restore
// ---------------------------------------------------------------------------

fn run_restore(bundle_id: &str, data_dir: &Path, out: Option<&Path>) {
    let audit_sink = audit::AuditSink::new(data_dir);
    let archive_root = data_dir.join("archive");
    let archive_path = match archive::find_archive_path(&archive_root, bundle_id) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    };

    let output_dir = out.unwrap_or(data_dir);
    match archive::restore_bundle(&archive_path, output_dir) {
        Ok(restored) => {
            audit_event(&audit_sink, "restore", "succeeded", &restored.display());
            println!("Restored: {}", restored.display());
        }
        Err(e) => {
            audit_event(&audit_sink, "restore", "failed", &archive_path.display());
            eprintln!("error: {e}");
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// validate
// ---------------------------------------------------------------------------

fn run_validate(bundle_id: &str, data_dir: &Path) {
    use validation::{PostBundle, PostMessage};

    let path = data_dir.join(format!("{bundle_id}.okf.json"));
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: cannot read {}: {e}", path.display());
            std::process::exit(2);
        }
    };

    let value: serde_json::Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: cannot parse {}: {e}", path.display());
            std::process::exit(2);
        }
    };

    // Re-package the on-disk OKF fields into a PostBundle for validation.
    let get_str = |key: &str| {
        value
            .get(key)
            .or_else(|| value.pointer(&format!("/metadata/{key}")))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned()
    };
    let get_i64 = |key: &str| {
        value
            .get(key)
            .or_else(|| value.pointer(&format!("/metadata/{key}")))
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
    };

    // Build PostMessages from the OKF entities array (label → content, type → role).
    let messages: Vec<PostMessage> = value
        .get("entities")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .map(|ent| {
                    let role =
                        ent.get("type").and_then(|v| v.as_str()).unwrap_or("assistant").to_owned();
                    let content =
                        ent.get("label").and_then(|v| v.as_str()).unwrap_or_default().to_owned();
                    PostMessage { role, content }
                })
                .collect()
        })
        .unwrap_or_default();

    let bundle = PostBundle {
        bundle_id: {
            let id = get_str("source_id");
            if id.is_empty() {
                bundle_id.to_owned()
            } else {
                id
            }
        },
        created_at: {
            let ca = get_str("created_at");
            // OKF documents may not carry created_at; fall back to a sentinel
            // so the validator produces a useful diagnostic rather than silently
            // accepting an empty string.
            if ca.is_empty() {
                String::new()
            } else {
                ca
            }
        },
        messages,
        token_count: get_i64("token_count"),
    };

    let result = validation::validate_okf_bundle(&bundle);
    let json = serde_json::to_string_pretty(&result).unwrap_or_default();
    println!("{json}");
    if !result.valid {
        std::process::exit(1);
    }
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

// ---------------------------------------------------------------------------
// replay
// ---------------------------------------------------------------------------

/// Format a timestamp from seconds as `HH:MM:SS`.
pub(crate) fn format_timestamp(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

/// Truncate `text` to at most `max_chars` characters, appending `…` when cut.
pub(crate) fn truncate(text: &str, max_chars: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max_chars {
        text.to_owned()
    } else {
        let mut s: String = chars[..max_chars.saturating_sub(1)].iter().collect();
        s.push('\u{2026}');
        s
    }
}

/// Format one OKF entity into a single stdout line.
///
/// Output: `[HH:MM:SS] <type>/<id>: <label_preview>`
pub(crate) fn format_entity_line(entity: &serde_json::Value, event_index: usize) -> String {
    let ts = format_timestamp(event_index as u64);
    let entity_type = entity.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
    let entity_id = entity.get("id").and_then(|v| v.as_str()).unwrap_or("?");
    let label = entity.get("label").and_then(|v| v.as_str()).unwrap_or("");
    let preview = truncate(label, 80);
    format!("[{ts}] {entity_type}/{entity_id}: {preview}")
}

async fn run_replay(base_url: &str, bundle_id: &str, speed: f64, no_stream: bool) {
    use futures_util::TryStreamExt as _;
    use tokio::io::{AsyncBufReadExt, BufReader};
    use tokio_util::io::StreamReader;

    // Build `?speed=N` only when not no_stream (speed irrelevant then).
    let speed_param = if no_stream { String::new() } else { format!("?speed={speed}") };
    let url = format!("{}/api/replay/{bundle_id}{speed_param}", base_url.trim_end_matches('/'));

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

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        eprintln!("server error {status}: {body}");
        std::process::exit(2);
    }

    let byte_stream = resp.bytes_stream().map_err(std::io::Error::other);
    let stream_reader = StreamReader::new(byte_stream);
    let mut lines = BufReader::new(stream_reader).lines();

    // Parse SSE line-by-line; accumulate data for each event block.
    let mut current_event: Option<String> = None;
    let mut current_data = String::new();

    let print_entity = |data: &str| {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
            // Done sentinel.
            if v.as_object().is_some_and(|o| o.is_empty()) {
                println!("[replay complete]");
                return;
            }
            let idx = v.get("event_index").and_then(|x| x.as_u64()).unwrap_or(0) as usize;
            if let Some(entity) = v.get("entity") {
                println!("{}", format_entity_line(entity, idx));
            }
        }
    };

    tokio::select! {
        _ = async {
            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(stripped) = line.strip_prefix("event: ") {
                    current_event = Some(stripped.to_owned());
                } else if let Some(data) = line.strip_prefix("data: ") {
                    current_data = data.to_owned();
                } else if line.is_empty() {
                    // End of event block.
                    if matches!(current_event.as_deref(), Some("done")) {
                        println!("[replay complete]");
                        break;
                    }
                    if !current_data.is_empty() {
                        print_entity(&current_data);
                    }
                    current_event = None;
                    current_data.clear();
                }
            }
        } => {}
        _ = tokio::signal::ctrl_c() => {
            println!("\nstopped.");
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_timestamp_zero() {
        assert_eq!(format_timestamp(0), "00:00:00");
    }

    #[test]
    fn format_timestamp_one_hour_one_min_one_sec() {
        assert_eq!(format_timestamp(3661), "01:01:01");
    }

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 80), "hello");
    }

    #[test]
    fn truncate_long_string_ends_with_ellipsis() {
        let s = "a".repeat(100);
        let result = truncate(&s, 80);
        assert!(result.ends_with('\u{2026}'));
        assert_eq!(result.chars().count(), 80);
    }

    #[test]
    fn format_entity_line_produces_expected_format() {
        let entity = serde_json::json!({
            "type": "intent",
            "id": "intent-0",
            "label": "fix the pagination bug"
        });
        let line = format_entity_line(&entity, 0);
        assert!(line.starts_with("[00:00:00]"));
        assert!(line.contains("intent/intent-0"));
        assert!(line.contains("fix the pagination bug"));
    }

    #[test]
    fn format_entity_line_missing_fields_use_defaults() {
        let entity = serde_json::json!({});
        let line = format_entity_line(&entity, 5);
        assert!(line.starts_with("[00:00:05]"));
        assert!(line.contains("unknown/?"));
    }

    #[test]
    fn http_bind_accepts_only_loopback_addresses() {
        assert!(parse_local_http_addr("127.0.0.1:8080").is_ok());
        assert!(parse_local_http_addr("[::1]:8080").is_ok());
        assert!(parse_local_http_addr("0.0.0.0:8080").is_err());
        assert!(parse_local_http_addr("[::]:8080").is_err());
    }
}
