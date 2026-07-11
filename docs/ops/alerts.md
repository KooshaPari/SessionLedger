# Alert queries — SessionLedger (C05)

**Status:** copy-paste PromQL examples; no Alertmanager or pager routing is
enabled by this repository. The daemon exposes aggregate HTTP RED counters at
`/metrics`, and an importable Grafana dashboard is available in
[`dashboards/`](dashboards/). Endpoint-specific ingest and replay signals
remain future work.

Canonical severity + routing table: [`observability.md`](observability.md#alert-stubs).
Operator triage: [`runbook.md`](runbook.md).

## PromQL for shipped RED metrics

The Wave-6 metrics have no `route`, `method`, or `status` labels. These queries
are intentionally service-wide. They aggregate across replicas selected by the
`job="sl-daemon"` label and use `rate`, which safely handles counter resets.

| Signal | PromQL |
|--------|--------|
| Request rate | `sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[5m]))` |
| Error rate | `sum(rate(sl_http_errors_total{job="sl-daemon"}[5m]))` |
| Error ratio | `sum(rate(sl_http_errors_total{job="sl-daemon"}[5m])) / clamp_min(sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[5m])), 1e-9)` |
| Mean duration | `sum(rate(sl_http_request_duration_seconds_sum{job="sl-daemon"}[5m])) / clamp_min(sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[5m])), 1e-9)` |
| Started requests | `sum(rate(sl_http_requests_total{job="sl-daemon"}[5m]))` |

`sl_http_requests_total` increments when a request starts, while
`sl_http_request_duration_seconds_count` increments when it completes. Use the
completed count as the denominator for both error ratio and mean duration.
Their difference can help identify in-flight or interrupted requests, but is
not a stable concurrency gauge.

The exporter exposes only `_sum` and `_count` for duration. Do not use
`histogram_quantile` with this metric: p95/p99 require future `_bucket` series.

## Rules using shipped metrics

These examples can be placed under a Prometheus rule group's `rules` array.
Tune thresholds and `for` durations against observed traffic before paging.

### `SL-HTTP-HIGH-ERROR-RATIO` (P2)

```yaml
alert: SL-HTTP-HIGH-ERROR-RATIO
expr: |
  (
    sum(rate(sl_http_errors_total{job="sl-daemon"}[15m]))
      /
    clamp_min(
      sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[15m])),
      1e-9
    )
  ) > 0.05
  and
  sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[15m])) > 0
for: 15m
labels:
  severity: ticket
annotations:
  summary: "sl-daemon HTTP error ratio exceeded 5%"
  runbook: docs/ops/runbook.md#common-failures
```

### `SL-HTTP-HIGH-MEAN-LATENCY` (P3)

```yaml
alert: SL-HTTP-HIGH-MEAN-LATENCY
expr: |
  (
    sum(rate(sl_http_request_duration_seconds_sum{job="sl-daemon"}[15m]))
      /
    clamp_min(
      sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[15m])),
      1e-9
    )
  ) > 1
  and
  sum(rate(sl_http_request_duration_seconds_count{job="sl-daemon"}[15m])) > 0
for: 15m
labels:
  severity: info
annotations:
  summary: "sl-daemon mean HTTP latency exceeded 1 second"
  runbook: docs/ops/runbook.md#common-failures
```

### `SL-RED-METRICS-MISSING` (P2)

This distinguishes a reachable scrape target from an exporter that no longer
publishes the expected application series.

```yaml
alert: SL-RED-METRICS-MISSING
expr: |
  up{job="sl-daemon"} == 1
  unless
  sl_http_requests_total{job="sl-daemon"}
for: 5m
labels:
  severity: ticket
annotations:
  summary: "sl-daemon scrape succeeds but RED metrics are missing"
  runbook: docs/ops/runbook.md#metrics
```

## Future-signal rule sketches

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
# STUB — requires endpoint-specific ingest counters (not the aggregate HTTP counters)
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
alert: SL-METRICS-STALE
expr: |
  max(up{job="sl-daemon"}) == 0
  or
  absent(up{job="sl-daemon"})
for: 5m
labels:
  severity: info
annotations:
  summary: "Prometheus /metrics unavailable while daemon expected up"
  runbook: docs/ops/runbook.md#metrics
```

## Promotion checklist

1. Import the RED dashboard and validate the queries against each deployed
   replica.
2. Tune aggregate error-ratio and mean-latency thresholds from production
   baselines.
3. Add route-labelled ingest/replay counters before enabling their alert
   stubs.
4. Replace `probe_*` stubs with real blackbox metrics.
5. Fill Slack / PagerDuty route IDs in the observability alert table.
6. Close remaining #65 exporter items; keep these files as the source of truth
   for rule intent.
