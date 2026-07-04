# SessionLedger

> Compile, distill, and **resume** agent sessions — losslessly.

SessionLedger is a session-ledger / observability + memory-distillation system for
agent sessions (Forge, Codex, Claude Code, Cursor, …). It:

- **Compiles** any agent session into a structured, observable ledger (OKF-style).
- **Distills** ("dream") a session into short-term + long-term memory stores.
- Serves a **wiki / docs / history viewer** over sessions.
- Produces a canonical, **injectable continuation bundle** for lossless resume:
  an `Acceptance` gate + `Contract` + `Context` + `Intent` (+ `Provenance`,
  `Worklog`, `Dedup`) bundle you can inject into a NEW session.

### Why

1. **Merge duplicate-scoped chats** — after a crash, resume ONE chat, not many.
2. **Recover lost work** from crashed/abandoned sessions into an
   *in-progress / unfinished* section.
3. General session **observability + history**.

## Launch the Viewer

The desktop viewer (`crates/sl-viewer/`) is a Dioxus 0.6 native app that
renders compiled bundles, session history, and distilled memory.

```bash
cargo run -p sl-viewer
```

Opens a native window with three tabs: **Bundles**, **History**, **Memory**.

### Web (WASM) mode

```bash
# one-time install
cargo install dioxus-cli

# serve on http://localhost:8080
dx serve --platform web -p sl-viewer
```

### Prebuilt binaries

Head to the [Releases page](../../releases) — each `v*` tag publishes
per-platform archives (`tar.gz` / `.zip`) with a standalone binary. No Rust
toolchain needed.

## CLI Commands (`sl-daemon`)

The daemon binary exposes four subcommands:

| Command | Description |
|---------|-------------|
| `sl serve` | Start the file-watcher daemon (long-running). |
| `sl status` | Check daemon liveness — exits 0 if running, 1 if not. |
| `sl list` | List compiled OKF bundle paths via the HTTP API. |
| `sl tail --url http://localhost:9001` | Stream new bundle paths as they arrive (SSE). |

### Common flags

- `--url <base-url>` — base URL for `status`, `list`, and `tail` (default: `http://127.0.0.1:8080`).
- `--watch <dir>` / `--out <dir>` — watch/output directories for `serve`.
- `--http-bind <addr>` — bind address for the embedded HTTP server (default: `127.0.0.1:8080`; pass `off` to disable).

### Quick start

```bash
# Build (debug)
cargo build -p sl-daemon

# Start daemon
./crates/sl-daemon/target/debug/sl-daemon serve \
  --watch ~/.forge/sessions \
  --out /tmp/sl-okf-output

# In another terminal — check health
./crates/sl-daemon/target/debug/sl-daemon status

# List compiled bundles
./crates/sl-daemon/target/debug/sl-daemon list

# Tail live bundle events
./crates/sl-daemon/target/debug/sl-daemon tail --url http://localhost:8080
```

Or use `process-compose up` to manage the full service lifecycle (build → serve → readiness check).

## Architecture (hexagonal)

```
ingestion/   per-corpus adapters → normalized Session       (ports::CorpusSource)
domain/      bundle model · intent FSM · dedup keys · session  (pure, no I/O)
distill/     "dream" → ContinuationBundle + memory writes     (ports::MemoryStore)
viewer/      wiki / history read model
ports/       trait boundaries composed from existing Phenotype systems
```

SessionLedger **composes, never duplicates**: forgecode (lifecycle FSM + zstd +
ADR-103 pruning), OmniRoute memory (FTS5 + Qdrant), context-mode / omni-context-rtk
(compression), pheno-tracing / PhenoObservability (observability). See
[`docs/DESIGN.md`](docs/DESIGN.md).

## Build

```bash
cargo build
cargo test
cargo clippy --all-targets   # warnings denied
```

Language: **Rust** (Phenotype scripting-policy default; composition targets are Rust).

## License

Dual-licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE).
