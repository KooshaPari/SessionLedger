# Wave-37 lane: w37-source-prov — C06 L59 source-code provenance policy

**Branch:** `feat/sl-w37-source-prov`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w37-source-prov`
**Cluster / pillar:** C06 L59

## Gap

L59 score 2: signed merges present but no in-repo signed-commit policy SSOT.
Extend branch-protection + contributor policy with machine-verifiable anchors.

## Acceptance criteria

1. Add `docs/ops/source-provenance.md` SSOT (signed commits, CODEOWNERS, human org gates).
2. Add `scripts/source-provenance-check.ps1 -SelfCheck`.
3. Extend `scripts/branch-protection-check.ps1` policy hooks if needed.
4. Update `CONTRIBUTING.md` cross-link.
5. CHANGELOG bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/source-provenance-check.ps1 -SelfCheck
pwsh ./scripts/branch-protection-check.ps1 -PolicyOnly
```
