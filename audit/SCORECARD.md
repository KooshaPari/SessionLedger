# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w15-reaudit
**Commit audited:** da347b3 (origin/main / Wave-15)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 23/30 | 77% | B | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 26/30 | 87% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 23/30 | 77% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 26/30 | 87% | B | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 22/30 | 73% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 25/30 | 83% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 22/30 | 73% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 33/45 | 73% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 29/36 | 81% | B | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 30/45 | 67% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 79% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 316 / 402.)

## Wave-15 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 22/30 | 23/30 | +1 | Committed OpenAPI 3.1 + ingest Idempotency-Key (#148) |
| C04 | 22/30 | 23/30 | +1 | gitleaks pre-commit hook (#149) |
| C10 | 26/36 | 29/36 | +3 | Type/theme tokens, theme preference, AA on-dark accents (#150) |
| **Overall** | **311/402 (77% B)** | **316/402 (79% B)** | **+5** | Holds B |

## Headline Findings

- **Strongest:** C03 (94% A); C01/C05 (87% B); C07 (83% B)
- **Weakest:** C11 Packaging (67% C); C06/C08 (73% C)
- **Wave-14 → Wave-15:** 77% B (311/402) → 79% B (316/402), +5 raw points
- **Remaining unpaid:** native installer signing/clean-host install (C11), OpenAPI drift CI (C00 L2), trufflehog dual-scan (C04 L31), spacing/motion tokens + live-feed tokenization (C10)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred
