# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-14
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w29-reaudit
**Commit audited:** 0bc319d (origin/main / Wave-29)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 28/30 | 93% | A | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 27/30 | 90% | A | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 27/30 | 90% | A | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 36/36 | 100% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 25/30 | 83% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 29/30 | 97% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 26/30 | 87% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 27/30 | 90% | A | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 26/30 | 87% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 41/45 | 91% | A | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 36/36 | 100% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 37/45 | 82% | B | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 91% · **Overall grade:** A

(Raw rubric total across all 12 clusters. Sum 365 / 402.)

## Wave-29 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C02 | 26/30 | 27/30 | +1 | Crypto inventory + TLS deploy guidance (#247); L22 1→2 |
| C08 | 25/30 | 26/30 | +1 | Per-fixture token-burn ledger smoke (#248); L78 2→3 |
| C09 | 40/45 | 41/45 | +1 | Expanded Cmd+K palette actions (#249); L81.14 2→3 |
| **Overall** | **362/402 (90% A)** | **365/402 (91% A)** | **+3** | Conservative; three pillars only |

## Headline Findings

- **Strongest:** C03/C10 (100% A); C00 (93% A); C05 (97% A); C01/C07 (90% A); C09 (91% A)
- **Weakest:** C11 Packaging (82% B); C04 Security (83% B); C08 Eval (87% B)
- **Wave-28 → Wave-29:** 90% A (362/402) → 91% A (365/402), +3 raw points
- **Held (no score):** #245 signing readiness checklist (C11 L112 already 2; deferred/unsigned); #246 release-blocking OCI when credentials present (C06 L56 already 3; skip path when GHCR/OIDC absent); #250 soft dhat alloc-profile smoke (C00 L8 held — jemalloc/production `#[global_allocator]` unpaid)
- **Remaining unpaid:** maintainer 2FA (C04 L36), full SLSA-L3 isolation (C06), unconditional release-blocking OCI (C06), Authenticode/notarization (C11 L112), live brew/winget publish, live Alertmanager webhooks, continuous profiling agent, full loom/shuttle/miri, jemalloc/dhat production allocator, cross-language eval (C08 L75), enforce soft latency (`latency.enforced=true`)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
