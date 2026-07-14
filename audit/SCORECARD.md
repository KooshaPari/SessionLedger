# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w19-reaudit
**Commit audited:** 6344eff (origin/main / Wave-19)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 24/30 | 80% | B | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 26/30 | 87% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 23/30 | 77% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 26/30 | 87% | B | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 23/30 | 77% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 26/30 | 87% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 23/30 | 77% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 34/45 | 76% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 33/36 | 92% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 81% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 327 / 402.)

## Wave-19 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 24/30 | 24/30 | 0 | SqliteMemoryStore adapter + sqlite CI gate (#171); L3 already pillar max — closes durable adapter gap |
| C05 | 26/30 | 26/30 | 0 | Per-route HTTP histogram buckets + `route` labels on `/metrics` (#169); L43 already pillar max — dashboard route panels remain |
| C07 | 25/30 | 26/30 | +1 | Flake tracker validation + property rerun stats CI (#170); L68 +1 |
| C10 | 33/36 | 33/36 | 0 | ContentSkeleton `StreamFeed` on live feed + replay (#168); L99 already pillar max — closes async-pane adoption gap |
| **Overall** | **326/402 (81% B)** | **327/402 (81% B)** | **+1** | Holds B |

## Headline Findings

- **Strongest:** C03 (94% A); C10 (92% A); C01/C05/C07 (87% B)
- **Weakest:** C11 Packaging (71% C); C09 (76% C)
- **Wave-18 → Wave-19:** 81% B (326/402) → 81% B (327/402), +1 raw point
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), maintainer 2FA attestation (C04 L36), SLSA-L3 material-metadata provenance (C06 L53), provisioned Grafana route rollups (C05 L49)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
