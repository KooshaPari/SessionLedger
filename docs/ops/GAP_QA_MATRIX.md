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
| C00 | 27/30 · partial | Loom-lite race_model landed; full loom/shuttle/miri and allocator profiling remain | `audit/.lane-c00/C00.md`; `tests/race_model.rs`; `docs/ops/concurrency-safety.md`; `.github/workflows/race-smoke.yml` | Add full permutation checkers and allocator profiling | 2026-07-14 |
| C01 | 27/30 · partial | `.env.example` + env-example CI gate landed; shell completions and quality-gate SHA pin remain | `audit/.lane-c01/C01.md`; `.env.example`; `scripts/env-example-check.ps1`; `.github/workflows/security.yml` | Pin reusable quality-gate.yml to commit SHA | 2026-07-14 |
| C02 | 24/30 · partial | Non-loopback API-key gate landed; IdP/OAuth beyond shared key remains | `crates/sl-daemon/src/http.rs`; `crates/sl-daemon/src/main.rs`; `docs/ops/local-trust-boundary.md` | Add IdP/OAuth if remote multi-user deploy is needed | 2026-07-14 |
| C03 | 36/36 · done | Agent-readiness pillars at max after role-form FR stories and feedback budgets | `audit/.lane-c03/C03.md`; `docs/functional_requirements.md`; `docs/ops/feedback-budgets.md`; `scripts/feedback-budget-check.ps1` | Keep FR/journey/budget artifacts current | 2026-07-14 |
| C04 | 24/30 · partial | Commit-signing ADR + blocking main-tip CI landed; maintainer 2FA is unproven | `audit/.lane-c04/C04.md`; `docs/adr/0004-commit-signing-policy.md`; `scripts/commit-signing-check.ps1` | Human records 2FA; machine-verify branch protection | 2026-07-13 |
| C05 | 29/30 · partial | Gated pprof smoke/docs landed; live Alertmanager webhook IDs, OTLP metrics, and real CPU sampler remain | `audit/.lane-c05/C05.md`; `scripts/pprof-smoke.ps1`; `docs/ops/observability.md`; `.github/workflows/ops-load.yml` | Wire live Slack/PagerDuty route IDs; replace profile 501 stub | 2026-07-14 |
| C06 | 24/30 · partial | SOURCE_DATE_EPOCH release repro landed; full L3 environment isolation and container provenance remain | `scripts/repro-check.ps1`; `.github/workflows/release.yml`; `docs/ops/reproducible-builds.md` | Close environment-isolation gaps; sign container images | 2026-07-14 |
| C07 | 27/30 · partial | jsonl_ingest fuzz + property expansion landed; exotic platforms and enforced perf-budget evidence remain | `tests/properties.rs`; `fuzz/fuzz_targets/jsonl_ingest.rs`; `scripts/flake-quarantine-apply.ps1`; `.github/workflows/cross-platform-build.yml` | Extend WBS-6.2 with enforced perf-budget evidence | 2026-07-14 |
| C08 | 24/30 · partial | Fourteen-fixture multi-lang corpus + anchor manifest landed; compression/token and larger families remain shallow | `audit/.lane-c08/C08.md`; `docs/ops/eval-manifest.json`; `docs/reference/conformance/README.md` | Grow larger task-family fixtures | 2026-07-14 |
| C09 | 39/45 · partial | Error-field association + Clear confirm landed; shell completions and live-daemon native parity remain | `audit/.lane-c09/C09.md`; `crates/sl-viewer/src/search_view.rs`; `tests/visual/harness/a11y.spec.js`; `docs/a11y/status-regions-and-native-smoke.md` | Complete live-daemon native WebView parity | 2026-07-14 |
| C10 | 36/36 · done | Visual identity pillars at max after splash goldens | `audit/.lane-c10/C10.md`; `tests/visual/golden/s1-launch-splash.png`; `tests/visual/golden/s1-launch-splash-light.png` | Keep splash/theme goldens current | 2026-07-14 |
| C11 | 32/45 · partial | Unsigned clean-host portable install smoke landed; platform signing deferred per ADR 0003 | `audit/.lane-c11/C11.md`; `docs/ops/distribution.md`; `scripts/installer-lifecycle-smoke.ps1` | Add credentials + signed production installer evidence | 2026-07-13 |

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
| PLAN-P6 | partial | Coverage, property, fuzz, and loom-lite race_model gates landed; full permutation checkers and enforced perf-budget gates remain | `tests/properties.rs`; `fuzz/`; `tests/race_model.rs`; CI repeats properties and runs bounded fuzz/race smoke | Finish full loom/shuttle and checked benchmark-budget evidence in WBS-6.2 | 2026-07-14 |
| PLAN-W8-B | done | Wave-24 result is 349/402 (87% B), meeting the B threshold | Independent audit-v38 result is at least 302/402 and >=75% | Hold B; pursue C11 signing implementation for higher grades | 2026-07-14 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

