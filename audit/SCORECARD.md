# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w27-reaudit
**Commit audited:** 114428a (origin/main / Wave-27)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 27/30 | 90% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 25/30 | 83% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 25/30 | 83% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 39/45 | 87% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 89% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 358 / 402.)

## Wave-27 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C06 | 25/30 | 26/30 | +1 | OCI verify-on-deploy script (#229); L56 +1 (2→3; release OCI still fail-soft) |
| C11 | 36/45 | 37/45 | +1 | Caddy/nginx TLS reverse-proxy configs (#227); L115 +1 |
| **Overall** | **356/402 (89% B)** | **358/402 (89% B)** | **+2** | Holds B |

## Headline Findings

- **Strongest:** C03 (100% A); C10 (100% A); C05 (97% A); C00/C01/C07 (90% A)
- **Weakest:** C08 Eval (80% B); C11 Packaging (82% B); C02/C04 (83% B)
- **Wave-26 → Wave-27:** 89% B (356/402) → 89% B (358/402), +2 raw points
- **Held (no score):** #228 branch-protection verify (C04 L34 already max; does not pay L36 2FA); #230 clap completions (C01 L15 already max); #231 allocation-budget companion (C00 L8 held — jemalloc/dhat unpaid); #232 unix CPU pprof sampler (C05 L45 held — continuous agent / Windows unpaid)
- **Remaining unpaid:** maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), mandatory blocking OCI release attest (C06 soft), platform Authenticode/notarization (C11 L112), published brew tap / winget-pkgs PR (C11 L109/L120 soft), live Alertmanager webhook IDs (C05 L48), continuous profiling agent beyond on-demand unix pprof (C05 L45), full loom/shuttle/miri permutation checkers (C00 L7), jemalloc/dhat allocator profiling (C00 L8), cross-language eval parity (C08 L75)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
