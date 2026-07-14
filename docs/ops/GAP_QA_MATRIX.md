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
| C00 | 24/30 · partial | Cancellation, profiling/race/RSS gates | `audit/.lane-c00/C00.md`; `crates/sl-daemon/src/main.rs`; `crates/sl-daemon/src/http.rs`; `src/ports/sqlite_memory.rs` | Add CancellationToken and formal concurrency checkers | 2026-07-13 |
| C01 | 26/30 · partial | Mutable auxiliary actions, no CI concurrency, uneven error/CLI polish | `audit/.lane-c01/C01.md`; all workflow actions SHA-pinned; built-viewer a11y | Pin remaining actions and add CI concurrency | 2026-07-12 |
| C02 | 23/30 · partial | Local-only binding, ingest admission, JSON errors, and actor/action events landed; authenticated remote access and append-only retention remain out of scope | `crates/sl-daemon/src/http.rs` limit/envelope/audit tests; `main.rs` loopback policy test; `docs/ops/local-trust-boundary.md` | Add authenticated proxy policy and durable append-only audit sink before any remote exposure | 2026-07-12 |
| C03 | 34/36 · partial | Role-form FR stories and measured feedback budget remain; journey catalog and .env.example landed | `audit/.lane-c03/C03.md`; traceability lint; journey-to-test mapping | Keep trace artifacts current; add named user journeys | 2026-07-12 |
| C04 | 23/30 · partial | Maintainer 2FA is unproven; commit signing policy incomplete | `audit/.lane-c04/C04.md`; `.github/workflows/security.yml`; `.pre-commit-config.yaml` | Human records 2FA; enforce signed commits | 2026-07-13 |
| C05 | 26/30 · partial | OTLP metrics, live alert routing, live dashboard proof, scheduled chaos/load remain | `audit/.lane-c05/C05.md`; `docs/ops/dashboards/sessionledger-red.json`; `scripts/dashboard-provision-check.ps1`; `docs/ops/observability.md` | Add scheduled operational tests and live alert routing | 2026-07-13 |
| C06 | 23/30 · partial | Per-matrix attest + contract gate landed; cross-host/Windows proof, SLSA-L3 material metadata, blocking container provenance remain incomplete | `scripts/provenance-contract-check.ps1`; `.github/workflows/provenance-contract.yml`; `.github/workflows/release.yml`; `docs/ops/reproducible-builds.md` | Close SLSA-L3 material-metadata gaps; sign container images | 2026-07-13 |
| C07 | 26/30 · partial | Exotic platform targets and enforced perf-budget evidence remain | `scripts/flake-quarantine-apply.ps1`; `.github/workflows/cross-platform-build.yml`; `docs/ops/cross-platform-ci.md`; `docs/ops/flake-tracker.md` | Extend WBS-6.2 with enforced perf-budget evidence | 2026-07-13 |
| C08 | 23/30 · partial | Eval reproducibility manifest gate landed; compression/token and cross-language parity remain shallow | `audit/.lane-c08/C08.md`; `docs/ops/eval-manifest.json`; `scripts/eval-repro-check.ps1` | Grow corpus scale and cross-language adapters | 2026-07-13 |
| C09 | 34/45 · partial | Native WebView/live-data, cognitive, help, and efficiency coverage remain incomplete | `audit/.lane-c09/C09.md`; `tests/visual/harness/responsive.spec.js` | Add native integration evidence | 2026-07-13 |
| C10 | 35/36 · partial | First-run empty variant and remaining R2/R3 error goldens | `audit/.lane-c10/C10.md`; `tests/visual/golden/e2-history-empty.png`; `tests/visual/golden/e4-search-empty.png`; `tests/visual/golden/r1-search-error.png`; `tests/visual/harness/visual.spec.js` | Add first-run empty + R2/R3 error variant goldens | 2026-07-13 |
| C11 | 32/45 · partial | Platform signing deferred per ADR 0003; native installer signing and signed clean-host install remain | `audit/.lane-c11/C11.md`; `docs/adr/0003-platform-code-signing.md`; `scripts/installer-lifecycle-smoke.ps1` | Add credentials + signed production installer evidence | 2026-07-13 |

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
| PLAN-P6 | partial | Coverage, property, and fuzz-smoke gates landed; race and enforced perf-budget gates remain | `tests/properties.rs`; `fuzz/`; CI repeats properties and runs a bounded fuzz smoke | Finish race and checked benchmark-budget evidence in WBS-6.2 | 2026-07-12 |
| PLAN-W8-B | done | Wave-20 result is 329/402 (82% B), meeting the B threshold | Independent audit-v38 result is at least 302/402 and >=75% | Hold B; pursue C11 signing implementation for higher grades | 2026-07-13 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

