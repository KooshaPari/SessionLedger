# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w31-reaudit
**Commit audited:** a7d1a8e (origin/main / Wave-31)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 28/30 | 93% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 28/30 | 93% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 26/30 | 87% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 29/30 | 97% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 27/30 | 90% | A | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 43/45 | 96% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 93% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 372 / 402.)

## Wave-31 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C04 | 25/30 | 26/30 | +1 | OSV/NVD CVE feed SSOT + SelfCheck (#259); L38 2→3 |
| C07 | 28/30 | 29/30 | +1 | Lifecycle FSM property tests (#261); L66 2→3 |
| C08 | 26/30 | 27/30 | +1 | Cross-language fixture parity SSOT + blocking SelfCheck (#260); L75 0→1 |
| C09 | 42/45 | 43/45 | +1 | Design-token single source (#263); L81.8 2→3 |
| **Overall** | **368/402 (92% A)** | **372/402 (93% A)** | **+4** | Conservative; four pillars only |

## Headline Findings

- **Strongest:** C03/C10 (100% A); C05/C07 (97% A); C09 (96% A); C00/C02 (93% A)
- **Weakest:** C11 Packaging (82% B); C04/C06 (87% B)
- **Wave-30 → Wave-31:** 92% A (368/402) → 93% A (372/402), +4 raw points
- **Held (no score):** #262 crypto inventory KMS/at-rest Phase-0 deferred guidance (C02 L22 already 2; in-tree KMS unpaid); #264 soft loom concurrency smoke (C00 L7 soft `continue-on-error`; full loom/shuttle/blocking miri unpaid)
- **Remaining unpaid:** maintainer 2FA (C04 L36), full SLSA-L3 isolation (C06), unconditional release-blocking OCI (C06), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, continuous profiling push backend, full loom/shuttle/miri, jemalloc/production allocator, cross-language adapters beyond fixture parity (C08 L75), multi-tenant / in-tree PII redaction pipeline (C02 L24), in-tree KMS/envelope encryption (C02 L22)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
