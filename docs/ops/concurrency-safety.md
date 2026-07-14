# Concurrency safety (L7 evidence)

SessionLedger's Concurrency Safety lane starts with **deterministic race smokes**
plus a **loom-lite bounded-channel / cancel model**. This is score-1 evidence:
it proves merge/OKF outputs stay deterministic under threads and that the
daemon's watcher contract (bounded queue + cooperative cancel) conserves
messages without CI flakes.

Full `loom` / `shuttle` permutation checkers and nightly Miri remain follow-ups
(`unsafe_code = forbid` today, so Miri is deferred until any `unsafe` lands).

## What runs in CI

| Artifact | Role |
|----------|------|
| [`tests/race_smoke.rs`](../../tests/race_smoke.rs) | Threaded merge + OKF determinism across shuffled inputs |
| [`tests/race_model.rs`](../../tests/race_model.rs) | Bounded `sync_channel` + cancel flag model of watcher `scan_once` |
| [`.github/workflows/race-smoke.yml`](../../.github/workflows/race-smoke.yml) | Both tests, 3 OS × 3 repeats, `--test-threads=1` |

The model uses `try_send` (never blocks) and an `AtomicBool` cancel bit so
assertions are conservation / capacity based — no sleeps, no OS event timing.

Daemon unit coverage for the same contract lives in
`crates/sl-daemon/src/watcher.rs` (`scan_once_stops_when_cancelled_*`).

## How to run locally

Prefer a worktree-local Cargo target dir so parallel agents do not collide:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $PWD "target-w24-c00-loom"
cargo test --test race_smoke --test race_model --locked -- --test-threads=1
```

Repeat a few times when validating a concurrency change:

```powershell
1..3 | ForEach-Object {
  Write-Host "race model repeat $_/3"
  cargo test --test race_model --locked -- --test-threads=1
}
```

## Loom follow-up

A `#[cfg(loom)]` permutation job is intentionally not wired yet: loom replaces
`std` concurrency primitives and needs a dedicated `RUSTFLAGS='--cfg loom'`
lane. When added, keep it off the default PR matrix so ordinary
`cargo test` stays green without special flags.
