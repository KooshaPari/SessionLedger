# Wave-38 lane: w38-slsa-protected — C06 L53 protected-environment SLSA checklist

**Branch:** `feat/sl-w38-slsa-protected`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w38-slsa-protected`
**Cluster / pillar:** C06 L53 (protected-environment residual)

## Gap

SCORECARD: *full protected-environment SLSA Build L3 attestation* unpaid. Add
honest policy SSOT + SelfCheck for GitHub Environments / protected-branch
requirements without claiming live Environment wiring from checkout.

## Acceptance criteria

1. Add `docs/ops/slsa-protected-environment.md` policy SSOT.
2. Add `scripts/slsa-protected-env-check.ps1 -SelfCheck` with done/unpaid rows.
3. Cross-link `docs/ops/hermetic-builds.md` + `branch-protection.md`.
4. Add `tests/slsa_protected_env.rs` wrapper.
5. Soft `slsa-protected-env-check` job in `.github/workflows/hermetic.yml`.
6. CHANGELOG bullet. **Do not edit** audit scorecard/traceability files.

## Verify

```powershell
pwsh ./scripts/slsa-protected-env-check.ps1 -SelfCheck
cargo test --test slsa_protected_env
```

## Score expectation

Evidence toward L53 protected-environment checklist; full L3 attestation and live
Environment proof remain unpaid (conservative).
