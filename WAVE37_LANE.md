# Wave-37 lane: w37-shuttle-hard — C00 L7 blocking shuttle permutation

**Branch:** `feat/sl-w37-shuttle-hard`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w37-shuttle-hard`
**Cluster / pillar:** C00 L7
**Wave-36 overlap:** #296 blocking loom permutation; #290 soft shuttle only

## Gap

Post-W36 SCORECARD: *full shuttle/miri/TSan blocking CI* unpaid. Soft shuttle
(`continue-on-error`) exists; promote to blocking permutation evidence mirroring
`loom-permutation.yml` without adding `shuttle` crate to default graph yet.

## Acceptance criteria

1. Add `scripts/shuttle-permutation-check.ps1 -SelfCheck` with done/unpaid rows.
2. Add `tests/shuttle_permutation.rs` wrapper.
3. Add blocking `.github/workflows/shuttle-permutation.yml` (SelfCheck + focused tests).
4. Update `docs/ops/concurrency-safety.md` + `docs/ops/shuttle-soft.md` cross-links.
5. CHANGELOG Unreleased bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/shuttle-permutation-check.ps1 -SelfCheck
cargo test shuttle_permutation
```
