# Wave-40 scope — SessionLedger audit-v38 (394/402)

**Base:** `origin/main` @ `ab29885` (Wave-39 closure #316–#322)  
**Method:** AgilePlus phased WBS → parallel lanes (width 5) → merge greens → reaudit → org mirror  
**Auditor posture:** conservative +1 per wave; pillar caps held without live creds/org attestation

## Audit enumeration (unpaid, machine-actionable first)

| Priority | Cluster / pillar | Gap (from SCORECARD #322) | Score today | W40 lane |
|:--------:|-------------------|---------------------------|:-----------:|----------|
| P0 | C11 L112 | blocking signing-readiness gate on release path (soft today) | 2 | w40-signing-hard |
| P0 | C11 L111 | user-initiated update check / version drift (no auto-install) | 2 | w40-update-check |
| P0 | C08 L79 | eval reproducibility manifest drift (`cargo_lock_sha256`) | 3 (held) | w40-eval-repro |
| P1 | C04 L40 | rootless-only OCI runner matrix scaffold (live creds unpaid) | 3 (held) | w40-rootless-matrix |
| P1 | C00 L7 | sl-daemon tokio broadcast/SSE graph loom ports (partial) | 3 (held) | w40-daemon-tokio |

**Deferred (human/creds):** C04 L36 org 2FA attestation; C11 L112 Authenticode/notarization live keys; live brew/winget publish; C06 L53 full protected-environment L3 attestation; C05 live Alertmanager/Pyroscope; C08 L76 Harbor agent-eval (N/A).

## Five lanes

| Lane | Branch | Worktree | Target | Score expectation |
|------|--------|----------|--------|-------------------|
| w40-signing-hard | `feat/sl-w40-signing-hard` | `w40-signing-hard` | C11 L112 blocking signing-readiness CI | **+1** (2→3) strongest |
| w40-update-check | `feat/sl-w40-update-check` | `w40-update-check` | C11 L111 user-initiated update check | **+1** (2→3) |
| w40-eval-repro | `feat/sl-w40-eval-repro` | `w40-eval-repro` | C08 L79 eval-manifest SHA sync + hard gate | held (L79 pillar max; fixes CI flake) |
| w40-rootless-matrix | `feat/sl-w40-rootless-matrix` | `w40-rootless-matrix` | C04 L40 rootless-only runner scaffold | held (L40 pillar max) |
| w40-daemon-tokio | `feat/sl-w40-daemon-tokio` | `w40-daemon-tokio` | C00 L7 sl-daemon broadcast loom ports | held (L7 pillar max) |

## PERT / critical path

See [`docs/ops/WAVE40_PERT.md`](docs/ops/WAVE40_PERT.md). Critical path:

1. **Scope PR** (this commit) — WBS/DAG/TRACEABILITY plan rows  
2. **Parallel impl** (5 lanes, no audit file edits)  
3. **Sequential merge** — eval-repro → signing → update-check → rootless → daemon-tokio  
4. **Reaudit PR** — SCORECARD + lanes + GAP/TRACEABILITY/WBS closure  
5. **Org mirror** — `phenotype-org-audits` (blocked while archived)

Estimated merge order (lowest conflict risk first): eval-repro → signing-hard → update-check → rootless-matrix → daemon-tokio.

## AgilePlus mapping

| WBS | AgilePlus epic (SessionLedger) | Story stub |
|-----|-------------------------------|------------|
| WBS-8.60 | Lifecycle and operations (NFR) | W40 evidence packages — five machine lanes |
| WBS-8.61 | Lifecycle and operations (NFR) | W40 independent re-audit refresh |

## Rules (unchanged)

- **Do not edit** `audit/SCORECARD.md`, `GAP_QA_MATRIX.md`, `TRACEABILITY.json`, `WBS.md` status in feature PRs — reaudit only  
- Per-worktree `CARGO_TARGET_DIR=target-w40-*`; never `git add -A` with targets inside worktree  
- CHANGELOG conflicts: keep `main` Unreleased + lane bullet  
- Commits: `git -c commit.gpgsign=false`; `Co-authored-by: Cursor <cursoragent@cursor.com>`

## Verify (post-impl wave)

```powershell
pwsh ./docs/ops/traceability_lint.ps1
cargo test --locked
```
