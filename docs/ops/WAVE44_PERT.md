# Wave-44 PERT — SessionLedger (carry-forward)

Companion to [`WAVE44_SCOPE.md`](../../WAVE44_SCOPE.md) (repo root).
Predecessor: [`WAVE43_PERT.md`](WAVE43_PERT.md).

**Base:** `origin/main` @ `41829e8` (**396/402 · 98% A**)
**Width:** 5 parallel lanes · **Theme:** close-out the 6 unpaid residuals from W43
to push from **396/402 → 402/402 (100% A+)** without inflating raw score.

## Unpaid residual inventory (carry-over from W43 SCORECARD)

| ID | Cluster | Pillar | Description | Severity | Owner |
|----|---------|--------|-------------|----------|-------|
| R-1 | C00 | L7 | Process-level HTTP SSE soak under loom (closes L7) | deep-evidence | machine |
| R-2 | C00 | L8 | Windows allocator parity + always-on production rollout | deep-evidence + rollout | machine + human |
| R-3 | C11 | L111/L112 | brew/winget publish + Authenticode/notarization live keys | platform-signing | **human (keys)** |
| R-4 | C02 | L22 OR L24 | in-tree KMS (L22) OR multi-tenant PII redaction (L24) | policy | **human (policy)** |
| R-5 | C01 | L16 | viewer/CLI Fluent `.ftl` migration | policy + DX | machine + human |
| R-6 | C08 | L73 | production-scale corpus breadth | evidence | machine |

**Three lanes require human-gated evidence** (R-2 rollout, R-3 signing keys,
R-4 policy). **Three lanes are machine-executable** (R-1, R-5 tooling portion,
R-6) but R-5 also depends on localization sign-off.

## Activity table

| ID | Activity | Pred | Est (h) | Owner | Closes |
|----|----------|------|---------|-------|--------|
| W44-A | Scope PR (`WAVE44_SCOPE.md` + this PERT + CHANGELOG) | W43-D reaudit (PR #366) | 2 | machine | — |
| W44-B1 | w44-loom-sse-soak — loom permutation for HTTP SSE consumer graph | W44-A | 4 | machine | R-1 |
| W44-B2 | w44-windows-allocator-prod — Windows jemalloc parity + canary rollout | W44-A | 6 | machine + human | R-2 |
| W44-B3 | w44-brew-winget-signing — brew/winget publish + Authenticode live | W44-A | 4 | **human (keys)** | R-3 |
| W44-B4 | w44-pii-or-kms — in-tree KMS OR multi-tenant PII redaction (pick one) | W44-A | 6 | **human (policy)** | R-4 |
| W44-B5 | w44-fluent-migration — viewer/CLI `.ftl` extraction (tooling part) | W44-A | 3 | machine | R-5 (partial) |
| W44-B6 | w44-corpus-breadth — production-scale corpus + replay fixtures | W44-A | 3 | machine | R-6 |
| W44-C | Merge B1–B6 sequentially (lowest conflict first) | B1–B6 | 3 | machine | — |
| W44-D | Full reaudit + traceability refresh | W44-C | 3 | machine | — |
| W44-E | Org mirror PR to phenotype-org-audits (skeleton) | W44-D | 2 | human (target archived) | — |

**Parallel width:** 6 (B1–B6). **Critical path:** A → **B4** (policy decision
slows both branches) → C → D (~25h nominal; gated on human R-3/R-4).

## Merge order (lowest conflict risk first)

1. **w44-loom-sse-soak** — `tests/loom_sse.rs`, sl-daemon graph touch only
2. **w44-fluent-migration** — viewer/CLI string extraction; new `locales/*.ftl`
3. **w44-corpus-breadth** — `tests/corpus/` fixtures + new replay harness
4. **w44-windows-allocator-prod** — `Cargo.toml` features + `jemalloc.md` + rollout
5. **w44-brew-winget-signing** — `.github/workflows/release.yml` + signing config
6. **w44-pii-or-kms** — `src/domain/redact.rs` OR `crates/sl-kms/` (largest diff)

## Lane detail (acceptance stubs)

| Lane | Key files | Acceptance |
|------|-----------|------------|
| w44-loom-sse-soak | `tests/loom_sse.rs`, sl-daemon graph | Loom permutation green; SSE consumer fanout exercised under load |
| w44-windows-allocator-prod | `Cargo.toml`, `jemalloc.md`, rollout flag | Windows parity verified; prod canary green for 7d |
| w44-brew-winget-signing | `.github/workflows/release.yml`, signing keys | brew tap live; winget PR merged; Authenticode green |
| w44-pii-or-kms | `src/domain/redact.rs` OR `crates/sl-kms/` | One of (L22 or L24) closed with evidence; other tracked |
| w44-fluent-migration | `locales/en-US.ftl`, viewer/CLI string tables | `.ftl` adopted for CLI strings; viewer partial; human sign-off pending |
| w44-corpus-breadth | `tests/corpus/*.jsonl`, `tests/replay_breadth.rs` | 10× current fixture count; replay coverage 80%+ |

## Score disposition (target)

**Pre-W44:** 396/402 (98% A)
**Target post-W44:** 402/402 (100% A+) **iff** all six residuals close.
**Realistic post-W44:** 398–401/402 (99–100% A+); R-3 and R-4 may slip if
human-gated keys/policy not received in window.

## Decision points (human-owned)

- **D-W44-1:** Pick R-4 branch (L22 KMS vs L24 PII redaction). Cannot close
  both in this wave; recommend L22 (smaller blast radius).
- **D-W44-2:** Accept partial W44-B5 close (machine-tooling only; viewer
  localization deferred to W45).
- **D-W44-3:** R-3 signing keys availability. If not received within 7d of
  W44-B3 start, wave downgrades R-3 to partial and defers to W45.

## Risk register

| Risk | Impact | Mitigation |
|------|--------|------------|
| Human-gated R-3 keys slip | R-3 partial close | Document partial in SCORECARD; defer to W45 |
| Human-gated R-4 policy unclear | R-4 lane block | Surface D-W44-1 early; pick L22 |
| R-2 canary rollback in prod | R-2 partial close | Canary 7d window; auto-revert on error rate spike |
| Loc tooling merge conflicts with viewer | W44-B5 partial | Land tooling PR first; viewer PR after viewer owner review |
| Corpus fixture bloat (>50MB) | CI timeout | git-lfs + selective replay in nightly only |

## Org mirror (W44-E)

`phenotype-org-audits` archived wave mirror pattern from W43 (#170). W44-E
emits a single skeleton PR containing:
- `audits/SessionLedger/wave-44/SCORECARD.md`
- `audits/SessionLedger/wave-44/DELTA.md`
- `audits/SessionLedger/wave-44/EVIDENCE.md`

W44-E is `human` because target repo `phenotype-org-audits` requires manual
approval per W43-E precedent.

## Acceptance criteria for W44 closure

- [ ] B1–B6 PRs all MERGED (or partial with human sign-off)
- [ ] SCORECARD.md updated to W44 score
- [ ] TRACEABILITY.json updated (`updated`, `commit`, `wave`)
- [ ] GAP_QA_MATRIX.md updated for any closed residual
- [ ] CHANGELOG.md Unreleased Changed entry for W44
- [ ] Org mirror PR opened (W44-E)

**Owner:** machine (W44-A through W44-D, plus partial B5/B6) + human (R-3 keys,
R-4 policy decision, W44-E approval).
