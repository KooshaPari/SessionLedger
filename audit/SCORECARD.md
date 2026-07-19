# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-18
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w40-reaudit
**Commit audited:** ec38d21 (origin/main / Wave-40 closure #324-#328)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 30/30 | 100% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 30/30 | 100% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 30/30 | 100% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 27/30 | 90% | A | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 30/30 | 100% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 30/30 | 100% | A | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 30/30 | 100% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 29/30 | 97% | A | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 45/45 | 100% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 43/45 | 96% | A | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 98% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 396 / 402.)

## Wave-40 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C11 | 41/45 | 43/45 | +2 | L111 user-initiated update check blocking gate (#328) 2→3; L112 signing-readiness hard gate (#326) 2→3 |
| **Overall** | **394/402 (98% A)** | **396/402 (98% A)** | **+2** | Conservative; two pillars only |

## Headline Findings

- **Strongest:** C00/C01/C02/C03/C05/C06/C07/C09/C10 (100% A); C08 (97% A)
- **Weakest:** C04 (90% A); C11 Packaging (96% A)
- **Wave-39 → Wave-40:** 98% A (394/402) → 98% A (396/402), +2 raw points
- **Held (no score):** #324 eval-manifest hard sync (C08 L79 already pillar max); #325 rootless-only runner matrix scaffold (C04 L40 already pillar max; live runner matrix unpaid); #327 daemon tokio broadcast loom ports (C00 L7 already pillar max; full tokio sl-daemon broadcast graph unpaid)
- **Remaining unpaid:** Authenticode/notarization live keys (C11 L112 residual), live brew/winget publish, human org 2FA attestation (C04 L36), live rootless-only runner matrix (C04 L40 residual), full protected-environment SLSA Build L3 attestation (C06 L53 residual), live branch-protection signed-commits attestation (C06 L59 residual), live Alertmanager webhooks, production Pyroscope profiling push, full tokio sl-daemon broadcast/SSE graph permutation ports (C00 L7 residual), default-on jemalloc / Windows allocator parity (C00 L8 residual), in-tree KMS (C02 L22 residual), multi-tenant / auto-ETL PII redaction (C02 L24), viewer/CLI Fluent migration (C01 L16 residual), auto-install/rollback updater (C11 L111 residual)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
- Serverless/edge deploy: explicit N/A per docs/adr/0005-no-serverless-edge.md
- MCP host provenance: explicit N/A per docs/adr/0006-no-mcp-server.md
