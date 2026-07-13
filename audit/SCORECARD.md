# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w12-reaudit
**Commit audited:** e19005c (origin/main / Wave-12)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 24/30 | 80% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 22/30 | 73% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 25/30 | 83% | B | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 22/30 | 73% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 25/30 | 83% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 22/30 | 73% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 30/45 | 67% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 26/36 | 72% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 28/45 | 62% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 75% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 302 / 402.)

## Wave-12 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C05 | 22/30 | 25/30 | +3 | W3C traceparent parent context; Alertmanager routing config; dashboard provision check |
| C07 | 23/30 | 25/30 | +2 | Release-matrix cross-platform acceptance + guard; `make seed` sample fixture |
| C08 | 20/30 | 22/30 | +2 | Compression ratio CI gate + token-proxy eval docs |
| C10 | 25/36 | 26/36 | +1 | Committed E1 golden PNG, CI comparison, PROVENANCE table |
| **Overall** | **294/402 (73% C)** | **302/402 (75% B)** | **+8** | Meets ≥75% B threshold |

## Headline Findings

- **Strongest:** C03 Agent Readiness (94% A); C05/C07 now 83% B; C01 80% B
- **Weakest:** C11 Packaging (62% C); C09 a11y/UX (67% C)
- **Wave-11 → Wave-12:** 73% C (294/402) → 75% B (302/402), +8 raw points
- **B threshold:** Met at exactly 302/402 (75%). Residual unpaid levers remain native installer clean-host install/signing (C11), deeper a11y/UX (C09), and AuthN beyond loopback (C02)

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md
- Tray/menubar and background auto-update: Soft / N/A in docs/adr/0001-desktop-companion-scope.md
- Mobile presence: Soft / N/A in docs/adr/0002-mobile-presence.md
- Apple notarization / Windows Authenticode: deferred; keyless cosign + blocking GitHub provenance on canonical releases
