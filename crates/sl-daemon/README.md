# sl-daemon

SessionLedger ETL daemon. Watches a directory of `*.jsonl` session transcripts
and, for every new or changed file, runs the session-ledger pipeline
(**ingest → compile → export**), writing one `<session-id>.okf.json` per session.

```text
  watcher (notify / scan) ──Sender<PathBuf>──▶ mpsc(256) ──▶ consumer (ETL) ──▶ *.okf.json
```

## Build & test (isolated)

Always build/test this crate **in isolation** — never the bare workspace root,
because `sl-viewer` pulls `webkit2gtk-sys`, which does not resolve on macOS:

```bash
cargo test -p sl-daemon
cargo run  -p sl-daemon -- --watch ~/.forge/sessions --out ./okf-out
cargo run  -p sl-daemon -- --watch ./sessions --out ./okf-out --once   # single sweep
```

Optional OTLP/gRPC trace export is feature-gated, so normal builds do not need
a collector or the OpenTelemetry exporter dependencies:

```bash
cargo build -p sl-daemon --features otel
SL_OTLP_ENDPOINT=http://localhost:4317 cargo run -p sl-daemon --features otel -- serve --watch ./sessions --out ./okf-out
```

`OTEL_EXPORTER_OTLP_ENDPOINT` is also accepted when `SL_OTLP_ENDPOINT` is not
set. If neither variable is set, the feature-enabled binary uses the same local
fmt logs and `RUST_LOG` filtering as the default build.

## Run options

### 1. Native process-compose (preferred for local dev — no container)

```bash
SL_WATCH_DIR=~/.forge/sessions SL_OUT_DIR=./okf-out \
  process-compose -f crates/sl-daemon/process-compose.yaml up
```

### 2. Apple `container` (default OCI runtime — OSS, per-container VM)

Apple `container` is the workspace default OCI runtime; Docker/OrbStack are
labelled fallbacks only. From the repo root:

```bash
container build -t sl-daemon:latest -f crates/sl-daemon/Containerfile .
container run --rm \
  -v "$HOME/.forge/sessions:/data/sessions:ro" \
  -v "$PWD/okf-out:/data/out" \
  sl-daemon:latest --watch /data/sessions --out /data/out
```

## Flags

| Flag | Meaning |
|------|---------|
| `--watch <dir>` | Directory of `*.jsonl` transcripts to watch (required) |
| `--out <dir>`   | Where `<id>.okf.json` files are written (auto-created) |
| `--once`        | Single deterministic sweep, then exit (CI / cron) |

The HTTP listener is local-only: `--http-bind` must use `127.0.0.0/8` or `::1`.
Ingest admission is configured with `SL_INGEST_MAX_BODY_BYTES` (default
`1048576`) and `SL_INGEST_MAX_CONCURRENCY` (default `8`). See
[`docs/ops/local-trust-boundary.md`](../../docs/ops/local-trust-boundary.md) for
the error envelope, audit fields, and operational boundary.
