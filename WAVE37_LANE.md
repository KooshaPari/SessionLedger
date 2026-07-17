# Wave-37 lane: w37-slsa-reusable — C06 L53 reusable-workflow provenance

**Branch:** `feat/sl-w37-slsa-reusable`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w37-slsa-reusable`
**Cluster / pillar:** C06 L53
**Wave-36 overlap:** #299 partial SLSA isolation checklist

## Gap

SCORECARD: *reusable-workflow provenance* unpaid. Extract at least one release
or hermetic job into a reusable workflow with documented caller provenance contract.

## Acceptance criteria

1. Add `.github/workflows/reusable-hermetic-build.yml` (or reusable release slice).
2. Update caller workflow(s) to `workflow_call` with pinned SHA refs documented.
3. Extend `scripts/slsa-isolation-check.ps1` or add `scripts/reusable-provenance-check.ps1 -SelfCheck`.
4. Update `docs/ops/hermetic-builds.md` / `reproducible-builds.md`.
5. CHANGELOG bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/reusable-provenance-check.ps1 -SelfCheck
```
