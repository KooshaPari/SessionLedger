# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-10
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w2-fanout
**Commit audited:** 4feed00 (feat/sl-w2-reaudit / Wave-2)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 19/30 | 63% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 21/30 | 70% | C | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 18/30 | 60% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 31/36 | 86% | B | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 19/30 | 63% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 16/30 | 53% | D | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 16/30 | 53% | D | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 18/30 | 60% | C | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 14/30 | 47% | D | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 23/45 | 51% | D | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 24/36 | 67% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 22/45 | 49% | D | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 60% · **Overall grade:** C

(Equal-weight mean of all 12 clusters. Sum 241 / 402.)

## Headline Findings

- **Strongest:** C03 Agent Readiness (86% B); C01 CI (70% C); C10 Visual (67% C)
- **Weakest:** C05/C06 still mid-D — OTel export and SLSA/signing remain soft goals
- **Wave-2 delta:** 46% D → 60% C (target ≥60% C)
- **Agent-readiness verdict (C03):** Agents can orient, claim PLAN tasks, and use FR catalog
- **Time-2 verdict (C11):** Distribution docs + packaging scaffold; codesign deferred

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md (#68)
- Dioxus 0.7: held (#77 / issue #82)
- Tray/auto-update/mobile: soft goals
- Cosign/notarization: deferred in docs/ops/distribution.md (#66)
