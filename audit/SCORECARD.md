# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-08
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-v38-completion-fleet
**Commit audited:** 125f7e6 (feat/sl-v38-reaudit post P0 merges)

> Scoring: each sub-pillar 0=absent / 1=seeded / 2=partial / 3=complete, evidence-mandatory.
> Cluster grade: A>=90 · B>=75 · C>=60 · D>=40 · F<40.
> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 18/30 | 60% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 19/30 | 63% | C | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 15/30 | 50% | D | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 30/36 | 83% | B | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 14/30 | 47% | D | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 7/30 | 23% | F | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 10/30 | 33% | F | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 14/30 | 47% | D | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 7/30 | 23% | F | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 13/45 | 29% | F | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 20/36 | 56% | D | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 17/45 | 38% | F | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 46% · **Overall grade:** D

(Equal-weight mean of all 12 clusters. Sum 184 / 402.)

## Headline Findings

- **Strongest:** C03 Agent Readiness (83% B) after FR/PLAN/llms/ops + AGENTS.md
- **Weakest:** C05 Observability deep (23% F) — healthz/metrics present; OTel/SLO/alerts absent
- **Highest-leverage fix:** root Cargo.lock for `--locked` CI + OTel/readyz + cosign on release
- **Agent-readiness verdict (C03):** Agents can orient and claim tasks; FR-linked tests still thin
- **Time-2 verdict (C11):** Releases + packaging scaffold exist; signing/installers incomplete

## N/A / soft goals

- Mobile (L117): product is desktop/web session viewer — document as intentional non-goal (partially done in lanes)
- Serverless/edge: local daemon profile — N/A with rationale in C11
- Tray/auto-update: soft goals, not P0

## Sync

Mirror `audit/.lane-*` + this file to `phenotype-org-audits/audit-v38/output/SessionLedger/`.
