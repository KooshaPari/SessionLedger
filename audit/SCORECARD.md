# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w30-reaudit
**Commit audited:** 27435e5 (origin/main / Wave-30)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 28/30 | 93% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 28/30 | 93% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 25/30 | 83% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 28/30 | 93% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 26/30 | 87% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 42/45 | 93% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 92% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 368 / 402.)

## Wave-30 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C02 | 27/30 | 28/30 | +1 | Privacy hygiene SSOT + SelfCheck (#252); L24 1→2 |
| C07 | 27/30 | 28/30 | +1 | Test pyramid SSOT + SelfCheck (#253); L64 2→3 |
| C09 | 41/45 | 42/45 | +1 | Overlay Escape consistency + a11y (#257); L81.9 2→3 |
| **Overall** | **365/402 (91% A)** | **368/402 (92% A)** | **+3** | Conservative; three pillars only |

## Headline Findings

- **Strongest:** C03/C10 (100% A); C00/C02/C07/C09 (93% A); C05 (97% A)
- **Weakest:** C11 Packaging (82% B); C04 Security (83% B); C06/C08 (87% B)
- **Wave-29 → Wave-30:** 91% A (365/402) → 92% A (368/402), +3 raw points
- **Held (no score):** #254 continuous profiling agent stub (C05 L45 soft `continue-on-error`; Pyroscope/OTLP unpaid); #255 sandbox boundary checklist (C04 L40 already 2; seccomp/no-net unpaid; soft CI); #256 enforce p95 SoftLatencyCheck (C00 L6 already 3; blocking evidence refreshed)
- **Remaining unpaid:** maintainer 2FA (C04 L36), full SLSA-L3 isolation (C06), unconditional release-blocking OCI (C06), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, continuous profiling push backend, full loom/shuttle/miri, jemalloc/production allocator, cross-language eval (C08 L75), multi-tenant / in-tree PII redaction pipeline (C02 L24)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
