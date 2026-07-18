# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-18
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w39-reaudit
**Commit audited:** 6283ce5 (origin/main / Wave-39 closure #316-#320)

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
| C11 | Packaging + Distribution | L108-L122 | 41/45 | 91% | A | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 98% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 394 / 402.)

## Wave-39 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 29/30 | 30/30 | +1 | Blocking jemalloc hard gate on Unix release profile (#319); L8 2→3 |
| C02 | 29/30 | 30/30 | +1 | Envelope-crypto blocking SelfCheck + chacha20poly1305 roundtrip (#316); L22 2→3 |
| C07 | 29/30 | 30/30 | +1 | Blocking sustained fuzz cadence beyond soft job (#317); L67 2→3 |
| **Overall** | **391/402 (97% A)** | **394/402 (98% A)** | **+3** | Conservative; three pillars only |

## Headline Findings

- **Strongest:** C00/C01/C02/C03/C05/C06/C07/C09/C10 (100% A); C08 (97% A)
- **Weakest:** C04 (90% A); C11 Packaging (91% A)
- **Wave-38 → Wave-39:** 97% A (391/402) → 98% A (394/402), +3 raw points
- **Held (no score):** #318 blocking cargo-fetch no-net for security jobs (C04 L40 already pillar max; live rootless-only runner matrix unpaid); #320 loom daemon-graph broadcast/SSE permutation ports (C00 L7 already pillar max; full tokio sl-daemon broadcast graph unpaid)
- **Remaining unpaid:** human org 2FA attestation (C04 L36), live rootless-only runner matrix (C04 L40 residual), full protected-environment SLSA Build L3 attestation (C06 L53 residual), live branch-protection signed-commits attestation (C06 L59 residual), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, production Pyroscope profiling push, full tokio sl-daemon broadcast/SSE graph permutation ports (C00 L7 residual), default-on jemalloc / Windows allocator parity (C00 L8 residual), in-tree KMS (C02 L22 residual), multi-tenant / auto-ETL PII redaction (C02 L24), viewer/CLI Fluent migration (C01 L16 residual)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
- Serverless/edge deploy: explicit N/A per docs/adr/0005-no-serverless-edge.md
- MCP host provenance: explicit N/A per docs/adr/0006-no-mcp-server.md
