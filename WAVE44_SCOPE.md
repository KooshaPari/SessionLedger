# Wave-44 scope — SessionLedger audit-v38 (close-out, 396/402 → 402/402 target)

**Base:** `origin/main` @ `41829e8` (Wave-43 closure #362 · **396/402 · 98% A**)
**Method:** Wave-43 widened evidence; Wave-44 closes the remaining 6 raw points
across 3 machine-executable lanes and 3 human-gated lanes.
**Auditor posture:** close-out wave; if all 6 residuals close, target **402/402
· 100% A+** without creds-dependent inflation.

Companion PERT: [`docs/ops/WAVE44_PERT.md`](docs/ops/WAVE44_PERT.md)

**Source:** Wave-43 SCORECARD headline at `41829e8` + Wave-43-D reaudit close
(PR #366).

---

## Top unpaid gaps (396/402 → 402/402 closure targets)

**6 raw points** remain across C00, C01, C02, C08, C11 from Wave-43. Wave-44
selects **6 lanes**, three machine-actionable and three human-gated.

| Rank | ID | Class | Gap | Pillar / cluster | Selected lane |
|:----:|----|-------|-----|------------------|---------------|
| **1** | GAP-W43-STAB-01 | **Concurrency depth** | Process-level HTTP SSE consumer fanout outside loom | C00 L7 | **w44-loom-sse-soak** |
| **2** | GAP-W43-STAB-02 | **Allocator policy** | Windows allocator parity + prod canary rollout | C00 L8 | **w44-windows-allocator-prod** |
| **3** | GAP-W43-PKG-01 | **Packaging signing** | brew/winget publish + Authenticode/notarization live keys | C11 L111/L112 | **w44-brew-winget-signing** |
| **4** | GAP-W43-API-01 | **API governance** | In-tree KMS (L22) OR multi-tenant PII redaction (L24) | C02 L22/L24 | **w44-pii-or-kms** |
| **5** | GAP-W43-DX-02 | **i18n** | Viewer/CLI Fluent `.ftl` migration complete | C01 L16 | **w44-fluent-migration** |
| **6** | GAP-W43-EVAL-01 | **Eval coverage** | Production-scale corpus breadth | C08 L73 | **w44-corpus-breadth** |

### Lane ownership breakdown

| Owner | Lanes | Why human-gated |
|-------|-------|-----------------|
| `machine` | w44-loom-sse-soak, w44-fluent-migration (tooling), w44-corpus-breadth | n/a |
| `machine + human` | w44-windows-allocator-prod, w44-fluent-migration (viewer portion) | rollout window + loc sign-off |
| `human (keys)` | w44-brew-winget-signing | Authenticode + notarization secrets |
| `human (policy)` | w44-pii-or-kms | picks L22 KMS vs L24 PII redaction |

### Decision points (human-owned)

- **D-W44-1:** Pick R-4 branch (L22 KMS vs L24 PII redaction). Recommend L22
  (smaller blast radius; L24 requires multi-tenant threat model).
- **D-W44-2:** R-3 keys availability window. If not received within 7d of W44-B3
  start, downgrade R-3 to partial close and defer to W45.
- **D-W44-3:** Accept partial W44-B5 close (machine-tooling only; viewer
  localization deferred to W45 if loc sign-off slips).

### Secondary gaps (deferred or alternate lanes)

| ID | Gap | Notes | Alternate lane |
|----|-----|-------|----------------|
| GAP-W43-C04-01 | C04 SBOM pillar residual (3 raw pts) | Schema gate incomplete; close in W45 | w45-sbom-pillar-close |
| GAP-W43-C09-01 | Viewer accessibility audit (C09 residual) | Pre-existing; covered by W43-B4 | (covered) |

## Wave-44 acceptance

- All B1–B6 PRs merged (or partial with human sign-off)
- SCORECARD.md refreshes to W44 score (target 402/402; realistic 398–401)
- TRACEABILITY.json updated (`updated: 2026-07-XX`, `commit: <W44-tip>`,
  `wave: Wave-44`)
- GAP_QA_MATRIX.md updated for any closed residual
- CHANGELOG.md Unreleased entry for W44
- Org mirror PR opened (W44-E; human-gated approval)

## Carry-over history

- Wave-41: 372/402 → 375/402 (#163/#164)
- Wave-42: 375/402 → 396/402 (#165, #169, #170)
- Wave-43: 396/402 → 396/402 (conservative hold; 5 impl lanes, #170–#362)
- Wave-44: **target** 396/402 → 402/402 (close-out)
