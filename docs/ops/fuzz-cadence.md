# Fuzz cadence (C07 L67)

SessionLedger ships two `cargo-fuzz` targets with a **seeded corpus** and a
**blocking PR smoke** (`ci.yml` → `fuzz-smoke`, 10 seconds per target). This
page is the SSOT for the **sustained / soft longer cadence** beyond that smoke:
nightly campaigns, crash artifact triage, and how to keep PR CI fast.

Related: [`test-pyramid.md`](test-pyramid.md) (pyramid layer),
[`fuzz/`](../../fuzz/), [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
(`fuzz-smoke`), [`.github/workflows/fuzz-cadence.yml`](../../.github/workflows/fuzz-cadence.yml).

## Cadence map

| Lane | Duration | When | Gate |
|------|----------|------|------|
| PR smoke | 10 s / target | every PR / push via `ci.yml` → `fuzz-smoke` | **blocking** |
| Sustained soft | 120 s / target | nightly schedule + `workflow_dispatch` | **soft** (`continue-on-error`) |
| Local campaign | operator-chosen | maintainer machine | manual |

PR smoke stays short on purpose. Sustained runs **do not** join the default PR
required-check surface — they live in `fuzz-cadence.yml` and stay soft so a
flaky libFuzzer campaign cannot block merges.

## Targets and corpora

| Target | Seed corpus | Exercises |
|--------|-------------|-----------|
| `okf_roundtrip` | `fuzz/corpus/okf_roundtrip/` | OKF parse + roundtrip invariants |
| `jsonl_ingest` | `fuzz/corpus/jsonl_ingest/` | JSONL ingest parse paths |

## Soft sustained workflow

[`fuzz-cadence.yml`](../../.github/workflows/fuzz-cadence.yml) is **non-blocking**
(`continue-on-error: true`). It:

1. Runs `scripts/fuzz-cadence-check.ps1 -SelfCheck` (docs/workflow/path anchors).
2. On schedule / `workflow_dispatch` only (skipped on `pull_request`): runs each
   target for `-max_total_time=120` with ASAN on `x86_64-unknown-linux-gnu`
   (same toolchain pins as `fuzz-smoke`).
3. On failure, uploads `fuzz/artifacts/` for crash corpus triage.

Schedule: nightly UTC (offset from miri/loom soft jobs) + `workflow_dispatch`.
`pull_request` only exercises the hermetic SelfCheck job so default PR CI is
not lengthened by the 120 s campaigns.

## Crash corpus triage

When a sustained (or local) run finds a crash, libFuzzer writes under
`fuzz/artifacts/<target>/` (for example `crash-*`). Triage steps:

1. Download the workflow artifact `fuzz-crash-artifacts` (or copy the local
   `fuzz/artifacts/` tree).
2. Reproduce with the failing input:
   `cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash-file>`.
3. Minimize when useful:
   `cargo +nightly fuzz tmin <target> fuzz/artifacts/<target>/<crash-file>`.
4. Reduce to a focused regression (unit/property test or a small corpus seed
   under `fuzz/corpus/<target>/`) and open a fix PR.
5. Do **not** commit raw unbounded crash dumps or corpus growth from CI without
   review — keep seeds small and intentional.

## Done gates

| Gate | Status | Evidence |
|------|--------|----------|
| Fuzz cadence SelfCheck | **done** | `scripts/fuzz-cadence-check.ps1 -SelfCheck` (+ `tests/fuzz_cadence.rs`) |
| Soft sustained fuzz CI | **done** | `.github/workflows/fuzz-cadence.yml` (`continue-on-error`, 120 s / target) |
| PR `fuzz-smoke` (10 s) | **done** | `.github/workflows/ci.yml` (unchanged; stays blocking + short) |
| Auto corpus promotion from CI crashes | **unpaid** | Triage remains maintainer-driven (see above) |

## Machine verification (SelfCheck)

Hermetic docs + path + workflow anchors (no `cargo fuzz`, no network):

```powershell
pwsh ./scripts/fuzz-cadence-check.ps1 -SelfCheck
```

## Local sustained campaign

Nightly toolchain + `cargo-fuzz` (same flags as soft CI):

```powershell
$env:CARGO_TARGET_DIR = Join-Path $PWD "target-w32-c07-fuzz"
cargo +nightly fuzz run okf_roundtrip --sanitizer address --target x86_64-unknown-linux-gnu -- -max_total_time=120
cargo +nightly fuzz run jsonl_ingest --sanitizer address --target x86_64-unknown-linux-gnu -- -max_total_time=120
```

On Windows hosts without ASAN/gnu, drop `--sanitizer` / `--target` and use a
shorter local time budget; prefer Linux (or the soft CI job) for ASAN campaigns.
