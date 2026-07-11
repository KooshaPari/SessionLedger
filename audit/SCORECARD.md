# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-10
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w6-reaudit
**Commit audited:** aa63227 (origin/main / Wave-6)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 22/30 | 73% | C | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 21/30 | 70% | C | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 18/30 | 60% | C | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 31/36 | 86% | B | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 19/30 | 63% | C | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 18/30 | 60% | C | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 17/30 | 57% | D | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 18/30 | 60% | C | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 18/30 | 60% | C | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 24/45 | 53% | D | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 25/36 | 69% | C | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 25/45 | 56% | D | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 64% · **Overall grade:** C

(Raw rubric total across all 12 clusters. Sum 256 / 402.)

## Wave-6 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C00 | 19/30 | 22/30 | +3 | Production observability surface completed; Criterion hot-path benchmarks seeded budgets/profiling |
| C01 | 21/30 | 21/30 | 0 | More goldens and an accessible filter label strengthen already-scored criteria |
| C02 | 18/30 | 18/30 | 0 | JSON logs are operational telemetry, not an append-only actor/action audit log |
| C03 | 31/36 | 31/36 | 0 | Continuation features strengthen guardrails but do not close FR story syntax/journey-map gaps |
| C04 | 19/30 | 19/30 | 0 | No Wave-6 security acceptance criterion closed |
| C05 | 17/30 | 18/30 | +1 | HTTP traceparent correlation is partial; JSON logs and Prometheus metrics strengthen existing full scores |
| C06 | 17/30 | 17/30 | 0 | Locked local packaging/checksum verification is not reproducible-build proof or SLSA provenance |
| C07 | 18/30 | 18/30 | 0 | Added tests/packaging do not close property, fuzz, flake, or PR cross-platform gaps |
| C08 | 14/30 | 18/30 | +4 | Criterion suite closes benchmark-suite criterion and partially covers micro/macro tiers; no load/perf gate |
| C09 | 24/45 | 24/45 | 0 | Filter naming helps, but automated WCAG/screen-reader/viewport evidence remains absent |
| C10 | 25/36 | 25/36 | 0 | No Wave-6 visual-identity acceptance criterion closed |
| C11 | 23/45 | 25/45 | +2 | Documented checksum-verified manual update path; installer/channel/tray/mobile/signing gaps remain |

## Headline Findings

- **Strongest:** C03 Agent Readiness (86% B); C00 Architecture (73% C); C01 CI (70% C)
- **Weakest:** C09 Accessibility (53% D); C11 Packaging (56% D); C06 Supply Chain (57% D)
- **Wave-2 delta:** 46% D → 60% C (target ≥60% C)
- **Wave-3 delta:** 60% C → 61% C (+5 raw points); optional OTLP traces, keyless checksum signing, reduced-motion, and a visual harness landed
- **Wave-6 re-audit delta:** 61% C → 64% C (+10 raw points); this is measurable C-grade movement, not a B
- **Why not 75% B:** the current evidence still lacks native installers/signing, automated a11y, load/chaos testing, dashboards, and multiple provenance/reproducibility controls
- **Agent-readiness verdict (C03):** Agents can orient, claim PLAN tasks, and use FR catalog
- **Time-3 verdict (C11):** Local Windows packaging and checksum-verified Unix install/update drafts landed; package channels and native installers/signing remain incomplete

## N/A / soft goals

- Harbor/agent-eval: documented N/A in docs/EVAL_SCOPE.md (#68)
- Dioxus 0.7 migration and sha2 0.11 upgrade landed after Wave-3; neither closes a separate rubric point by itself
- Tray/menubar and background auto-update: accepted Soft / N/A in docs/adr/0001-desktop-companion-scope.md; rubric still records missing daemon-tray QOL
- Mobile-presence decision remains undocumented and scores 0 at L117
- Apple notarization / Windows Authenticode: deferred in docs/ops/distribution.md (#66); keyless cosign checksum signing is implemented
