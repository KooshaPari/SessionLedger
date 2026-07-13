# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w17-reaudit
**Commit audited:** f6f03ce (origin/main / Wave-17)

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
| C06 | Supply Chain | L51-L60 | 23/30 | 77% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 25/30 | 83% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 23/30 | 77% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 34/45 | 76% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 32/36 | 89% | B | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 81% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 324 / 402.)

## Wave-17 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C06 | 22/30 | 23/30 | +1 | Immutable hermetic builder pin + MSRV gate (#160); L54 digest-pinned offline CI |
| C08 | 22/30 | 23/30 | +1 | Eval reproducibility manifest + PR gate (#161); L79 seed/lockfile/MSRV anchors |
| C10 | 30/36 | 32/36 | +2 | Caption/measure typography tokens + splash/banner (#158); L97 + L103 |
| C11 | 32/45 | 32/45 | 0 | ADR 0003 formalizes signing deferral (#159); L112 deferral already scored — decision evidence only |
| **Overall** | **320/402 (80% B)** | **324/402 (81% B)** | **+4** | Holds B |

## Headline Findings

- **Strongest:** C03 (94% A); C01/C05 (87% B); C10 (89% B); C07 (83% B)
- **Weakest:** C11 Packaging (71% C); C09 (76% C)
- **Wave-16 → Wave-17:** 80% B (320/402) → 81% B (324/402), +4 raw points
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), durable schema/migrations (C00 L3), maintainer 2FA attestation (C04 L36), SLSA-L3 per-build provenance (C06 L53)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
