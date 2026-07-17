# Wave-36 lane: w36-slsa-l3 — C06 L53 SLSA L3 environment isolation

**Branch:** `feat/sl-w36-slsa-l3`  
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w36-slsa-l3`  
**Cluster / pillar:** C06 L53 (SLSA provenance / environment isolation)  
**Status:** scoped — implementation deferred  
**Wave-35 overlap:** none

## Gap (audit-v38)

Wave-21 #177 landed SLSA materials-metadata contract fixture. SCORECARD:
*full SLSA-L3 isolation* — reusable-workflow provenance, pinned runner image
snapshots, protected release environments — remains unpaid. C06 at 26/30 (B).

## Acceptance criteria

1. Close checklist items in `docs/ops/hermetic-builds.md` § Environment isolation
   (at least one new machine-verifiable row).
2. Add `scripts/slsa-isolation-check.ps1 -SelfCheck` mirroring checklist SSOT.
3. Extend `.github/workflows/hermetic.yml` or `repro-check` policy for isolated
   rebuild evidence (document non-claims where L3 cannot be met yet).
4. Update `docs/ops/reproducible-builds.md` cross-links.
5. **Do not edit** `audit/SCORECARD.md`.

## Files to touch (exclusive)

- `docs/ops/hermetic-builds.md`, `docs/ops/reproducible-builds.md`
- `docs/ops/fixtures/slsa-materials-contract.sample.json` (if contract extends)
- `scripts/slsa-isolation-check.ps1` (new), `scripts/repro-check.ps1` (policy hooks)
- `.github/workflows/hermetic.yml`
- `CHANGELOG.md`

## Verify

```powershell
pwsh ./scripts/slsa-isolation-check.ps1 -SelfCheck
pwsh ./scripts/repro-check.ps1 -PolicyOnly
```

## Score expectation

Incremental L53 evidence; full L3 attestation likely multi-wave / human.
