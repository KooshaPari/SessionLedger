# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w10-reaudit
**Commit audited:** dcde091 (origin/main / Wave-10)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 24/30 | 80% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 21/30 | 70% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 22/30 | 73% | C | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 19/30 | 63% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 21/30 | 70% | C | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 20/30 | 67% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 30/45 | 67% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 25/36 | 69% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 25/45 | 56% | D | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 71% · **Overall grade:** C

(Raw rubric total across all 12 clusters. Sum 285 / 402.)

## Wave-10 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 22/30 | 22/30 | 0 | No architecture acceptance coverage added |
| C01 | 23/30 | 24/30 | +1 | a11y/nightly actions SHA-pinned; CI/a11y/security use cancel-in-progress concurrency |
| C02 | 21/30 | 21/30 | 0 | No new AuthN or durable audit retention |
| C03 | 32/36 | 34/36 | +2 | `.env.example` plus `docs/USER_JOURNEYS.md` with three named journeys |
| C04 | 22/30 | 22/30 | 0 | No new maintainer-control or mandatory signing evidence |
| C05 | 21/30 | 22/30 | +1 | Scheduled/manual ops load smoke; alert/dashboard provisioning still non-live |
| C06 | 19/30 | 19/30 | 0 | No hermetic/SLSA-L3 expansion in Wave-10 |
| C07 | 21/30 | 21/30 | 0 | No race/OS-matrix expansion in Wave-10 |
| C08 | 19/30 | 20/30 | +1 | Criterion/micro bench gate with checked-in baseline on PR/push |
| C09 | 30/45 | 30/45 | 0 | No new a11y/UX acceptance coverage |
| C10 | 25/36 | 25/36 | 0 | No golden/theme automation |
| C11 | 25/45 | 25/45 | 0 | Native installer/signing/channels still unpaid |

## Headline Findings

- **Strongest:** C03 Agent Readiness (94% A); C01 CI/DX (80% B)
- **Weakest:** C11 Packaging (56% D); C06 Supply Chain (63% C); C08 Eval (67% C)
- **Wave-9 re-audit delta:** 68% C (273/402) → 70% C (280/402), +7 raw points
- **Wave-10 re-audit delta:** 70% C (280/402) → 71% C (285/402), +5 raw points / +1 rounded percentage point
- **Why not 75% B:** 17 more raw points are required (minimum 302/402). Highest unpaid levers remain native installer smoke/signing (C11), hermetic/SLSA-L3 (C06), live alert/dashboard routing (C05), race/OS-matrix QEng (C07), and AuthN/durable audit retention (C02)
- **Agent-readiness verdict (C03):** Journey catalog and env sample close remaining orientation gaps; agents can claim and verify FR-mapped paths
- **Time-3 verdict (C11):** Portable archive smoke remains; native installers unsigned and unvalidated for install/uninstall

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md (#68)
- Tray/menubar and background auto-update: accepted Soft / N/A in docs/adr/0001-desktop-companion-scope.md
- Mobile-presence decision remains undocumented and scores 0 at L117
- Apple notarization / Windows Authenticode: deferred in docs/ops/distribution.md; keyless cosign checksum signing and fail-soft GitHub build attestations are implemented
