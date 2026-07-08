# Runbook — local SessionLedger

How to run the daemon + viewer stack and verify liveness.

## Prerequisites

- Rust toolchain from `rust-toolchain.toml` (MSRV 1.85+)
- [`process-compose`](https://github.com/F1bonacc1/process-compose) on `PATH`
- Optional: Dioxus CLI for viewer bundling (`cargo install dioxus-cli`)

## Start (`make dev`)

From repo root:

```bash
make build          # cargo build -p sl-daemon -p sl-viewer
make dev            # build, then process-compose up
```

`process-compose.yaml` starts:

| Process | Command | Notes |
|---------|---------|-------|
| `sl-daemon` | `cargo run -p sl-daemon -- serve` | `SL_PORT=8080`, `SL_DATA_DIR=./.sl-data` |
| `sl-viewer` | `cargo run -p sl-viewer` | waits until daemon readiness probe passes |

Tear down:

```bash
make dev-down
# or: process-compose down
```

Manual (without process-compose):

```bash
cargo run -p sl-daemon -- serve
cargo run -p sl-viewer
```

## Health check

```bash
curl -s http://127.0.0.1:8080/healthz
# expect: ok  (HTTP 200)
```

Readiness probe in `process-compose.yaml` hits the same path on port **8080**
(`initial_delay_seconds: 3`).

## Metrics

```bash
curl -s http://127.0.0.1:8080/api/metrics | jq .
```

Returns `total_bundles`, `total_tokens`, `avg_tokens`, `model_counts`,
`daily_counts` over JSON bundles in the data/out directory. See
[`observability.md`](observability.md).

## Common failures

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| `process-compose: command not found` | CLI missing | Install process-compose; or run crates manually |
| Viewer never starts | Daemon not healthy | Confirm `/healthz`; check port 8080 free; raise probe delay |
| `Address already in use` | Stale daemon | Kill process on 8080; `make dev-down` |
| Empty metrics / bundles | Wrong data dir | Set `SL_DATA_DIR`; ensure `*.okf.json` / `*.json` under out dir |
| `cargo` / MSRV errors | Wrong toolchain | `rustup show`; use repo `rust-toolchain.toml` |
| Viewer build fails (webkit) | Platform GTK deps | Prefer `cargo test -p sl-daemon` isolation; see `crates/sl-daemon/README.md` |
| Ingest 4xx | Invalid OKF payload | Run `sl validate`; see `validation.rs` / FR-002 |

## Related

- [`observability.md`](observability.md) — metrics + future OTel
- [`../functional_requirements.md`](../functional_requirements.md) — FR-014
- [`../../AGENTS.md`](../../AGENTS.md) — agent build/test norms
