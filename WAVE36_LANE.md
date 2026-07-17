# Wave-36 lane: w36-loom — C00 L7 full loom permutation checkers

**Branch:** `feat/sl-w36-loom`  
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w36-loom`  
**Cluster / pillar:** C00 L7 (concurrency safety)  
**Status:** scoped — implementation deferred  
**Wave-35 overlap:** none (`w35-shuttle` covers shuttle soft smoke only)

## Gap (audit-v38)

SCORECARD headline: *full loom/shuttle/miri* remain unpaid. Wave-31 landed soft
loom smoke (`continue-on-error`); Wave-35 #290 lands shuttle soft SelfCheck.
**Full loom permutation checkers** for broadcast/SSE/daemon graph remain.

## Acceptance criteria

1. Expand `tests/loom_model.rs` (or sibling) with permutation coverage beyond
   the current `sync_channel` capacity model.
2. Add `scripts/loom-permutation-check.ps1 -SelfCheck` documenting unpaid vs
   done rows in `docs/ops/concurrency-safety.md`.
3. Promote `.github/workflows/loom-smoke.yml` toward blocking **or** add a new
   `loom-permutation.yml` job that runs `cargo test --cfg loom` permutation suite.
4. `tests/loom_model.rs` / rust wrapper proves hermetic doc anchors.
5. **Do not edit** `audit/SCORECARD.md` in this PR.

## Files to touch (exclusive)

- `tests/loom_model.rs`, `tests/race_model.rs` (read-only cross-link only)
- `scripts/loom-smoke-check.ps1`, new `scripts/loom-permutation-check.ps1`
- `docs/ops/concurrency-safety.md`
- `.github/workflows/loom-smoke.yml` (or new workflow)
- `CHANGELOG.md` (Unreleased)

## Verify

```powershell
$env:CARGO_TARGET_DIR = "C:\Users\koosh\SessionLedger-wtrees\w36-loom\target-w36-loom"
pwsh ./scripts/loom-permutation-check.ps1 -SelfCheck
cargo test loom
```

## Score expectation

Conservative: evidence toward L7; score held until blocking permutation CI
lands (per Wave-31/Wave-35 soft-gate precedent).
