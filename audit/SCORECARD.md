# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w25-reaudit
**Commit audited:** c60c205 (origin/main / Wave-25)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 27/30 | 90% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 24/30 | 80% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 24/30 | 80% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 24/30 | 80% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 39/45 | 87% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 36/45 | 80% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 88% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 353 / 402.)

## Wave-25 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C11 | 32/45 | 36/45 | +4 | Unsigned MSI/PKG release smoke (#211) L108; curl/irm install (#210) L109; runtime facade parity (#209) L116; brew/winget manifests (#210) L120 |
| **Overall** | **349/402 (87% B)** | **353/402 (88% B)** | **+4** | Holds B; C11 71% C → 80% B |

## Headline Findings

- **Strongest:** C03 (100% A); C10 (100% A); C05 (97% A); C00/C01/C07 (90% A)
- **Weakest:** C02/C04/C06/C08 (80% B); C11 Packaging (80% B, formerly weakest at 71% C)
- **Wave-24 → Wave-25:** 87% B (349/402) → 88% B (353/402), +4 raw points
- **Held (no score):** #208 just/Taskfile reinforce C01 L13 / C07 L62 already at pillar max
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), published brew tap / winget-pkgs PR (C11 L109/L120 soft), maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), live Alertmanager webhook IDs (C05 L48), full loom/shuttle/miri permutation checkers (C00 L7), continuous CPU profiling beyond stub 501 (C05 L45)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
