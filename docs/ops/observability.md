# Observability — SessionLedger

Current HTTP surfaces and intended future instrumentation. Soft goals are
explicitly non-blocking for P0 product work.

## Current surfaces

| Surface | Path | Purpose |
|---------|------|---------|
| Liveness | `GET /healthz` | Returns `200` + body `ok`. Used by process-compose readiness. |
| Metrics | `GET /api/metrics` | Aggregated bundle stats: totals, avg tokens, model + daily histograms (`crates/sl-daemon/src/metrics.rs`). |
| Live events | `GET /api/stream` | SSE of newly written `*.okf.json` paths (viewer LiveFeed). |
| Replay | `GET /api/replay/:id` | SSE entity playback (not ops metrics; product replay). |

Default bind: port **8080** (`SL_PORT`). See [`runbook.md`](runbook.md).

There is **no** Prometheus scrape endpoint or OTLP exporter in-tree yet.
`TraceSink` is a design port ([`docs/DESIGN.md`](../DESIGN.md) §6) composed with
external Phenotype observability systems.

## Intended OpenTelemetry (soft goal)

Future work (PLAN `T-023` / `T-021` / `T-034`):

1. `tracing` subscriber in `sl-daemon` with request/span IDs on HTTP + ETL stages.
2. Optional OTLP export behind a feature flag (no hard dependency on collector).
3. Propagate W3C `traceparent` across ingest → compile → export when adapters land.
4. Map RED-ish counters (ingest success/fail, compile latency) onto `/api/metrics`
   or a dedicated scrape path — without breaking the current JSON summary.

Until then, operators rely on `/healthz`, `/api/metrics`, and process logs.

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
- FR-014 — healthz + local ops
- FR-015 — observability stubs (this doc)
