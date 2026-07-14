# OTLP metrics export soft stub (L43 / L42 evidence)

SessionLedger ships **Prometheus HTTP RED** metrics at `GET /metrics` with no
collector or Cargo feature required. This document describes the **soft OTLP
metrics export stub**: operator contract, optional `otel-metrics` feature
acknowledgment, and hermetic SelfCheck evidence. It does **not** replace or
break the default Prometheus scrape path.

## What exists today

| Layer | Status | Evidence |
|-------|--------|----------|
| Prometheus HTTP RED (`/metrics`) | **landed** | `crates/sl-daemon/src/metrics.rs`; per-route labels + histogram buckets |
| App JSON aggregates | **landed** | `GET /api/metrics` (product summary; parallel to RED) |
| Feature-gated OTLP **traces** | **landed** | `otel` feature + `SL_OTLP_ENDPOINT` ([`observability.md`](observability.md#opentelemetry-feature-gated-sketch--issue-65)) |
| OTLP **metrics** push | **stub** | `otel-metrics` feature + this doc; `otlp_metrics_export: none` |
| Labeled RED → OTLP bridge | **unpaid** | Future: counters/histograms over OTLP/gRPC |
| Process USE gauges | **unpaid** | Soft goal alongside OTLP metrics |

See also the RED mapping table in
[`observability.md`](observability.md#red-metrics-mapping).

## Operator contract (non-breaking)

1. **Default builds** — only `GET /metrics` (Prometheus text) and `GET /api/metrics`
   (JSON). No OTLP metrics client, no collector required.
2. **Optional feature** — `cargo build -p sl-daemon --features otel-metrics`
   compiles the soft stub module. It does **not** open sockets or alter scrape
   output.
3. **Acknowledgment env** — with the feature enabled, set `SL_OTLP_METRICS=1` to
   log that the unpaid push path is acknowledged. Prometheus `/metrics` stays
   unchanged.
4. **Endpoint reuse (future)** — when push lands, reuse `SL_OTLP_ENDPOINT` /
   `OTEL_EXPORTER_OTLP_ENDPOINT` (same precedence as OTLP traces).

```text
┌─────────────┐  always on          ┌──────────────────┐
│ sl-daemon   │────────────────────►│ Prometheus scrape│
│ GET /metrics│  HTTP RED text      │ (default path)   │
└─────────────┘                     └──────────────────┘
       │
       │  otel-metrics + SL_OTLP_METRICS=1 (stub today)
       ▼
┌──────────────────┐   unpaid  ───► OTLP metrics push
│ acknowledgment   │
│ log only         │
└──────────────────┘
```

## Stub configuration

[`otlp-metrics.json`](otlp-metrics.json) pins the stub knobs:

| Field | Default | Meaning |
|-------|---------|---------|
| `prometheus_http` | `/metrics` | Default RED scrape path (must remain) |
| `otlp_metrics_export` | `none` | Push target — **unpaid** until wired |
| `cargo_feature` | `otel-metrics` | Soft compile-time stub flag |
| `ack_env` | `SL_OTLP_METRICS` | Operator acknowledgment env |

## How to run

### Self-check (hermetic; CI default)

```powershell
pwsh ./scripts/otlp-metrics-check.ps1 -SelfCheck
```

Validates doc/JSON anchors, the `otel-metrics` Cargo feature, and that the
Prometheus `/metrics` renderer remains present.

### Feature-gated stub build (optional)

```bash
cargo build -p sl-daemon --manifest-path crates/sl-daemon/Cargo.toml --features otel-metrics
SL_OTLP_METRICS=1 cargo run -p sl-daemon --manifest-path crates/sl-daemon/Cargo.toml \
  --features otel-metrics -- serve --watch ./sessions --out ./okf-out
```

## CI / scheduling

| Job | Workflow | Blocking? | Notes |
|-----|----------|-----------|-------|
| `otlp-metrics-stub` | [`ops-load.yml`](../../.github/workflows/ops-load.yml) | **soft** (`continue-on-error: true`) | SelfCheck + `cargo check --features otel-metrics` |

## Unpaid gaps (explicit)

1. **OTLP metrics exporter** — no `MetricExporter` / periodic reader wiring yet.
2. **RED bridge** — process-local counters are not mirrored as OTLP instruments.
3. **USE gauges** — CPU/memory/FD gauges remain soft goals.
4. **Exemplars** — not claimed by this stub.

When export lands, set `otlp_metrics_export` in
[`otlp-metrics.json`](otlp-metrics.json) to the collector protocol (e.g.
`otlp-grpc`) and extend `crates/sl-daemon/src/otel_metrics.rs` with a real
exporter. Until then, Prometheus `/metrics` remains the supported scrape path.
