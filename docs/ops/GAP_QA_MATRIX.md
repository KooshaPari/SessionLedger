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
| C00 | 27/30 · partial | Loom-lite race_model + allocation-budget companion landed; full loom/shuttle/miri and jemalloc/dhat remain | `audit/.lane-c00/C00.md`; `tests/race_model.rs`; `tests/allocation_budget.rs`; `docs/ops/allocation-budget.md`; `docs/ops/concurrency-safety.md` | Add full permutation checkers and jemalloc/dhat profiling | 2026-07-14 |
| C01 | 27/30 · partial | Quality-gate SHA pin + clap completions landed; i18n remains | `audit/.lane-c01/C01.md`; `.github/workflows/qgate.yml`; `crates/sl-daemon/completions/`; `scripts/install-sl-daemon-completions.ps1` | Consider i18n only if multi-locale becomes a goal | 2026-07-14 |
| C02 | 25/30 · partial | Process-wide `/api/*` rate limit landed; IdP/OAuth beyond shared key remains | `crates/sl-daemon/src/http.rs`; `crates/sl-daemon/src/main.rs`; `docs/ops/local-trust-boundary.md` | Add IdP/OAuth if remote multi-user deploy is needed | 2026-07-14 |
| C03 | 36/36 · done | Agent-readiness pillars at max after role-form FR stories and feedback budgets | `audit/.lane-c03/C03.md`; `docs/functional_requirements.md`; `docs/ops/feedback-budgets.md`; `scripts/feedback-budget-check.ps1` | Keep FR/journey/budget artifacts current | 2026-07-14 |
| C04 | 25/30 · partial | Renovate + branch-protection verify landed; maintainer 2FA is unproven | `audit/.lane-c04/C04.md`; `scripts/branch-protection-check.ps1`; `docs/ops/branch-protection.md`; `renovate.json` | Human records 2FA; optionally Strict branch-protection CI | 2026-07-14 |
| C05 | 29/30 · partial | Unix on-demand CPU pprof landed; live Alertmanager webhook IDs, OTLP metrics, and continuous agent remain | `audit/.lane-c05/C05.md`; `crates/sl-daemon/src/http.rs`; `scripts/pprof-smoke.ps1`; `docs/ops/observability.md` | Wire live Slack/PagerDuty route IDs; add continuous profiling agent | 2026-07-14 |
| C06 | 26/30 · partial | OCI verify-on-deploy landed; full L3 environment isolation and release-blocking OCI remain | `scripts/oci-cosign-verify.ps1`; `.github/workflows/release.yml`; `docs/ops/distribution.md`; `scripts/repro-check.ps1` | Close environment-isolation gaps; make OCI attest release-blocking | 2026-07-14 |
| C07 | 27/30 · partial | Enforced perf-budget gate landed (scored under C08 L74); exotic platforms remain | `tests/properties.rs`; `fuzz/fuzz_targets/jsonl_ingest.rs`; `.github/workflows/bench-gate.yml`; `docs/ops/perf-baseline.json` | Widen exotic platform targets beyond release/race matrices | 2026-07-14 |
| C08 | 24/30 · partial | Seventeen-fixture task-family corpus + enforced perf-budget landed; compression/token and cross-lang remain shallow | `audit/.lane-c08/C08.md`; `docs/ops/eval-manifest.json`; `docs/reference/conformance/README.md`; `.github/workflows/bench-gate.yml` | Grow production-scale families; cross-language adapters | 2026-07-14 |
| C09 | 39/45 · partial | Error-field association + Clear confirm landed; live-daemon native parity remain | `audit/.lane-c09/C09.md`; `crates/sl-viewer/src/search_view.rs`; `tests/visual/harness/a11y.spec.js`; `docs/a11y/status-regions-and-native-smoke.md` | Complete live-daemon native WebView parity | 2026-07-14 |
| C10 | 36/36 · done | Visual identity pillars at max after splash goldens | `audit/.lane-c10/C10.md`; `tests/visual/golden/s1-launch-splash.png`; `tests/visual/golden/s1-launch-splash-light.png` | Keep splash/theme goldens current | 2026-07-14 |
| C11 | 37/45 · partial | Reverse-proxy TLS configs + unsigned MSI/PKG + curl/irm + brew/winget manifests landed; platform signing deferred | `audit/.lane-c11/C11.md`; `packaging/caddy/Caddyfile`; `packaging/nginx/sessionledger.conf`; `scripts/install.sh`; `.github/workflows/release.yml` | Publish brew/winget; add credentials + signed production installer evidence | 2026-07-14 |

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
| PLAN-P6 | partial | Coverage, property, fuzz, loom-lite race_model, and enforced perf-budget gates landed; full permutation checkers remain | `tests/properties.rs`; `fuzz/`; `tests/race_model.rs`; `.github/workflows/bench-gate.yml`; `docs/ops/perf-baseline.json` | Finish full loom/shuttle permutation checkers in WBS-6.2 | 2026-07-14 |
| PLAN-W8-B | done | Wave-27 result is 358/402 (89% B), meeting the B threshold | Independent audit-v38 result is at least 302/402 and >=75% | Hold B; pursue C11 signing + published brew/winget + C04 2FA for higher grades | 2026-07-14 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

