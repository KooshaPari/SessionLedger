# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w22-reaudit
**Commit audited:** b08c3fe (origin/main / Wave-22)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 25/30 | 83% | B | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 26/30 | 87% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 24/30 | 80% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 28/30 | 93% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 23/30 | 77% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 26/30 | 87% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 37/45 | 82% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 35/36 | 97% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 84% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 337 / 402.)

## Wave-22 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 24/30 | 25/30 | +1 | Cooperative CancellationToken on serve (#190); L4 +1 |
| C02 | 23/30 | 23/30 | 0 | Audit retention/export policy + audit-review.ps1 (#187); L23 already pillar max |
| C05 | 27/30 | 28/30 | +1 | Multi-window SLO burn rules + game-day/alert routing (#189); L46 +1 |
| C08 | 24/30 | 24/30 | 0 | Eval corpus grown to fourteen multi-lang fixtures (#188); L71/L80 already pillar max |
| C09 | 35/45 | 37/45 | +2 | Status regions + inclusive/cognitive CI + help golden (#192); L81.7 +1, L81.10 +1 |
| C10 | 35/36 | 35/36 | 0 | E5 first-run empty + R2/R3 error goldens (#191); L100/L101/L107 already pillar max |
| **Overall** | **333/402 (83% B)** | **337/402 (84% B)** | **+4** | Holds B; C09 C→B |

## Headline Findings

- **Strongest:** C03 (94% A); C10 (97% A); C05 (93% A); C01/C07 (87% B)
- **Weakest:** C11 Packaging (71% C); C02/C06 (77% B)
- **Wave-21 → Wave-22:** 83% B (333/402) → 84% B (337/402), +4 raw points
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), live Alertmanager webhook IDs (C05 L48)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
