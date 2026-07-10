# Observability — SessionLedger

Current HTTP surfaces, SLO/error-budget stubs, RED metric mapping, and
intended future instrumentation. Soft goals are explicitly non-blocking for
P0 product work. Remaining deep-obs work tracks [issue #65](https://github.com/KooshaPari/SessionLedger/issues/65).

## Current surfaces

| Surface | Path | Purpose |
|---------|------|---------|
| Liveness | `GET /healthz` | Returns `200` + body `ok`. Process is up. |
| Readiness | `GET /readyz` | Returns `200` + `ready` when `out_dir` exists; else `503`. Used by process-compose. |
| Metrics | `GET /api/metrics` | Aggregated bundle stats: totals, avg tokens, model + daily histograms (`crates/sl-daemon/src/metrics.rs`). |
| Live events | `GET /api/stream` | SSE of newly written `*.okf.json` paths (viewer LiveFeed). |
| Replay | `GET /api/replay/:id` | SSE entity playback (not ops metrics; product replay). |

Default bind: port **8080** (`SL_PORT`). See [`runbook.md`](runbook.md).

### `/healthz` vs `/readyz`

| Probe | Question it answers | Success | Failure | Who should call it |
|-------|---------------------|---------|---------|-------------------|
| `/healthz` | Is the HTTP process alive? | `200` + `ok` | Connection refused / timeout | Process supervisors, `sl status`, crude uptime checks |
| `/readyz` | Can the daemon serve work that needs `out_dir`? | `200` + `ready` when `out_dir` exists and is a directory | `503` when `out_dir` is missing or not a dir | `process-compose` readiness probe, load balancers, viewer `depends_on` |

Rules of thumb:

- **Liveness ≠ readiness.** A process can answer `/healthz` while `/readyz` is
  `503` (e.g. `SL_DATA_DIR` / out dir not created yet).
- **Do not** point readiness probes at `/healthz` — that hides data-dir
  misconfiguration and can start the viewer against an unready daemon.
- **Do not** restart solely on `/readyz` failures without checking whether the
  data directory is expected to exist; prefer fixing config over thrashing.
- Future dependency checks (DB, object store, collectors) belong on `/readyz`,
  not `/healthz`. Tracking: issue #65 acceptance + PLAN `T-021`/`T-023`.

## SLO stubs (intentional placeholders)

Local-dev / single-operator SLOs. Not production SLAs.

| SLO | SLI | Target (stub) | Window | Signal today |
|-----|-----|---------------|--------|--------------|
| Daemon availability | Fraction of successful `/readyz` probes | ≥99% during `make dev` sessions | Per session | `/readyz` HTTP status |
| Ingest success rate | Valid OKF posts accepted / total posts | ≥95% | Rolling 24h (manual) | Logs + future RED counter; `/api/metrics` for volume |
| Replay start latency | Time to first SSE event on `/api/replay/:id` | p95 &lt; 2s for fixture bundles | Per fixture run | Manual stopwatch / future histogram |
| Metrics endpoint availability | `/api/metrics` returns `200` | ≥99% when daemon ready | Per session | curl / future scrape |

### Error budget (stub policy)

| Concept | Stub rule |
|---------|-----------|
| Budget | 1% unavailability of `/readyz` during active `make dev` (≈ 36s / hour) |
| Burn action | Log a friction-log entry; do **not** page |
| Exhaustion | Pause non-essential feature work; fix probe/config/data-dir first |
| Reset | New local session / clean `make dev` cycle |

Error budget: treat local-dev SLO misses as friction-log entries, not pages.
Alerting/dashboards remain soft goals — see [Alert stubs](#alert-stubs) and
issue #65 (OTLP + remaining C05 depth).

There is **no** Prometheus scrape endpoint or OTLP exporter in-tree yet.
`TraceSink` is a design port ([`docs/DESIGN.md`](../DESIGN.md) §6) composed with
external Phenotype observability systems.

## RED metrics mapping

[RED](https://www.weave.works/blog/the-red-method-key-metrics-for-microservices-architecture/)
(Rate, Errors, Duration) mapped to SessionLedger surfaces. Columns marked
*stub* are not emitted yet — placeholders for exporters under issue #65.

| RED | Meaning | Current signal | Intended metric (stub name) | Export path (future) |
|-----|---------|----------------|-----------------------------|----------------------|
| **R**ate | Ingest / HTTP request volume | `/api/metrics` `total_bundles`; access logs (none yet) | `sl_ingest_requests_total` | OTLP counter / Prometheus |
| **R**ate | Replay / stream consumers | SSE connect count (not counted) | `sl_sse_clients` | OTLP up-down counter |
| **E**rrors | Failed ingest / 4xx–5xx | Process logs (`error`/`warn`) | `sl_ingest_errors_total{reason}` | OTLP counter |
| **E**rrors | Readiness failures | `/readyz` → `503` | `sl_readyz_failures_total` | OTLP counter |
| **D**uration | Ingest → compile → write | Not timed | `sl_ingest_duration_seconds` | OTLP histogram |
| **D**uration | Replay time-to-first-byte | Manual | `sl_replay_ttfb_seconds` | OTLP histogram |
| **D**uration | `/api/metrics` compute | Not timed | `sl_metrics_handler_duration_seconds` | OTLP histogram |

App-level JSON (`/api/metrics`) stays the product summary. RED exporters must
**not** break that contract — add parallel scrape/OTLP paths instead.

## Alert stubs

Placeholders only — no pager routing wired. Severity policy for when alerts
land; until then, operators use the runbook triage table.

| Alert ID | Condition (stub) | Severity | Route (stub) | Runbook |
|----------|------------------|----------|--------------|---------|
| `SL-READYZ-DOWN` | `/readyz` ≠ 200 for &gt; 2m while process up | P2 | Slack `#sessionledger-ops` (TBD) | [`runbook.md`](runbook.md) — Health check |
| `SL-HEALTHZ-DOWN` | `/healthz` unreachable for &gt; 1m | P1 | PagerDuty service TBD | [`runbook.md`](runbook.md) — Common failures |
| `SL-INGEST-ERROR-BUDGET` | Ingest error rate &gt; 5% over 15m | P2 | Slack (TBD) | friction-log + validate OKF |
| `SL-REPLAY-LATENCY` | Replay TTFB p95 &gt; 2s (fixtures) | P3 | None (friction-log) | Manual fixture replay |
| `SL-METRICS-STALE` | `/api/metrics` 5xx or timeout &gt; 5m | P3 | Slack (TBD) | [`runbook.md`](runbook.md) — Metrics |

See also [`alerts.md`](alerts.md) for copy-paste stub definitions.

## Intended OpenTelemetry (soft goal — issue #65)

Tracked under [issue #65](https://github.com/KooshaPari/SessionLedger/issues/65)
(v38 P1: OTel + `/readyz` + SLO stubs). Docs/SLO/readyz portions of that issue
are addressed here; **remaining code work**:

| #65 item | Status in-tree | PLAN / notes |
|----------|----------------|--------------|
| `GET /readyz` distinct from liveness | **Done** (daemon + process-compose) | Documented above |
| SLO / error-budget stubs in this doc | **Done** (stubs) | This file |
| `tracing` subscriber + env log discipline | **Not done** | `T-021`, `T-022` |
| Soft-goal OTLP export sketch | **Doc only** | `T-023`; SDK/exporter still absent |
| W3C `traceparent` propagation | **Not done** | `T-034` / TraceSink adapters |
| Prometheus / OTLP RED exporters | **Not done** | Parallel to `/api/metrics` |

Future implementation sketch (PLAN `T-023` / `T-021` / `T-034`):

1. `tracing` subscriber in `sl-daemon` with request/span IDs on HTTP + ETL stages.
2. Optional OTLP export behind a feature flag (no hard dependency on collector).
3. Propagate W3C `traceparent` across ingest → compile → export when adapters land.
4. Emit RED counters/histograms (table above) via OTLP or a dedicated scrape path —
   without breaking the current `/api/metrics` JSON summary.

Until then, operators rely on `/healthz`, `/readyz`, `/api/metrics`, and process logs.

## Log level discipline

| Level | Use for |
|-------|---------|
| `error` | Failed ingest, bind failures, unrecoverable ETL errors |
| `warn` | Skipped malformed bundles, archive dry-run anomalies |
| `info` | Startup banner, listen address, successful archive/restore counts |
| `debug` | Per-file watch events, filter match details |
| `trace` | Payload dumps — **never** default in production |

Preferred env (once `tracing` lands): `RUST_LOG=sl_daemon=info,tower_http=warn`.
Avoid logging full session transcripts at `info` or above (PII / token bloat).

## FR mapping

- FR-005 — metrics aggregation
- FR-014 — healthz + readyz + local ops
- FR-015 — observability stubs (this doc)
