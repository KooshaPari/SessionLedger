# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w11-reaudit
**Commit audited:** fa65de4 (origin/main / Wave-11)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 24/30 | 80% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 22/30 | 73% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 22/30 | 73% | C | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 22/30 | 73% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 23/30 | 77% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 20/30 | 67% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 30/45 | 67% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 25/36 | 69% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 28/45 | 62% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 73% · **Overall grade:** C

(Raw rubric total across all 12 clusters. Sum 294 / 402.)

## Wave-11 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C02 | 21/30 | 22/30 | +1 | Durable append-only JSONL audit sink under local data dir |
| C06 | 19/30 | 22/30 | +3 | Hermetic offline CI for sl-daemon; blocking provenance on canonical releases |
| C07 | 21/30 | 23/30 | +2 | `.editorconfig` + flake tracker seed; race smoke supports QEng depth |
| C11 | 25/45 | 28/45 | +3 | Mobile presence N/A ADR; installer lifecycle smoke remains scaffold-only |
| **Overall** | **285/402 (71% C)** | **294/402 (73% C)** | **+9** | Still **8** raw points below 302/402 B |

## Headline Findings

- **Strongest:** C03 Agent Readiness (94% A); C01 CI/DX (80% B); C07 now 77% B
- **Weakest:** C11 Packaging (62% C); C08 Eval (67% C); C09 a11y/UX (67% C)
- **Wave-10 → Wave-11:** 71% C (285/402) → 73% C (294/402), +9 raw points
- **Why not 75% B:** 8 more raw points required. Remaining high-value unpaid levers: native installer install/uninstall on clean hosts + platform signing (C11), live alert/dashboard routing (C05), PR OS matrix (C07 L69), AuthN beyond loopback (C02), visual goldens (C10)

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md
- Tray/menubar and background auto-update: Soft / N/A in docs/adr/0001-desktop-companion-scope.md
- Mobile presence: Soft / N/A in docs/adr/0002-mobile-presence.md (Wave-11)
- Apple notarization / Windows Authenticode: deferred; keyless cosign checksum signing + blocking GitHub provenance on canonical releases
