# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-12
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w9-reaudit
**Commit audited:** fe9e26b (origin/main / Wave-9)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 23/30 | 77% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 21/30 | 70% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 32/36 | 89% | B | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 22/30 | 73% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 21/30 | 70% | C | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 19/30 | 63% | C | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 21/30 | 70% | C | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 19/30 | 63% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 30/45 | 67% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 25/36 | 69% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 25/45 | 56% | D | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 70% · **Overall grade:** C

(Raw rubric total across all 12 clusters. Sum 280 / 402.)

## Wave-9 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 22/30 | 22/30 | 0 | No architecture acceptance coverage added |
| C01 | 23/30 | 23/30 | 0 | Wave-9 CI jobs do not change CI/DX pin/concurrency criteria |
| C02 | 18/30 | 21/30 | +3 | Loopback bind enforcement, ingest admission/JSON errors, structured local audit events |
| C03 | 32/36 | 32/36 | 0 | Traceability already credited in Wave-8 |
| C04 | 22/30 | 22/30 | 0 | No new maintainer-control, secret-policy, or mandatory signing evidence |
| C05 | 21/30 | 21/30 | 0 | No new live dashboard/alert or scheduled chaos/load evidence |
| C06 | 18/30 | 19/30 | +1 | PR-gated Linux two-build `sl-daemon` repro-check (not hermetic/SLSA L3) |
| C07 | 18/30 | 21/30 | +3 | proptest suites + fuzz harness with PR property/fuzz smoke |
| C08 | 19/30 | 19/30 | 0 | Property/fuzz evidence is QEng, not a perf regression gate |
| C09 | 30/45 | 30/45 | 0 | No new a11y/UX evidence beyond Wave-8 built-viewer axe |
| C10 | 25/36 | 25/36 | 0 | No deterministic golden or identity-system acceptance coverage |
| C11 | 25/45 | 25/45 | 0 | Portable archive release smoke strengthens partial packaging evidence but does not clear native installer/signing/channel pillars |

## Headline Findings

- **Strongest:** C03 Agent Readiness (89% B); C01 CI/DX (77% B); C02 and C07 now 70% C
- **Weakest:** C11 Packaging (56% D); C06 Supply Chain (63% C); C08 Eval (63% C)
- **Wave-2 delta:** 46% D → 60% C (target ≥60% C)
- **Wave-3 delta:** 60% C → 61% C (+5 raw points); optional OTLP traces, keyless checksum signing, reduced-motion, and a visual harness landed
- **Wave-6 re-audit delta:** 61% C → 64% C (+10 raw points); this is measurable C-grade movement, not a B
- **Wave-7 re-audit delta:** 64% C (256/402) → 67% C (268/402), +12 raw points / +3 rounded percentage points
- **Wave-8 re-audit delta:** 67% C (268/402) → 68% C (273/402), +5 raw points / +1 rounded percentage point
- **Wave-9 re-audit delta:** 68% C (273/402) → 70% C (280/402), +7 raw points / +2 rounded percentage points
- **Why not 75% B:** 22 more raw points are required (minimum 302/402). Current evidence still lacks validated/published native installers and platform signing, scheduled load/chaos gates, live dashboard/alert provisioning, hermetic/SLSA-L3 builds, race/OS-matrix QEng, and AuthN/durable audit retention
- **Agent-readiness verdict (C03):** Agents can orient, claim PLAN/WBS work, inspect FR and audit gaps, and rely on CI-enforced status-mirror consistency
- **Accessibility verdict (C09):** Built production Dioxus web markup is scanned across all tabs and three viewports; native WebView/live-data and broader UX criteria remain partial
- **Time-3 verdict (C11):** Portable Windows/Linux archives now have release-gating `--version` smoke; native-format scaffolds remain unvalidated for install/uninstall and unsigned

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md (#68)
- Dioxus 0.7 migration and sha2 0.11 upgrade landed after Wave-3; neither closes a separate rubric point by itself
- Tray/menubar and background auto-update: accepted Soft / N/A in docs/adr/0001-desktop-companion-scope.md; rubric still records missing daemon-tray QOL
- Mobile-presence decision remains undocumented and scores 0 at L117
- Apple notarization / Windows Authenticode: deferred in docs/ops/distribution.md (#66); keyless cosign checksum signing and fail-soft GitHub build attestations are implemented
