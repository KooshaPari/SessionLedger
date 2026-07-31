# Viewer discovery benchmark

This is an evidence-only baseline harness for the `sl-viewer` corpus-loader
path. It measures opening the committed Forge SQLite fixture and converting its
two valid conversations into viewer sessions. It is deliberately smaller than a
user-home scan: it is hermetic, deterministic, and isolates the UI-ready
discovery path from operator-specific corpus size and filesystem layout.

## What it measures

`crates/sl-viewer/benches/discovery.rs` calls the public
`load_sessions(DataSource::ForgeDb(...))` path against
`crates/sl-viewer/tests/fixtures/forge_fixture.db`. The fixture contract is two
successfully parsed sessions; a changed fixture or failed discovery aborts the
benchmark.

The benchmark does not launch Dioxus, render a frame, crawl `$HOME`, or claim a
release/readiness score. It is a repeatable discovery baseline only.

## Run

Run from the repository root with an isolated target directory when other Cargo
work may be active:

```bash
CARGO_TARGET_DIR="$PWD/target-viewer-discovery" \
cargo bench -p sl-viewer --no-default-features --features sqlite --bench discovery
```

For a fast local sample, use the same Criterion controls used by the pipeline
benchmark pattern:

```bash
SESSION_LEDGER_VIEWER_DISCOVERY_SAMPLE_SIZE=10 \
SESSION_LEDGER_VIEWER_DISCOVERY_WARM_UP_SECONDS=1 \
SESSION_LEDGER_VIEWER_DISCOVERY_MEASUREMENT_SECONDS=2 \
CARGO_TARGET_DIR="$PWD/target-viewer-discovery" \
cargo bench -p sl-viewer --no-default-features --features sqlite --bench discovery
```

Criterion stores the local result under the configured target directory. Record
the machine, OS, Rust version, feature set, and Criterion mean/p95 before
proposing any threshold. Existing `docs/ops/perf-baseline.json` thresholds apply
only to `benches/pipeline.rs`; this new benchmark has no enforced ceiling yet.

## Acceptance

The evidence command is valid when Criterion completes the named benchmark and
the fixture-load assertion remains true. A number from one machine is not a
performance gate, UI SLA, or release claim. Stable-hardware repeated runs and a
separate checked-in policy are required before adding an enforced threshold.
