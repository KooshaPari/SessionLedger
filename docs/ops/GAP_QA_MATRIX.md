# Gap audit / QA matrix

This matrix is the human-readable QA view of
[`TRACEABILITY.json`](TRACEABILITY.json). Status values are limited to `done`,
`partial`, `todo`, `blocked`, and `na`. Scores are copied from
[`audit/SCORECARD.md`](../../audit/SCORECARD.md) and MUST NOT be changed without
a re-audit. Agents changing a row must update `status_updated`, the linked WBS
package, and the JSON mirror in the same change.

## audit-v38 clusters

| ID | Current score / status | Gap | Acceptance test / evidence | Next action | status_updated |
|---|---|---|---|---|---|
| C00 | 29/30 · partial | Blocking loom permutation SelfCheck + `cargo test loom` landed; soft shuttle/Miri and jemalloc/production allocator remain | `audit/.lane-c00/C00.md`; `scripts/loom-permutation-check.ps1`; `.github/workflows/loom-permutation.yml`; `tests/loom_model.rs`; `.github/workflows/miri-smoke.yml`; `docs/ops/shuttle-soft.md` | Promote shuttle/Miri to blocking; add jemalloc/production allocator | 2026-07-17 |
| C01 | 29/30 · partial | Quality-gate SHA pin + clap completions + soft `es` i18n catalog landed; Fluent/ICU multi-locale remains | `audit/.lane-c01/C01.md`; `locales/es.json`; `src/i18n.rs`; `docs/ops/i18n.md`; `scripts/i18n-check.ps1` | Wire Fluent/ICU and migrate viewer/CLI strings through catalogs | 2026-07-17 |
| C02 | 29/30 · partial | Privacy hygiene SSOT + crypto inventory + Phase-0 KMS/at-rest deferred guidance + PII redaction stub landed; IdP/OAuth beyond shared key, in-tree KMS, and production PII redaction remain | `docs/ops/privacy-hygiene.md`; `scripts/privacy-hygiene-check.ps1`; `docs/ops/pii-redaction.md`; `docs/ops/crypto-inventory.md`; `crates/sl-daemon/src/resilience.rs`; `docs/ops/local-trust-boundary.md` | Add IdP/OAuth if remote multi-user deploy is needed; automated redaction if exports leave host | 2026-07-14 |
| C03 | 36/36 · done | Agent-readiness pillars at max after role-form FR stories and feedback budgets | `audit/.lane-c03/C03.md`; `docs/functional_requirements.md`; `docs/ops/feedback-budgets.md`; `scripts/feedback-budget-check.ps1` | Keep FR/journey/budget artifacts current | 2026-07-14 |
| C04 | 27/30 · partial | CVE feed GHSA+OSV+NVD SSOT + blocking sandbox-boundary SelfCheck + Renovate + branch-protection verify landed; maintainer 2FA and hard rootless/no-net remain | `docs/ops/cve-feed-subscription.md`; `scripts/cve-feed-check.ps1`; `docs/ops/sandbox-boundary.md`; `scripts/sandbox-boundary-check.ps1`; `.github/workflows/security.yml`; `scripts/branch-protection-check.ps1`; `docs/ops/branch-protection.md`; `renovate.json` | Human records 2FA; add rootless/no-net hard CI | 2026-07-17 |
| C05 | 30/30 · done | OTLP traces + blocking OTLP metrics gRPC export + soft continuous agent stub landed; live Alertmanager webhook IDs and Pyroscope push remain | `audit/.lane-c05/C05.md`; `crates/sl-daemon/src/otel_metrics.rs`; `scripts/otlp-metrics-check.ps1`; `docs/ops/continuous-profiling.md`; `scripts/pprof-smoke.ps1` | Wire live Slack/PagerDuty route IDs; add Pyroscope push | 2026-07-17 |
| C06 | 29/30 · partial | Unconditional release-blocking OCI cosign + partial SLSA L3 isolation checklist landed; full reusable-workflow L3 and source provenance breadth remain | `scripts/oci-cosign-verify.ps1`; `.github/workflows/release.yml`; `docs/ops/distribution.md`; `scripts/slsa-isolation-check.ps1`; `docs/ops/hermetic-builds.md`; `scripts/repro-check.ps1` | Close reusable-workflow L3 + L59 source provenance gaps | 2026-07-17 |
| C07 | 29/30 · partial | Lifecycle FSM property tests + test pyramid SSOT + exotic aarch64/musl cargo check + enforced perf-budget landed; mutation breadth remains | `docs/ops/test-pyramid.md`; `scripts/test-pyramid-check.ps1`; `tests/properties.rs`; `.github/workflows/cross-platform-build.yml`; `.github/workflows/bench-gate.yml` | Expand mutation depth beyond property FSM coverage | 2026-07-14 |
| C08 | 29/30 · partial | Cross-language fixture parity + Python/Go/TypeScript OKF adapter stubs landed; production-scale corpus breadth remains | `docs/ops/cross-language-parity.md`; `scripts/cross-language-parity-check.ps1`; `adapters/typescript/okf_adapter.ts`; `adapters/go/main.go`; `audit/.lane-c08/C08.md` | Grow production-scale families | 2026-07-17 |
| C09 | 45/45 · done | Design-token single source + overlay Escape + expanded Cmd+K palette + ErrorState non-color cues landed; native WebView parity gaps remain soft | `docs/a11y/design-tokens.md`; `docs/a11y/overlay-escape.md`; `audit/.lane-c09/C09.md`; `crates/sl-viewer/src/async_states.rs`; `crates/sl-viewer/src/tokens.rs`; `scripts/record-native-webview-smoke.ps1` | Deepen native parity; undo beyond confirm dialogs | 2026-07-17 |
| C10 | 36/36 · done | Visual identity pillars at max after splash goldens | `audit/.lane-c10/C10.md`; `tests/visual/golden/s1-launch-splash.png`; `tests/visual/golden/s1-launch-splash-light.png` | Keep splash/theme goldens current | 2026-07-14 |
| C11 | 41/45 · partial | ADR 0005 edge N/A + versioning policy + brew/winget fill script + reverse-proxy + unsigned MSI/PKG + signing readiness checklist landed; live tap/winget publish and platform signing remain | `audit/.lane-c11/C11.md`; `docs/adr/0005-no-serverless-edge.md`; `docs/ops/versioning-policy.md`; `docs/ops/signing-readiness.md`; `scripts/signing-readiness-check.ps1`; `docs/ops/brew-winget-publish.md`; `.github/workflows/release.yml` | Publish brew/winget; add credentials + signed production installer evidence | 2026-07-17 |

## Functional requirements

| ID | Current score / status | Gap | Acceptance test / evidence | Next action | status_updated |
|---|---|---|---|---|---|
| FR-001 | done | None in FR scope | `crates/sl-daemon/src/etl.rs`; `src/ingestion/`; pipeline tests | Preserve adapter fixtures when formats change | 2026-07-12 |
| FR-002 | done | None in FR scope | `crates/sl-daemon/src/validation.rs`; `POST /api/ingest`; `sl validate` | Keep conformance corpus synchronized | 2026-07-12 |
| FR-003 | done | None in FR scope | `GET /api/bundles`; `GET /api/search`; filter/tag tests | Preserve query compatibility | 2026-07-12 |
| FR-004 | done | None in FR scope | `GET /api/replay/:id`; `crates/sl-daemon/tests/sse_bridge.rs` | Preserve SSE contract | 2026-07-12 |
| FR-005 | done | None in FR scope | `GET /api/metrics`; metrics unit tests | Preserve product metrics contract beside RED metrics | 2026-07-12 |
| FR-006 | done | None in FR scope | `sl archive`; `sl restore`; `crates/sl-daemon/src/archive.rs` | Retain gzip roundtrip tests | 2026-07-12 |
| FR-007 | done | None in FR scope | `crates/sl-viewer/src/timeline.rs` | Keep viewer smoke coverage | 2026-07-12 |
| FR-008 | done | None in FR scope | `crates/sl-viewer/src/search_view.rs`; `GET /api/search` | Keep search UI/API mapping | 2026-07-12 |
| FR-009 | done | None in FR scope | `crates/sl-viewer/src/replay_view.rs`; replay SSE tests | Keep replay UI/API mapping | 2026-07-12 |
| FR-010 | done | None in FR scope | `crates/sl-viewer/src/live_feed.rs`; `GET /api/stream` | Retain reconnect and roundtrip evidence | 2026-07-12 |
| FR-011 | done | None: unfinished tab shipped in #97 | `src/domain/worklog.rs`; `crates/sl-viewer/src/unfinished_tab.rs`; T-024, T-036 | Preserve detector/projection/view tests | 2026-07-12 |
| FR-012 | done | None in FR scope | `src/domain/bundle.rs`; `src/distill/`; `src/inject.rs` | Preserve acceptance-slice injection gate | 2026-07-12 |
| FR-013 | done | None in FR scope | `tests/okf_roundtrip.rs`; `tests/okf_golden.rs`; conformance fixtures | Add fixtures for every new corpus shape | 2026-07-12 |
| FR-014 | done | None in FR scope | `/healthz`; `/readyz`; `process-compose.yaml`; `docs/ops/runbook.md` | Keep liveness/readiness semantics distinct | 2026-07-12 |
| FR-015 | done | FR scope is observability surfaces; deeper C05 operationalization remains | `crates/sl-daemon/src/otel.rs`; `GET /metrics`; `docs/ops/dashboards/sessionledger-red.json`; `docs/ops/observability.md` | Track provisioning/routing gaps under C05, not by reopening this FR | 2026-07-12 |

## PLAN and roadmap residual themes

These are residual themes only; they do not create new PLAN T-IDs.

| ID | Current score / status | Gap | Acceptance test / evidence | Next action | status_updated |
|---|---|---|---|---|---|
| PLAN-P3 | partial | LLM-backed intent extraction and `curate.py` convergence remain | `docs/DESIGN.md` §6-7; adapter contract tests with provenance | Claim WBS-3.2 after cross-repo destination approval | 2026-07-12 |
| PLAN-P4 | partial | context-mode FTS recall and explicit TUI scope decision remain | `docs/DESIGN.md` §3, §7; recall E2E or accepted `na` decision | Human decides TUI; machine implements approved recall boundary | 2026-07-12 |
| PLAN-P6 | partial | Coverage, property (incl. lifecycle FSM), fuzz, loom-lite race_model, blocking loom permutation, enforced perf-budget, and enforced p95 latency gates landed; full shuttle/miri blocking remain | `tests/properties.rs`; `fuzz/`; `tests/race_model.rs`; `tests/loom_model.rs`; `.github/workflows/loom-permutation.yml`; `.github/workflows/bench-gate.yml`; `docs/ops/perf-baseline.json` | Promote shuttle/Miri blocking in WBS-6.2 | 2026-07-17 |
| PLAN-W8-B | done | Wave-36 result is 389/402 (97% A), meeting the A threshold | Independent audit-v38 result is at least 362/402 and >=90% | Hold A; pursue C11 signing + published brew/winget + C04 2FA + full SLSA-L3 for higher grades | 2026-07-17 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

