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
