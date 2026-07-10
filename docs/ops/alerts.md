# Alert stubs — SessionLedger (C05)

**Status:** placeholders only. No Alertmanager / PagerDuty / Grafana rules are
shipped. Wire exporters first ([issue #65](https://github.com/KooshaPari/SessionLedger/issues/65)
OTLP + RED metrics), then promote these stubs to real rules.

Canonical severity + routing table: [`observability.md`](observability.md#alert-stubs).
Operator triage: [`runbook.md`](runbook.md).

## Stub rule sketches

### `SL-HEALTHZ-DOWN` (P1)

```yaml
# STUB — not loaded by any system
alert: SL-HEALTHZ-DOWN
expr: up{job="sl-daemon"} == 0 or probe_success{path="/healthz"} == 0
for: 1m
labels:
  severity: page
annotations:
  summary: "sl-daemon liveness failed"
  runbook: docs/ops/runbook.md#health-check
```

### `SL-READYZ-DOWN` (P2)

```yaml
# STUB — not loaded by any system
alert: SL-READYZ-DOWN
expr: probe_success{path="/readyz"} == 0 and probe_success{path="/healthz"} == 1
for: 2m
labels:
  severity: ticket
annotations:
  summary: "sl-daemon alive but not ready (check out_dir / SL_DATA_DIR)"
  runbook: docs/ops/runbook.md#health-check
```

### `SL-INGEST-ERROR-BUDGET` (P2)

```yaml
# STUB — requires future RED counter sl_ingest_errors_total
alert: SL-INGEST-ERROR-BUDGET
expr: |
  rate(sl_ingest_errors_total[15m])
    / rate(sl_ingest_requests_total[15m]) > 0.05
for: 15m
labels:
  severity: ticket
annotations:
  summary: "ingest error rate exceeded 5% SLO stub"
  runbook: docs/ops/observability.md#slo-stubs-intentional-placeholders
```

### `SL-REPLAY-LATENCY` (P3)

```yaml
# STUB — requires future histogram sl_replay_ttfb_seconds
alert: SL-REPLAY-LATENCY
expr: histogram_quantile(0.95, rate(sl_replay_ttfb_seconds_bucket[30m])) > 2
for: 30m
labels:
  severity: info
annotations:
  summary: "replay TTFB p95 above 2s fixture SLO stub"
```

### `SL-METRICS-STALE` (P3)

```yaml
# STUB
alert: SL-METRICS-STALE
expr: probe_success{path="/api/metrics"} == 0
for: 5m
labels:
  severity: info
annotations:
  summary: "/api/metrics unavailable while daemon expected up"
  runbook: docs/ops/runbook.md#metrics
```

## Promotion checklist (when OTLP lands)

1. Emit RED names from [`observability.md`](observability.md#red-metrics-mapping).
2. Replace `probe_*` stubs with real blackbox or in-process metrics.
3. Fill Slack / PagerDuty route IDs in observability alert table.
4. Close remaining #65 exporter items; keep these files as the source of truth
   for rule intent.
