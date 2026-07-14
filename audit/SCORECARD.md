# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w28-reaudit
**Commit audited:** 495b3a8 (origin/main / Wave-28)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 28/30 | 93% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 26/30 | 87% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 25/30 | 83% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 25/30 | 83% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 40/45 | 89% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 90% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 362 / 402.)

## Wave-28 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 27/30 | 28/30 | +1 | Soft p95 SoftLatencyCheck + baselines (#243); L6 2→3 |
| C02 | 25/30 | 26/30 | +1 | API circuit breaker + CLI retry (#240); L26 2→3 |
| C08 | 24/30 | 25/30 | +1 | Compression/token fixtures 20 + eval gate (#239); L77 2→3 |
| C09 | 39/45 | 40/45 | +1 | Cmd+K command palette (#236); L81.12 1→2 |
| **Overall** | **358/402 (89% B)** | **362/402 (90% A)** | **+4** | First A overall |

## Headline Findings

- **Strongest:** C03/C10 (100% A); C00 (93% A); C05 (97% A); C01/C07 (90% A)
- **Weakest:** C11 Packaging (82% B); C04 Security (83% B); C08 Eval (83% B)
- **Wave-27 → Wave-28:** 89% B (358/402) → 90% A (362/402), +4 raw points
- **Held (no score):** #233 brew/winget fill (C11 L109/L120 already max; no live tap); #234 hermetic SelfCheck (C06 L53 already max; unpaid L3 rows remain); #235 soft miri (C00 L7 held — continue-on-error, no loom/shuttle); #237 alert route stubs (C05 L48 already max; REPLACE_ME unpaid); #238 live-daemon native attach (C09 soft evidence; L81.4 already max); #242 exotic cargo check (C07 L69 already max)
- **Remaining unpaid:** maintainer 2FA (C04 L36), full SLSA-L3 isolation (C06), blocking OCI release attest, Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, continuous profiling agent, full loom/shuttle/miri, jemalloc/dhat, cross-language eval (C08 L75), enforce soft latency (`latency.enforced=true`)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
