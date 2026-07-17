# Wave-38 lane: w38-use-gauges — C05 L42 USE process gauges

**Branch:** `feat/sl-w38-use-gauges`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w38-use-gauges`
**Cluster / pillar:** C05 L42 (process USE gauges on Prometheus `/metrics`)

## Gap

Process USE gauges (CPU, RSS, open FDs) were soft goals alongside OTLP metrics.
Prometheus `/metrics` exported RED only; operators could not scrape basic process
utilization from the default path.

## Acceptance criteria

1. `crates/sl-daemon/src/metrics.rs` — `append_process_use_gauges` emits
   `process_cpu_seconds_total`, `process_resident_memory_bytes`, and
   `process_open_fds` from `HttpMetrics::render_prometheus()` (Linux via
   `/proc/self`; Windows/macOS stub 0 with comment).
2. `docs/ops/observability.md` — RED/USE section documents the three series.
3. `scripts/use-gauges-check.ps1 -SelfCheck` — hermetic doc + code anchors.
4. `tests/use_gauges.rs` — SelfCheck wrapper at repo root.
5. Unit test in `metrics.rs` asserting `process_resident_memory_bytes` in output.
6. CHANGELOG bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/use-gauges-check.ps1 -SelfCheck
cargo test -p sl-daemon prometheus_snapshot_contains_process_use_gauges --manifest-path crates/sl-daemon/Cargo.toml
cargo test use_gauges_doc_self_check_validates_anchors
```

## Score expectation

Evidence toward L42 USE gauges on default Prometheus scrape; OTLP mirror remains unpaid.
