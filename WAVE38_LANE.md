# Wave-38 lane: w38-tsan-hard — C00 L7 blocking TSan permutation

**Branch:** `feat/sl-w38-tsan-hard`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w38-tsan-hard`
**Cluster / pillar:** C00 L7
**Wave-37 overlap:** blocking loom/shuttle/Miri permutation CI (#296/#303/#304); TSan unpaid

## Gap

Post-W37 SCORECARD: *TSan + daemon-graph permutation ports* unpaid. Add blocking
TSan permutation CI on the pure-`std` `tests/race_model.rs` subset (ubuntu
x86_64, nightly `-Zsanitizer=thread` + `rust-src` / `-Zbuild-std`) mirroring
`miri-permutation.yml` / `shuttle-permutation.yml` hermetic SelfCheck wiring.

## Acceptance criteria

1. Add `scripts/tsan-permutation-check.ps1 -SelfCheck` with done/unpaid rows.
2. Add `tests/tsan_permutation.rs` hermetic wrapper.
3. Add blocking `.github/workflows/tsan-permutation.yml` (SelfCheck + TSan `race_model`).
4. Update `docs/ops/concurrency-safety.md` TSan permutation section + CI table.
5. CHANGELOG Unreleased bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/tsan-permutation-check.ps1 -SelfCheck
cargo test tsan_permutation
```

Blocking TSan suite (Linux nightly + rust-src only):

```powershell
$env:CARGO_TARGET_DIR = Join-Path $PWD "target-w38-c00-tsan"
$env:RUSTFLAGS = "-Zsanitizer=thread"
rustup toolchain install nightly --component rust-src
rustup target add x86_64-unknown-linux-gnu
cargo +nightly test --test race_model -Zbuild-std --target x86_64-unknown-linux-gnu --locked -- --test-threads=1
```
