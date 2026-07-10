# Runbook — local SessionLedger

How to run the daemon + viewer stack and verify liveness / readiness.

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
| `sl-viewer` | `cargo run -p sl-viewer` | waits until daemon **readiness** probe passes |

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

Two probes — do not conflate them. Full policy:
[`observability.md`](observability.md#healthz-vs-readyz).

| Probe | Meaning | Expect |
|-------|---------|--------|
| `GET /healthz` | **Liveness** — process accepts HTTP | `200`, body `ok` |
| `GET /readyz` | **Readiness** — `out_dir` exists and is usable | `200`, body `ready`; else `503` |

```bash
curl -s -o /dev/null -w "%{http_code} " http://127.0.0.1:8080/healthz
curl -s http://127.0.0.1:8080/healthz
# 200 ok

curl -s -o /dev/null -w "%{http_code} " http://127.0.0.1:8080/readyz
curl -s http://127.0.0.1:8080/readyz
# 200 ready  (requires SL_DATA_DIR / out_dir to exist)
# 503        if out_dir missing — daemon may still be "alive" on /healthz
```

Readiness probe in `process-compose.yaml` hits **`/readyz`** on port **8080**
(`initial_delay_seconds: 3`). Viewer start depends on that probe, not `/healthz`.

If `/healthz` is `ok` but `/readyz` is `503`: create/fix `SL_DATA_DIR` (default
`./.sl-data`); do not treat it as a crash loop by itself.

## Metrics

```bash
curl -s http://127.0.0.1:8080/api/metrics | jq .
```

Returns `total_bundles`, `total_tokens`, `avg_tokens`, `model_counts`,
`daily_counts` over JSON bundles in the data/out directory. See
[`observability.md`](observability.md) for RED mapping and SLO stubs.

## Common failures

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| `process-compose: command not found` | CLI missing | Install process-compose; or run crates manually |
| Viewer never starts | Daemon not **ready** | Confirm `/readyz` returns `ready`; `/healthz` alone is insufficient; check port 8080 free; raise probe delay |
| `/healthz` ok, `/readyz` 503 | Missing or non-dir `out_dir` | Ensure `SL_DATA_DIR` exists; mkdir if needed; restart serve |
| `Address already in use` | Stale daemon | Kill process on 8080; `make dev-down` |
| Empty metrics / bundles | Wrong data dir | Set `SL_DATA_DIR`; ensure `*.okf.json` / `*.json` under out dir |
| `cargo` / MSRV errors | Wrong toolchain | `rustup show`; use repo `rust-toolchain.toml` |
| Viewer build fails (webkit) | Platform GTK deps | Prefer `cargo test -p sl-daemon` isolation; see `crates/sl-daemon/README.md` |
| Ingest 4xx | Invalid OKF payload | Run `sl validate`; see `validation.rs` / FR-002 |

## Related

- [`observability.md`](observability.md) — SLO stubs, RED map, `/healthz` vs `/readyz`, OTel/#65
- [`alerts.md`](alerts.md) — alert rule stubs (not wired)
- [`../functional_requirements.md`](../functional_requirements.md) — FR-014
- [`../../AGENTS.md`](../../AGENTS.md) — agent build/test norms
