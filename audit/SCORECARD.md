# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w26-reaudit
**Commit audited:** cbf54d1 (origin/main / Wave-26)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 27/30 | 90% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 25/30 | 83% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 25/30 | 83% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 25/30 | 83% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 39/45 | 87% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 36/45 | 80% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 89% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 356 / 402.)

## Wave-26 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C02 | 24/30 | 25/30 | +1 | Process-wide `/api/*` rate limit beyond ingest bulkhead (#222); L25 +1 |
| C04 | 24/30 | 25/30 | +1 | Renovate + patch automerge (#220); L37 +1 (gitleaks SARIF refreshes L33 at pillar max) |
| C06 | 24/30 | 25/30 | +1 | Best-effort OCI cosign sign + attest for sl-daemon (#224); L56 +1 (1→2; verify-on-deploy unpaid) |
| **Overall** | **353/402 (88% B)** | **356/402 (89% B)** | **+3** | Holds B |

## Headline Findings

- **Strongest:** C03 (100% A); C10 (100% A); C05 (97% A); C00/C01/C07 (90% A)
- **Weakest:** C08 Eval (80% B); C11 Packaging (80% B); C02/C04/C06 (83% B)
- **Wave-25 → Wave-26:** 88% B (353/402) → 89% B (356/402), +3 raw points
- **Held (no score):** #225 quality-gate SHA pin (C01 L10 already max); #221 seventeen-fixture task-family corpus (C08 L71/L80 already max); #223 enforced perf-budget gate (C08 L74 already max; closes WBS-6.2 perf soft goal)
- **Remaining unpaid:** maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), OCI verify-on-deploy (C06 L56), platform Authenticode/notarization (C11 L112), published brew tap / winget-pkgs PR (C11 L109/L120 soft), live Alertmanager webhook IDs (C05 L48), full loom/shuttle/miri permutation checkers (C00 L7), continuous CPU profiling beyond stub 501 (C05 L45), cross-language eval parity (C08 L75)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
