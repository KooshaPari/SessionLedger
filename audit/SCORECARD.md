# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w23-reaudit
**Commit audited:** 1a45f2c (origin/main / Wave-23)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 26/30 | 87% | B | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 26/30 | 87% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 24/30 | 80% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 28/30 | 93% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 24/30 | 80% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 26/30 | 87% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 38/45 | 84% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 85% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 343 / 402.)

## Wave-23 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 25/30 | 26/30 | +1 | RSS memory budget smoke for ingest (#196); L8 +1 |
| C03 | 34/36 | 36/36 | +2 | Role-form FR stories (#195) + measured feedback budgets (#197); L30.1 +1, L30.10 +1 |
| C06 | 23/30 | 24/30 | +1 | SOURCE_DATE_EPOCH + release repro wiring (#194); L52 +1 |
| C09 | 37/45 | 38/45 | +1 | SR procedure + a11y depth (#198); L81.4 +1 |
| C10 | 35/36 | 36/36 | +1 | Launch splash visual goldens (#199); L103 +1 |
| **Overall** | **337/402 (84% B)** | **343/402 (85% B)** | **+6** | Holds B; C03/C10 → 100% A |

## Headline Findings

- **Strongest:** C03 (100% A); C10 (100% A); C05 (93% A); C00/C01/C07 (87% B)
- **Weakest:** C11 Packaging (71% C); C02 (77% B)
- **Wave-22 → Wave-23:** 84% B (337/402) → 85% B (343/402), +6 raw points
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), live Alertmanager webhook IDs (C05 L48), formal concurrency checkers (C00 L7)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
