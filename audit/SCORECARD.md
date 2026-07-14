# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w24-reaudit
**Commit audited:** aa8267f (origin/main / Wave-24)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 27/30 | 90% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 24/30 | 80% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 24/30 | 80% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 24/30 | 80% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 39/45 | 87% | B | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 87% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 349 / 402.)

## Wave-24 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 26/30 | 27/30 | +1 | Loom-lite race_model + race-smoke CI (#202); L7 +1 |
| C01 | 26/30 | 27/30 | +1 | `.env.example` + env-example CI gate + rotation docs (#201); L18 +1 |
| C02 | 23/30 | 24/30 | +1 | Non-loopback API-key gate for `/api/*` (#205); L21 +1 |
| C05 | 28/30 | 29/30 | +1 | Gated pprof operator docs + pprof-smoke (#204); L45 +1 |
| C07 | 26/30 | 27/30 | +1 | `jsonl_ingest` fuzz target + corpus/CI (#203); L67 +1 |
| C09 | 38/45 | 39/45 | +1 | Search error-field a11y + Clear confirm (#206); L81.6 +1 |
| **Overall** | **343/402 (85% B)** | **349/402 (87% B)** | **+6** | Holds B; C00/C01/C07 → 90% A |

## Headline Findings

- **Strongest:** C03 (100% A); C10 (100% A); C05 (97% A); C00/C01/C07 (90% A)
- **Weakest:** C11 Packaging (71% C); C02 (80% B)
- **Wave-23 → Wave-24:** 85% B (343/402) → 87% B (349/402), +6 raw points
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), live Alertmanager webhook IDs (C05 L48), full loom/shuttle/miri permutation checkers (C00 L7), continuous CPU profiling beyond stub 501 (C05 L45)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
