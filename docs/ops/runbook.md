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

## Headless load smoke

Start `sl-daemon` with a valid data directory, then run the PowerShell 7 load
smoke from another terminal. No viewer or GUI is required.

```powershell
New-Item -ItemType Directory -Force .sl-watch, .sl-data | Out-Null
cargo run --manifest-path crates/sl-daemon/Cargo.toml -- `
  serve --watch .sl-watch --out .sl-data
```

```powershell
pwsh ./scripts/load-smoke.ps1 `
  -BaseUrl http://127.0.0.1:8080 `
  -Requests 400 `
  -Concurrency 20 `
  -MinSuccessRate 99 `
  -MaxP95Ms 500
```

The request count is distributed evenly across `/healthz`, `/readyz`,
`/api/metrics`, and `/metrics`. The script prints per-endpoint results and an
overall success rate and p95 latency. It exits non-zero if the success rate is
below `MinSuccessRate` or p95 exceeds `MaxP95Ms`, so the same command can be
used as a non-blocking nightly or pre-release smoke check. A `503` from
`/readyz` counts as a failure; ensure the directory passed to `--out` exists
before testing.

For a shorter operational chaos pass (readiness fault, metrics shape, kill +
recovery, mini load), run `scripts/ops-chaos-smoke.ps1` after building
`sl-daemon`. CI runs this on weekdays via
[`.github/workflows/ops-chaos-smoke.yml`](../../.github/workflows/ops-chaos-smoke.yml);
see [`observability.md`](observability.md) for the scheduled-evidence table.

## Audit retention and review

The durable audit sink is append-only JSONL (default) or SQLite under
`<data_dir>/audit/`. `sl-daemon` never prunes it; operators rotate by moving or
copying files while the process is stopped. Full policy:
[`local-trust-boundary.md`](local-trust-boundary.md#retention-and-rotation-policy).

```powershell
# Tail recent audit events from the compose data root
pwsh ./scripts/audit-review.ps1 -DataDir ./.sl-data -Tail 20

# Export a date-bounded bundle for compliance review (local files only)
pwsh ./scripts/audit-review.ps1 -DataDir ./.sl-data -Since "2026-07-01" `
  -Export ./review/audit-export.jsonl
```

Audit review does not use HTTP; keep `sl serve` on loopback and treat exported
bundles like security-sensitive local metadata.

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

## CI traceability

Requirements live in [`functional_requirements.md`](../functional_requirements.md),
claimable work maps them in [`PLAN.md`](../../PLAN.md), phased delivery and
organization controls live in [`WBS.md`](WBS.md), and audit acceptance gaps live
in [`GAP_QA_MATRIX.md`](GAP_QA_MATRIX.md). The machine-readable SSOT is
[`TRACEABILITY.json`](TRACEABILITY.json).

Agents changing an FR, PLAN task, WBS package, or cluster status must update the
affected in-document status and `TRACEABILITY.json` in the same change. Run:

```powershell
pwsh ./docs/ops/traceability_lint.ps1
```

The dedicated CI `traceability` job runs this command and fails when the schema
is missing, an FR is absent from JSON, FR catalog and JSON statuses disagree, or
any tracked status is outside `done|partial|todo|blocked|na`. Acceptance
evidence belongs in the linked tests and paths; record intentional gaps in the
matrix instead of silently dropping coverage.

### Coverage ratchet

The blocking CI job runs `cargo llvm-cov --all-features --fail-under-lines 85`,
which is the DESIGN line-coverage target. The latest successful `main` baseline
before the T-038 increment (commit `8733051`) was **98.00% lines**; the portable
default-feature baseline was **98.17% lines**. The gate remains at 85% because it
already matches the DESIGN target; future increments should add branch-relevant
tests and must not lower this floor. The per-module qgate remains non-blocking
until each module reaches 85%.

## Related

- [`observability.md`](observability.md) — SLO stubs, RED map, `/healthz` vs `/readyz`, OTel/#65
- [`alerts.md`](alerts.md) — alert rule stubs (not wired)
- [`WBS.md`](WBS.md) — phased project and organization work packages
- [`GAP_QA_MATRIX.md`](GAP_QA_MATRIX.md) — current audit and acceptance gaps
- [`TRACEABILITY.json`](TRACEABILITY.json) — machine-readable status SSOT
- [`../functional_requirements.md`](../functional_requirements.md) — FR-014
- [`../../AGENTS.md`](../../AGENTS.md) — agent build/test norms
