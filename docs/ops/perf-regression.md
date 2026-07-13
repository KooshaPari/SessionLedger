# Pipeline Performance Regression Gate

SessionLedger gates the pipeline Criterion suite in `benches/pipeline.rs` against
the checked-in baseline at `docs/ops/perf-baseline.json`.

## What The Gate Measures

The gate runs:

```powershell
./scripts/bench-gate.ps1
```

The script executes:

```powershell
cargo bench --locked --bench pipeline -- --save-baseline current
```

For CI speed, `scripts/bench-gate.ps1` sets the Criterion sample size to 10 with
1 second of warm-up and 2 seconds of measurement per benchmark. That keeps the
gate suitable for pull requests while still exercising the existing Criterion
benchmark code. The measured operations are:

- `pipeline/distill_compile_200_messages`
- `pipeline/okf_export_200_messages`
- `pipeline/inject_render_200_messages`

## Failure Policy

The gate compares Criterion's saved `mean.point_estimate` values from the
`current` baseline to the checked-in `mean_ns` values. A benchmark fails when it
is more than `policy.threshold_percent` slower than the baseline. The current
threshold is 25%.

A failure should be treated as a regression until proven otherwise. Re-run the
gate on a quiet machine before changing the baseline.

## Refreshing The Baseline

Refresh the baseline only when the pipeline intentionally becomes slower, the
benchmark workload changes, or repeated runs show that the existing baseline is
stale for the supported CI environment. Run `./scripts/eval-repro-check.ps1`
first so lockfile and fixture anchors still match [`eval-manifest.json`](eval-manifest.json).

Run:

```powershell
./scripts/bench-gate.ps1 -UpdateBaseline
```

Review the printed Criterion output, inspect the JSON diff in
`docs/ops/perf-baseline.json`, and commit the baseline update with the code or
benchmark change that justified it. Do not refresh the baseline just to hide an
unexplained pull-request regression.
