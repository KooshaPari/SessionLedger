# audit-v38 Scorecard — SessionLedger

**Repo:** KooshaPari/SessionLedger
**Date:** 2026-07-13
**Repo-type profile:** CLI+daemon + desktop (sl-daemon + sl-viewer)
**Auditor:** cursor-w21-reaudit
**Commit audited:** 92b2792 (origin/main / Wave-21)

> Rubric SSOT: phenotype-org-audits/audit-v38

## Category Scores

| Cluster | Category | Pillars | Score (sum/max) | Pct | Grade | Notes |
|---------|----------|---------|:---------------:|:---:|:-----:|-------|
| C00 | Architecture + Module | L0-L9 | 24/30 | 80% | B | see audit/.lane-c00 |
| C01 | CI, DX, Observability | L10-L19 | 26/30 | 87% | B | see audit/.lane-c01 |
| C02 | Error handling, API, Governance | L20-L29 | 23/30 | 77% | B | see audit/.lane-c02 |
| C03 | Agent Readiness | L30 | 34/36 | 94% | A | see audit/.lane-c03 |
| C04 | Security | L31-L40 | 24/30 | 80% | B | see audit/.lane-c04 |
| C05 | Observability (deep) | L41-L50 | 27/30 | 90% | A | see audit/.lane-c05 |
| C06 | Supply Chain | L51-L60 | 23/30 | 77% | B | see audit/.lane-c06 |
| C07 | DX, QEng, Portability | L61-L70 | 26/30 | 87% | B | see audit/.lane-c07 |
| C08 | Eval Coverage | L71-L80 | 24/30 | 80% | B | see audit/.lane-c08 |
| C09 | Accessibility + UX | L81-L95 | 35/45 | 78% | C | see audit/.lane-c09 |
| C10 | Visual Identity | L96-L107 | 35/36 | 97% | A | see audit/.lane-c10 |
| C11 | Packaging + Distribution | L108-L122 | 32/45 | 71% | C | see audit/.lane-c11 |

## Overall

**Weighted overall score:** 83% · **Overall grade:** B

(Raw rubric total across all 12 clusters. Sum 333 / 402.)

## Wave-21 Delta

| Cluster | Before | After | Raw delta | Evidence-backed movement |
|---------|:------:|:-----:|:---------:|--------------------------|
| C01 | 26/30 | 26/30 | 0 | CI concurrency + auxiliary workflow SHA pins + CLI exit-code polish (#185); L10/L15 already pillar max — closes documented gaps |
| C02 | 23/30 | 23/30 | 0 | Durable append-only audit sink JSONL/SQLite backends (#182); L23 already pillar max — closes runtime audit-store gap |
| C04 | 23/30 | 24/30 | +1 | ADR 0004 commit-signing policy + blocking main-tip check (#180); L34 +1 |
| C05 | 26/30 | 27/30 | +1 | Scheduled weekday ops chaos/load smoke with readiness fault + process-kill recovery (#184); L50 +1 |
| C06 | 23/30 | 23/30 | 0 | SLSA materials-metadata contract fixture + provenance gate (#177); L53 already pillar max — partial L3 metadata evidence |
| C08 | 23/30 | 24/30 | +1 | Eval corpus grown to eleven fixtures with manifest anchors (#179); L80 +1 |
| C09 | 34/45 | 35/45 | +1 | In-viewer keyboard help overlay + a11y harness (#183); L81.13 +1 |
| C11 | 32/45 | 32/45 | 0 | Unsigned clean-host portable install smoke + evidence checklist (#178); L121 already pillar max — signed MSI still deferred |
| **Overall** | **329/402 (82% B)** | **333/402 (83% B)** | **+4** | Holds B |

## Headline Findings

- **Strongest:** C03 (94% A); C10 (97% A); C05 (90% A); C01/C07 (87% B)
- **Weakest:** C11 Packaging (71% C); C09 (78% C)
- **Wave-20 → Wave-21:** 82% B (329/402) → 83% B (333/402), +4 raw points
- **Remaining unpaid:** platform Authenticode/notarization implementation (C11 L112), maintainer 2FA attestation (C04 L36), full SLSA-L3 environment isolation (C06 L53), game-day chaos cadence (C05 L50)

## N/A / soft goals

- Harbor/agent-eval: docs/EVAL_SCOPE.md
- Tray/menubar auto-update: docs/adr/0001-desktop-companion-scope.md
- Mobile presence: docs/adr/0002-mobile-presence.md
- Platform Authenticode/notarization: deferred per docs/adr/0003-platform-code-signing.md
