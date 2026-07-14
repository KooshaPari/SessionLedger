# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w32-reaudit
**Commit audited:** 776910b (origin/main / Wave-32)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 28/30 | 93% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 28/30 | 93% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 28/30 | 93% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 26/30 | 87% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 29/30 | 97% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 28/30 | 93% | A | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 44/45 | 98% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 93% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 375 / 402.)

## Wave-32 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C01 | 27/30 | 28/30 | +1 | i18n scaffold / locale catalog stub (#269); L16 0→1 |
| C08 | 27/30 | 28/30 | +1 | Cross-language structural OKF invariant harness (#267); L75 1→2 |
| C09 | 43/45 | 44/45 | +1 | Progressive disclosure beyond command palette (#270); L81.12 2→3 |
| **Overall** | **372/402 (93% A)** | **375/402 (93% A)** | **+3** | Conservative; three pillars only |

## Headline Findings

- **Strongest:** C03/C10 (100% A); C09 (98% A); C05/C07 (97% A); C00/C01/C02/C08 (93% A)
- **Weakest:** C11 Packaging (82% B); C04/C06 (87% B)
- **Wave-31 → Wave-32:** 93% A (372/402) → 93% A (375/402), +3 raw points
- **Held (no score):** #266 sustained fuzz cadence soft job (C07 L67 soft `continue-on-error`; longer crash-corpus triage unpaid); #268 seccomp/no-net soft policy (C04 L40 soft CI; hard rootless/no-net unpaid); #271 W3C traceparent worker helper (C05 L44 already pillar max; OTel parent-context/tracestate unpaid)
- **Remaining unpaid:** maintainer 2FA (C04 L36), full SLSA-L3 isolation (C06), unconditional release-blocking OCI (C06), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, continuous profiling push backend, full loom/shuttle/miri, jemalloc/production allocator, native language adapters beyond structural fixture parity (C08 L75), multi-tenant / in-tree PII redaction pipeline (C02 L24), in-tree KMS/envelope encryption (C02 L22), Fluent/ICU multi-locale (C01 L16), hard seccomp/no-net CI (C04 L40)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
