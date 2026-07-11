# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-11
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w7-reaudit
**Commit audited:** bb400c9 (origin/main / Wave-7)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 22/30 | 73% | C | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 18/30 | 60% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 31/36 | 86% | B | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 21/30 | 70% | C | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 18/30 | 60% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 18/30 | 60% | C | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 19/30 | 63% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 27/45 | 60% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 25/36 | 69% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 25/45 | 56% | D | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 67% · **Overall grade:** C

(Raw rubric total across all 12 clusters. Sum 268 / 402.)

## Wave-7 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 22/30 | 22/30 | 0 | No Wave-7 architecture acceptance criterion closed |
| C01 | 21/30 | 22/30 | +1 | Core CI/security/release actions are SHA-pinned; auxiliary workflows and one reusable workflow remain mutable |
| C02 | 18/30 | 18/30 | 0 | Wave-7 operational evidence does not add actor/action governance or API error-envelope coverage |
| C03 | 31/36 | 31/36 | 0 | No Wave-7 agent-readiness acceptance criterion closed |
| C04 | 19/30 | 22/30 | +3 | Cosign/Rekor release signing plus GitHub OIDC build attestations satisfy signed-release evidence; rollout remains fail-soft |
| C05 | 18/30 | 21/30 | +3 | Importable per-service RED dashboard (+2) and one manual concurrent load script (+1); alert routing and chaos cadence remain absent |
| C06 | 17/30 | 18/30 | +1 | GitHub build provenance reaches partial evidence; fail-soft release-job attestation is not SLSA L3 |
| C07 | 18/30 | 18/30 | 0 | New browser/load harnesses do not close property, fuzz, flake, or PR cross-platform gaps |
| C08 | 18/30 | 19/30 | +1 | Micro + representative macro + concurrent load-SLO tiers now exist; no per-PR performance gate |
| C09 | 24/45 | 27/45 | +3 | Main/ARIA structure, Escape exits, and viewport checks improve coverage; axe scans a mirror fixture rather than the built viewer |
| C10 | 25/36 | 25/36 | 0 | The contract fixture reinforces existing tokens but closes no separate visual-identity criterion |
| C11 | 25/45 | 25/45 | 0 | SBOM/attestation/dashboard evidence strengthens already-full release/observability pillars; native installer/signing/channel gaps remain |

## Headline Findings

- **Strongest:** C03 Agent Readiness (86% B); C00 Architecture, C01 CI, and C04 Security (73% C)
- **Weakest:** C11 Packaging (56% D); C06 Supply Chain and C09 Accessibility (60% C)
- **Wave-2 delta:** 46% D → 60% C (target ≥60% C)
- **Wave-3 delta:** 60% C → 61% C (+5 raw points); optional OTLP traces, keyless checksum signing, reduced-motion, and a visual harness landed
- **Wave-6 re-audit delta:** 61% C → 64% C (+10 raw points); this is measurable C-grade movement, not a B
- **Wave-7 re-audit delta:** 64% C (256/402) → 67% C (268/402), +12 raw points / +3 rounded percentage points
- **Why not 75% B:** 33 more raw points are required. Current evidence still lacks native installers/platform signing, built-viewer a11y scans, scheduled load/chaos testing, live dashboard/alert provisioning, and reproducible/hermetic/SLSA-L3 builds
- **Agent-readiness verdict (C03):** Agents can orient, claim PLAN tasks, and use FR catalog
- **Time-3 verdict (C11):** Release SBOMs and attestations now complement checksum-verified archives, but package channels and native installers/platform signing remain incomplete

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md (#68)
- Dioxus 0.7 migration and sha2 0.11 upgrade landed after Wave-3; neither closes a separate rubric point by itself
- Tray/menubar and background auto-update: accepted Soft / N/A in docs/adr/0001-desktop-companion-scope.md; rubric still records missing daemon-tray QOL
- Mobile-presence decision remains undocumented and scores 0 at L117
- Apple notarization / Windows Authenticode: deferred in docs/ops/distribution.md (#66); keyless cosign checksum signing and fail-soft GitHub build attestations are implemented
