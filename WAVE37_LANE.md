# Wave-37 lane: w37-miri-hard — C00 L7 blocking Miri permutation CI

**Branch:** `feat/sl-w37-miri-hard`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w37-miri-hard`
**Cluster / pillar:** C00 L7
**Wave-36 overlap:** #296 loom permutation blocking

## Gap

`miri-smoke.yml` is nightly + `continue-on-error`. Add PR-blocking Miri job for
`race_model` (and optional `loom_model` subset) with policy doc anchors.

## Acceptance criteria

1. Add `scripts/miri-permutation-check.ps1 -SelfCheck` documenting policy.
2. Add `.github/workflows/miri-permutation.yml` blocking on PR/push (ubuntu, race_model).
3. Keep existing `miri-smoke.yml` soft nightly or document split.
4. Update `docs/ops/concurrency-safety.md`.
5. CHANGELOG bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/miri-permutation-check.ps1 -SelfCheck
```
