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
| C00 | 28/30 · partial | Enforced p95 SoftLatencyCheck + soft Miri + soft dhat alloc-profile landed; full loom/shuttle/blocking miri and jemalloc/production allocator remain | `audit/.lane-c00/C00.md`; `scripts/bench-gate.ps1`; `docs/ops/perf-baseline.json`; `.github/workflows/miri-smoke.yml`; `docs/ops/alloc-profile.md`; `scripts/alloc-profile-check.ps1` | Add full permutation checkers and jemalloc/production allocator | 2026-07-14 |
| C01 | 27/30 · partial | Quality-gate SHA pin + clap completions landed; i18n remains | `audit/.lane-c01/C01.md`; `.github/workflows/qgate.yml`; `crates/sl-daemon/completions/`; `scripts/install-sl-daemon-completions.ps1` | Consider i18n only if multi-locale becomes a goal | 2026-07-14 |
| C02 | 28/30 · partial | Privacy hygiene SSOT + crypto inventory landed; IdP/OAuth beyond shared key, KMS/at-rest, and in-tree PII redaction remain | `docs/ops/privacy-hygiene.md`; `scripts/privacy-hygiene-check.ps1`; `docs/ops/crypto-inventory.md`; `crates/sl-daemon/src/resilience.rs`; `docs/ops/local-trust-boundary.md` | Add IdP/OAuth if remote multi-user deploy is needed; automated redaction if exports leave host | 2026-07-14 |
| C03 | 36/36 · done | Agent-readiness pillars at max after role-form FR stories and feedback budgets | `audit/.lane-c03/C03.md`; `docs/functional_requirements.md`; `docs/ops/feedback-budgets.md`; `scripts/feedback-budget-check.ps1` | Keep FR/journey/budget artifacts current | 2026-07-14 |
| C04 | 25/30 · partial | Sandbox boundary checklist + Renovate + branch-protection verify landed; maintainer 2FA and seccomp/no-net remain | `docs/ops/sandbox-boundary.md`; `scripts/sandbox-boundary-check.ps1`; `scripts/branch-protection-check.ps1`; `docs/ops/branch-protection.md`; `renovate.json` | Human records 2FA; add seccomp/no-net CI sandbox | 2026-07-14 |
| C05 | 29/30 · partial | Unix on-demand CPU pprof + soft continuous agent stub landed; live Alertmanager webhook IDs, OTLP metrics, and push backends remain | `audit/.lane-c05/C05.md`; `docs/ops/continuous-profiling.md`; `scripts/continuous-profiling-agent.ps1`; `scripts/pprof-smoke.ps1`; `docs/ops/observability.md` | Wire live Slack/PagerDuty route IDs; add Pyroscope/OTLP push | 2026-07-14 |
| C06 | 26/30 · partial | OCI verify-on-deploy + release-blocking OCI when GHCR/OIDC credentials present landed; full L3 environment isolation and unconditional release-blocking OCI remain | `scripts/oci-cosign-verify.ps1`; `.github/workflows/release.yml`; `docs/ops/distribution.md`; `scripts/repro-check.ps1` | Close environment-isolation gaps; require OCI attest on all canonical releases | 2026-07-14 |
| C07 | 28/30 · partial | Test pyramid SSOT + exotic aarch64/musl cargo check + enforced perf-budget landed; mutation breadth remains | `docs/ops/test-pyramid.md`; `scripts/test-pyramid-check.ps1`; `tests/properties.rs`; `.github/workflows/cross-platform-build.yml`; `.github/workflows/bench-gate.yml` | Expand mutation/property depth beyond pyramid SSOT | 2026-07-14 |
| C08 | 26/30 · partial | Twenty-fixture corpus + compression eval gate + per-fixture token-burn ledger smoke landed; cross-lang remain shallow | `audit/.lane-c08/C08.md`; `docs/ops/token-burn.json`; `scripts/token-burn-check.ps1`; `tests/compression_eval.rs`; `.github/workflows/bench-gate.yml` | Grow production-scale families; cross-language adapters | 2026-07-14 |
| C09 | 42/45 · partial | Overlay Escape consistency + expanded Cmd+K palette + live-daemon native attach evidence landed; native WebView parity gaps remain | `docs/a11y/overlay-escape.md`; `audit/.lane-c09/C09.md`; `crates/sl-viewer/src/command_palette.rs`; `scripts/record-native-webview-smoke.ps1` | Deepen native parity; undo beyond confirm dialogs | 2026-07-14 |
| C10 | 36/36 · done | Visual identity pillars at max after splash goldens | `audit/.lane-c10/C10.md`; `tests/visual/golden/s1-launch-splash.png`; `tests/visual/golden/s1-launch-splash-light.png` | Keep splash/theme goldens current | 2026-07-14 |
| C11 | 37/45 · partial | Brew/winget fill script + reverse-proxy + unsigned MSI/PKG + signing readiness checklist landed; live tap/winget publish and platform signing remain | `audit/.lane-c11/C11.md`; `docs/ops/signing-readiness.md`; `scripts/signing-readiness-check.ps1`; `docs/ops/brew-winget-publish.md`; `.github/workflows/release.yml` | Publish brew/winget; add credentials + signed production installer evidence | 2026-07-14 |

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
| PLAN-P6 | partial | Coverage, property, fuzz, loom-lite race_model, enforced perf-budget, and enforced p95 latency gates landed; full permutation checkers remain | `tests/properties.rs`; `fuzz/`; `tests/race_model.rs`; `.github/workflows/bench-gate.yml`; `docs/ops/perf-baseline.json` | Finish full loom/shuttle permutation checkers in WBS-6.2 | 2026-07-14 |
| PLAN-W8-B | done | Wave-30 result is 368/402 (92% A), meeting the B threshold | Independent audit-v38 result is at least 302/402 and >=75% | Hold A; pursue C11 signing + published brew/winget + C04 2FA for higher grades | 2026-07-14 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

