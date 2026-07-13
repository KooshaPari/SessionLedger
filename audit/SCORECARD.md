# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w13-reaudit
**Commit audited:** d6441eb (origin/main / Wave-13)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 24/30 | 80% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 25/30 | 83% | B | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 22/30 | 73% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 25/30 | 83% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 22/30 | 73% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 31/45 | 69% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 26/36 | 72% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 30/45 | 67% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 76% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 306 / 402.)

## Wave-13 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C02 | 22/30 | 23/30 | +1 | Optional `SL_API_KEY` gate on mutating ingest |
| C09 | 30/45 | 31/45 | +1 | Built-viewer overflow assertions at 360/768/1280 |
| C11 | 28/45 | 30/45 | +2 | Install channels doc + systemd unit |
| **Overall** | **302/402 (75% B)** | **306/402 (76% B)** | **+4** | Holds B; residual C11 signing/clean-host install and deeper C09 unpaid |

## Headline Findings

- **Strongest:** C03 (94% A); C05/C07 (83% B); C01/C02 (77–80% B)
- **Weakest:** C11 Packaging (67% C); C09 a11y/UX (69% C)
- **Wave-12 → Wave-13:** 75% B (302/402) → 76% B (306/402), +4 raw points
- **Remaining unpaid for higher grades:** native installer clean-host install/signing (C11), deeper cognitive/help/hotkey UX (C09), continuous profiling (C05 L45)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred; keyless cosign + blocking GitHub provenance on canonical releases
