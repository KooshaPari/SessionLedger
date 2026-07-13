# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w16-reaudit
**Commit audited:** 7798234 (origin/main / Wave-16)

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
| C09 | Accessibility + UX | L81-L95 | 34/45 | 76% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 30/36 | 83% | B | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 80% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 320 / 402.)

## Wave-16 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 23/30 | 23/30 | 0 | OpenAPI drift CI gate landed (#153); L2 already at pillar max |
| C04 | 23/30 | 23/30 | 0 | TruffleHog dual-scan in security workflow (#152); L31 already at pillar max |
| C09 | 33/45 | 34/45 | +1 | Production responsive shell + 44px touch targets (#155) |
| C10 | 29/36 | 30/36 | +1 | Live-feed/compare/diff chrome tokenized (#156); spacing/motion scales close L96/L102 gaps at max score |
| C11 | 30/45 | 32/45 | +2 | OCI HEALTHCHECK (#154); Windows install lifecycle CI smoke (#154) |
| **Overall** | **316/402 (79% B)** | **320/402 (80% B)** | **+4** | Holds B |

## Headline Findings

- **Strongest:** C03 (94% A); C01/C05 (87% B); C07/C10 (83% B)
- **Weakest:** C11 Packaging (71% C); C06/C08 (73% C)
- **Wave-15 → Wave-16:** 79% B (316/402) → 80% B (320/402), +4 raw points
- **Remaining unpaid:** platform Authenticode/notarization (C11), durable schema/migrations (C00 L3), maintainer 2FA attestation (C04 L36), caption/measure typography tokens (C10 L97)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred
