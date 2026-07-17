# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-17
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w38-reaudit
**Commit audited:** 758f5e5 (origin/main / Wave-38 closure #309-#313)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 29/30 | 97% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 30/30 | 100% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 29/30 | 97% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 27/30 | 90% | A | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 30/30 | 100% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 30/30 | 100% | A | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 29/30 | 97% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 29/30 | 97% | A | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 45/45 | 100% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 41/45 | 91% | A | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 97% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 391 / 402.)

## Wave-38 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C01 | 29/30 | 30/30 | +1 | Fluent `.ftl` catalog stub + optional `fluent-catalog` feature (#312); L16 2→3 |
| **Overall** | **390/402 (97% A)** | **391/402 (97% A)** | **+1** | Conservative; one pillar only |

## Headline Findings

- **Strongest:** C01/C03/C05/C06/C09/C10 (100% A); C00/C02/C07/C08 (97% A)
- **Weakest:** C04 (90% A); C11 Packaging (91% A)
- **Wave-37 → Wave-38:** 97% A (390/402) → 97% A (391/402), +1 raw point
- **Held (no score):** #309 blocking TSan permutation (C00 L7 already pillar max); #310 hard rootless/no-net CI evidence (C04 L40 already pillar max; live runner matrix unpaid); #311 protected-environment SLSA L3 checklist (C06 L53 already pillar max); #313 USE process gauges on `/metrics` (C05 L42 already pillar max)
- **Remaining unpaid:** human org 2FA attestation (C04 L36), live rootless-only runner matrix + blocking cargo-fetch no-net (C04 L40 residual), full protected-environment SLSA Build L3 attestation (C06 L53 residual), live branch-protection signed-commits attestation (C06 L59 residual), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, production Pyroscope profiling push, daemon-graph/broadcast SSE permutation ports (C00 L7), default-on jemalloc, multi-tenant / auto-ETL PII redaction (C02 L24), in-tree KMS/envelope encryption (C02 L22), viewer/CLI Fluent migration (C01 L16 residual)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
- Serverless/edge deploy: explicit N/A per docs/adr/0005-no-serverless-edge.md
- MCP host provenance: explicit N/A per docs/adr/0006-no-mcp-server.md
