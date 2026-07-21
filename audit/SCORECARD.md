# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-18
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w42-reaudit
**Commit audited:** d5f999f (origin/main / Wave-42 closure #339-#344)

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

## Wave-42 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| — | 396/402 | 396/402 | 0 | All five impl lanes deepen evidence at pillar max; conservative hold |
| **Overall** | **396/402 (98% A)** | **396/402 (98% A)** | **0** | Conservative; no raw score inflation |

## Headline Findings

- **Strongest:** C00/C01/C02/C03/C05/C06/C07/C09/C10 (100% A); C08 (97% A)
- **Weakest:** C04 (90% A); C11 Packaging (96% A)
- **Wave-41 → Wave-42:** 98% A (396/402) → 98% A (396/402), held
- **Held (no score):** #340 bounded commit-signing header scan (C04 L34 already pillar max); #341 pinned CycloneDX + SBOM schema validation (C04 L32 residual unpaid); #342 SLSA protected-env blocking on PRs (C06 L53 residual attestation unpaid); #343 blocking alloc-profile / dhat hard gate (C00 L8 already pillar max); #344 first-run corpus CTA (C09 UX polish)
- **Remaining unpaid:** Authenticode/notarization live keys (C11 L112 residual), live brew/winget publish, human org 2FA attestation (C04 L36), live rootless-only runner matrix (C04 L40 residual), full protected-environment SLSA Build L3 attestation (C06 L53 residual), live branch-protection signed-commits attestation (C06 L59 residual), live Alertmanager webhooks, production Pyroscope profiling push, full tokio sl-daemon broadcast/SSE graph permutation ports (C00 L7 residual), default-on jemalloc / Windows allocator parity (C00 L8 residual), in-tree KMS (C02 L22 residual), multi-tenant / auto-ETL PII redaction (C02 L24), viewer/CLI Fluent migration (C01 L16 residual), auto-install/rollback updater (C11 L111 residual), phenotype-org-audits org mirror (403/403)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
- Serverless/edge deploy: explicit N/A per docs/adr/0005-no-serverless-edge.md
- MCP host provenance: explicit N/A per docs/adr/0006-no-mcp-server.md
