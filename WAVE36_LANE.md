# Wave-36 lane: w36-otlp-hard — C05 L43 production OTLP metrics push

**Branch:** `feat/sl-w36-otlp-hard`  
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w36-otlp-hard`  
**Cluster / pillar:** C05 L43/L42 (OTLP metrics export)  
**Status:** scoped — implementation deferred  
**Wave-35 overlap:** none (#289 profile `http_soft` is continuous profiling L45, not OTLP metrics)

## Gap (audit-v38)

Wave-33 #276 landed **soft** OTLP metrics stub (`otel-metrics` feature,
`continue-on-error` in `ops-load.yml`). SCORECARD: *production OTLP metrics push*
and USE process gauges remain unpaid. L43 already pillar max — score held until
production push is blocking.

## Acceptance criteria

1. Promote `crates/sl-daemon/src/otel_metrics.rs` from ack stub to real OTLP/gRPC
   metrics export when `SL_OTLP_METRICS_ENDPOINT` / `OTEL_EXPORTER_OTLP_ENDPOINT`
   is set.
2. Update `docs/ops/otlp-metrics.md` + `docs/ops/otlp-metrics.json` SSOT.
3. Extend `scripts/otlp-metrics-check.ps1 -SelfCheck` for export-path anchors.
4. Add blocking or opt-in CI job (hermetic mock collector or documented skip).
5. **Do not edit** `audit/SCORECARD.md`.

## Files to touch (exclusive)

- `crates/sl-daemon/src/otel_metrics.rs`, `crates/sl-daemon/Cargo.toml`
- `docs/ops/otlp-metrics.md`, `docs/ops/otlp-metrics.json`, `docs/ops/observability.md`
- `scripts/otlp-metrics-check.ps1`, `tests/otlp_metrics.rs`
- `.github/workflows/ops-load.yml`
- `CHANGELOG.md`

## Verify

```powershell
$env:CARGO_TARGET_DIR = "C:\Users\koosh\SessionLedger-wtrees\w36-otlp-hard\target-w36-otlp-hard"
pwsh ./scripts/otlp-metrics-check.ps1 -SelfCheck
cargo test -p sl-daemon otlp_metrics
```

## Score expectation

Evidence only unless maintainer promotes to blocking export; L43 max held.
