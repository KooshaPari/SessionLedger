# Observability — SessionLedger

Current HTTP surfaces, SLO/error-budget stubs, RED metric mapping, and
intended future instrumentation. Soft goals are explicitly non-blocking for
P0 product work. Remaining deep-obs work tracks [issue #65](https://github.com/KooshaPari/SessionLedger/issues/65).

## Current surfaces

| Surface | Path | Purpose |
|---------|------|---------|
| Liveness | `GET /healthz` | Returns `200` + body `ok`. Process is up. |
| Readiness | `GET /readyz` | Returns `200` + `ready` when `out_dir` exists and optional `SL_MEMORY_DB` passes a probe; else `503`. Used by process-compose. |
| Metrics | `GET /api/metrics` | Aggregated bundle stats: totals, avg tokens, model + daily histograms (`crates/sl-daemon/src/metrics.rs`). |
| Prometheus RED metrics | `GET /metrics` | Process-local request count, HTTP errors, and request-duration sum/count. |
| Local pprof debug | `GET /debug/pprof/*` | Optional loopback-only pprof-style surface when `SL_ENABLE_PPROF=1`. Disabled by default. |
| Live events | `GET /api/stream` | SSE of newly written `*.okf.json` paths (viewer LiveFeed). |
| Replay | `GET /api/replay/:id` | SSE entity playback (not ops metrics; product replay). |

Default bind: port **8080** (`SL_PORT`). See [`runbook.md`](runbook.md).

Scheduled evidence:

| Workflow | Cadence | What it proves |
|----------|---------|----------------|
| [`.github/workflows/ops-chaos-smoke.yml`](../../.github/workflows/ops-chaos-smoke.yml) | Weekdays 06:23 UTC + `workflow_dispatch` | Short ops/chaos smoke via [`scripts/ops-chaos-smoke.ps1`](../../scripts/ops-chaos-smoke.ps1): `/healthz` vs `/readyz` separation, metrics shape checks, light load burst, process-kill recovery. Smoke phases target **&lt;2 min** once the daemon binary is built. |
| [`.github/workflows/ops-load.yml`](../../.github/workflows/ops-load.yml) | Weekly + `workflow_dispatch` | Heavier concurrent load against `/healthz`, `/readyz`, `/api/metrics`, and `/metrics` via [`scripts/load-smoke.ps1`](../../scripts/load-smoke.ps1). |
| [`.github/workflows/ops-gameday.yml`](../../.github/workflows/ops-gameday.yml) | Quarterly (manual) + `workflow_dispatch` | Game-day evidence pass: same short chaos smoke with `-EvidencePath` → `gameday-evidence.json` artifact. See [Game-day cadence](#game-day-cadence). |

Prometheus SLO alert rules live in
[`alerts/sessionledger-slo.yaml`](alerts/sessionledger-slo.yaml) and are meant
to be loaded with Prometheus `rule_files`. The Alertmanager routing placeholder
lives in [`alerts/alertmanager.yaml`](alerts/alertmanager.yaml).

### `/healthz` vs `/readyz`

| Probe | Question it answers | Success | Failure | Who should call it |
|-------|---------------------|---------|---------|-------------------|
| `/healthz` | Is the HTTP process alive? | `200` + `ok` | Connection refused / timeout | Process supervisors, `sl status`, crude uptime checks |
| `/readyz` | Can the daemon serve work that needs `out_dir` (and optional memory DB)? | `200` + `ready` when `out_dir` exists and is a directory, and `SL_MEMORY_DB` answers a probe when configured | `503` when `out_dir` is missing or memory DB probe fails | `process-compose` readiness probe, load balancers, viewer `depends_on` |

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
| Metrics endpoint availability | `/api/metrics` and `/metrics` return `200` | ≥99% when daemon ready | Per session | curl / Prometheus scrape |

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

The lightweight `/metrics` endpoint requires no collector or extra feature and
exports process-lifetime HTTP RED counters. A feature-gated OTLP trace export
sketch is also available as described below. `TraceSink` is a design port
([`docs/DESIGN.md`](../DESIGN.md) §6) composed with external Phenotype
observability systems.

## RED metrics mapping

[RED](https://www.weave.works/blog/the-red-method-key-metrics-for-microservices-architecture/)
(Rate, Errors, Duration) mapped to SessionLedger surfaces.

| RED | Meaning | Current signal | Metric | Export path |
|-----|---------|----------------|--------|-------------|
| **R**ate | HTTP request volume | Process-local counter | `sl_http_requests_total` | `/metrics` |
| **R**ate | Replay / stream consumers | SSE connect count (not counted) | `sl_sse_clients` | OTLP up-down counter |
| **E**rrors | HTTP 4xx–5xx responses | Process-local counter | `sl_http_errors_total` | `/metrics` |
| **E**rrors | Readiness failures | `/readyz` → `503` | `sl_readyz_failures_total` | OTLP counter |
| **D**uration | HTTP request time | Process-local summary + per-route histogram buckets | `sl_http_request_duration_seconds` | `/metrics` |
| **D**uration | Ingest → compile → write | Not timed | `sl_ingest_duration_seconds` | OTLP histogram (future) |
| **D**uration | Replay time-to-first-byte | Manual | `sl_replay_ttfb_seconds` | OTLP histogram |
| **D**uration | `/api/metrics` compute | Not timed | `sl_metrics_handler_duration_seconds` | OTLP histogram |

App-level JSON (`/api/metrics`) stays the product summary. RED exporters must
**not** break that contract — add parallel scrape/OTLP paths instead.

## Alert stubs

Severity policy for when alerts land. **Promoted** rules use shipped RED or
scrape metrics and load from [`alerts/sessionledger-slo.yaml`](alerts/sessionledger-slo.yaml).
**Stub** rules need ingest/replay counters or blackbox probes — see
[`alerts.md`](alerts.md).

| Alert ID | Condition | Severity | Route (evidence) | Status | Runbook |
|----------|-----------|----------|------------------|--------|---------|
| `SessionLedgerDaemonScrapeDown` | `up{job="sl-daemon"} == 0` for 2m | P1 / `warning` | Alertmanager → `sessionledger-webhook-placeholder` | **Promoted** | [`runbook.md`](runbook.md) — Common failures |
| `SessionLedgerDaemonScrapeMissing` | `absent(up{job="sl-daemon"})` for 5m | P2 / `warning` | Alertmanager placeholder | **Promoted** | [`dashboards/README.md`](dashboards/README.md) |
| `SessionLedgerFastErrorBudgetBurn` | HTTP error ratio &gt; 5% / 5m | P2 / `warning` | Alertmanager placeholder | **Promoted** | [`runbook.md`](runbook.md) — Common failures |
| `SessionLedgerSlowErrorBudgetBurn` | HTTP error ratio &gt; 1% / 1h | P3 / `info` | Alertmanager placeholder | **Promoted** | friction-log |
| `SessionLedgerHighMeanLatency` | mean HTTP latency &gt; 1s / 10m | P3 / `info` | Alertmanager placeholder | **Promoted** | [`runbook.md`](runbook.md) — Common failures |
| `SessionLedgerRedMetricsMissing` | scrape up but no `sl_http_requests_total` | P2 / `warning` | Alertmanager placeholder | **Promoted** | [`runbook.md`](runbook.md) — Metrics |
| `SL-READYZ-DOWN` | `/readyz` ≠ 200 while process up | P2 | Slack `#sessionledger-ops` (TBD) | **Stub** (no probe metric) | [`runbook.md`](runbook.md) — Health check |
| `SL-HEALTHZ-DOWN` | `/healthz` unreachable | P1 | PagerDuty service TBD | **Stub** (use scrape-down) | [`runbook.md`](runbook.md) — Common failures |
| `SL-INGEST-ERROR-BUDGET` | Ingest error rate &gt; 5% / 15m | P2 | Slack (TBD) | **Stub** (no ingest counters) | friction-log + validate OKF |
| `SL-REPLAY-LATENCY` | Replay TTFB p95 &gt; 2s | P3 | None (friction-log) | **Stub** (no histogram) | Manual fixture replay |
| `SL-METRICS-STALE` | `/metrics` unavailable &gt; 5m | P3 | Slack (TBD) | **Promoted** (scrape `up`) | [`runbook.md`](runbook.md) — Metrics |

Liveness/readiness separation is proven by [`ops-chaos-smoke.ps1`](../../scripts/ops-chaos-smoke.ps1)
and quarterly [`gameday-evidence.json`](fixtures/gameday-evidence.sample.json)
until blackbox `probe_*` metrics exist.

See also [`alerts.md`](alerts.md) for copy-paste stub definitions and
[`alerts/sessionledger-slo.yaml`](alerts/sessionledger-slo.yaml) for
Prometheus-loadable SLO alert rules.

### Prometheus and Alertmanager loading

Load the SLO rules from `prometheus.yml`:

```yaml
rule_files:
  - /etc/prometheus/rules/sessionledger-slo.yaml
```

Point Prometheus at Alertmanager if you want the rules to route beyond the
Prometheus UI:

```yaml
alerting:
  alertmanagers:
    - static_configs:
        - targets:
            - alertmanager:9093
```

Start Alertmanager with the SessionLedger placeholder config after replacing
the webhook URL with an operator-owned endpoint:

```bash
alertmanager --config.file=/etc/alertmanager/alertmanager.yaml
```

The in-tree placeholder routes `job="sl-daemon"` alerts to
`sessionledger-webhook-placeholder`; it is evidence wiring, not a production
receiver. Runnable PromQL and the severity→receiver mapping table live in
[`alerts.md`](alerts.md#alert-routing-evidence).

## Game-day cadence

Automated weekday chaos smoke (#184) and weekly load smoke cover steady-state
regression. **Game-day** is a quarterly, operator-led pass that proves alert
routing intent against shipped RED metrics without a long multi-host soak.

### Calendar

| Cadence | When | Workflow / script | Artifact |
|---------|------|-------------------|----------|
| Weekday smoke | Mon–Fri 06:23 UTC | [`ops-chaos-smoke.yml`](../../.github/workflows/ops-chaos-smoke.yml) | CI job log |
| Weekly load | Mon 09:17 UTC | [`ops-load.yml`](../../.github/workflows/ops-load.yml) | CI job log |
| **Quarterly game-day** | First **Wednesday** of Jan, Apr, Jul, Oct (local ops TZ) | [`ops-gameday.yml`](../../.github/workflows/ops-gameday.yml) or local script below | `gameday-evidence.json` |

Quarterly game-day does **not** add a cron — dispatch the workflow or run the
script locally and archive the JSON with the friction log for the quarter.

### Game-day checklist

Run on a clean branch at or after the quarterly date. Target wall time **&lt;5 min**
(build + short chaos smoke).

| Step | Action | Pass criteria |
|------|--------|---------------|
| 1. Dispatch evidence | GitHub **Actions → Ops Game Day Evidence → Run workflow**, or locally: `pwsh ./scripts/ops-chaos-smoke.ps1 -DaemonPath <sl-daemon> -EvidencePath docs/ops/fixtures/gameday-evidence.json` after `cargo build -p sl-daemon` | Exit code 0; `outcome: pass` in JSON |
| 2. Validate RED snapshot | Open uploaded `gameday-evidence.json` (sample: [`fixtures/gameday-evidence.sample.json`](fixtures/gameday-evidence.sample.json)) | `redMetrics` shows aggregate counters; `route_labels` / `histogram_buckets` true after load phases |
| 3. Load Prometheus rules | `promtool check rules docs/ops/alerts/sessionledger-slo.yaml` (or Prometheus UI → Status → Rules) | Rules compile; `SessionLedger*` alerts present |
| 4. Exercise PromQL | Paste aggregate queries from [`alerts.md`](alerts.md#promql-for-shipped-red-metrics) into Prometheus against a scraped `job="sl-daemon"` target | Queries return data (or `NaN` when idle — not an error) |
| 5. Review routing placeholder | Inspect [`alerts/alertmanager.yaml`](alerts/alertmanager.yaml) matchers vs [`alerts.md` routing table](alerts.md#alert-routing-evidence) | `job="sl-daemon"` routes to placeholder receiver; severity labels documented |
| 6. Manual triage drill | Pick one promoted alert (e.g. `SessionLedgerFastErrorBudgetBurn`); walk [`runbook.md`](runbook.md#common-failures) | Operator can reach runbook anchor without a live page |
| 7. Archive | Store JSON + friction-log note in release/ops notes | Quarter labeled (e.g. `2026-Q3-gameday`) |

Multi-host steady-state hypothesis remains future work (issue #65); game-day
evidence documents single-replica chaos + alert **intent**, not production paging.

## OpenTelemetry (feature-gated sketch — issue #65)

Tracked under [issue #65](https://github.com/KooshaPari/SessionLedger/issues/65)
(v38 P1: OTel + `/readyz` + SLO stubs). Docs/SLO/readyz portions of that issue
are addressed here; **remaining code work**:

| #65 item | Status in-tree | PLAN / notes |
|----------|----------------|--------------|
| `GET /readyz` distinct from liveness | **Done** (daemon + process-compose) | Documented above |
| SLO / error-budget stubs in this doc | **Done** (stubs) | This file |
| `tracing` subscriber + env log discipline | **Done** | fmt subscriber + `RUST_LOG` |
| Optional production JSON logs | **Done** | `json-logs` feature + `SL_LOG_FORMAT=json` |
| Soft-goal OTLP export sketch | **Feature-gated sketch** | `otel` Cargo feature; traces only |
| W3C `traceparent` propagation | **Done for HTTP ingress** | Valid v00 context parsed, set as OTel parent when `otel` is enabled, echoed on response |
| Prometheus / OTLP RED exporters | **Prometheus HTTP RED subset done** | `/metrics`, parallel to `/api/metrics` |

Build the optional exporter without changing the default dependency graph:

```bash
cargo build -p sl-daemon --features otel
```

With that feature enabled, set `SL_OTLP_ENDPOINT` to the collector's OTLP/gRPC
endpoint (for example, `http://localhost:4317`). The standard
`OTEL_EXPORTER_OTLP_ENDPOINT` is used as a fallback when the SessionLedger
variable is absent. `SL_OTLP_ENDPOINT` takes precedence when both are set. If
neither variable is set, the daemon keeps its normal fmt subscriber and
`RUST_LOG` filtering and does not create an exporter or require a collector.

Remaining future work:

1. Continue W3C context across ingest → compile → export when adapters land.
2. Bridge labeled RED signals to OTLP and add process gauges.

Operators without the `otel` feature continue to rely on `/healthz`, `/readyz`,
`/api/metrics`, `/metrics`, and process logs.

## Local pprof-style profiling

Continuous profiling is intentionally local-only and off by default. The daemon's
HTTP bind parser still rejects non-loopback addresses, and the debug routes are
only registered when explicitly enabled:

```bash
SL_ENABLE_PPROF=1 sl serve --http-bind 127.0.0.1:8080
```

Available debug paths:

| Path | Status | Notes |
|------|--------|-------|
| `GET /debug/pprof/cmdline` | `200` | Returns null-delimited process argv bytes, matching the pprof-style cmdline surface. |
| `GET /debug/pprof/profile` | `501` | CPU sampling is not implemented in the default cross-platform build. The endpoint exists as the gated profiling surface and returns explanatory bytes. |

The default build avoids pulling in a sampler that would make Windows + Linux CI
fragile. If deeper profiling becomes a hard requirement, prefer adding a
feature-gated sampler (`pprof` on supported targets, `jemalloc_pprof` for
allocator profiles, or `tokio-console` for async task diagnostics) without
changing the default dependency graph.

## HTTP trace-context sketch

Every HTTP route accepts a W3C `traceparent` header in the common
`00-<trace-id>-<parent-id>-<flags>` form. Valid lowercase contexts are attached
to the `http.request` tracing span as `trace_id`, `parent_span_id`, and
`trace_flags`, then echoed on the response. When the `otel` feature is enabled,
the same parsed context becomes the remote OpenTelemetry parent context for the
request span; `tracestate` is preserved when present and valid. Invalid headers
are ignored. This does not create a replacement trace ID and does not yet
connect context to ETL adapter spans.

## Production log format

The default build and output remain the human-readable fmt subscriber. For
newline-delimited JSON suitable for log collectors:

```bash
cargo build -p sl-daemon --features json-logs
SL_LOG_FORMAT=json RUST_LOG=sl_daemon=info ./target/debug/sl serve ...
```

Setting `SL_LOG_FORMAT=json` without the `json-logs` feature has no effect.
The feature composes with `otel` (`--features json-logs,otel`).

## Log level discipline

| Level | Use for |
|-------|---------|
| `error` | Failed ingest, bind failures, unrecoverable ETL errors |
| `warn` | Skipped malformed bundles, archive dry-run anomalies |
| `info` | Startup banner, listen address, successful archive/restore counts |
| `debug` | Per-file watch events, filter match details |
| `trace` | Payload dumps — **never** default in production |

Preferred env: `RUST_LOG=sl_daemon=info,tower_http=warn`.
Avoid logging full session transcripts at `info` or above (PII / token bloat).

## FR mapping

- FR-005 — metrics aggregation
- FR-014 — healthz + readyz + local ops
- FR-015 — observability stubs (this doc)
