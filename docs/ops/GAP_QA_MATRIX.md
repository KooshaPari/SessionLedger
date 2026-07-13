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
| C00 | 22/30 · partial | OpenAPI/idempotency, cancellation, durable schema, profiling/race/RSS gates | `audit/.lane-c00/C00.md`; architecture tests and new gates pass | Prioritize API contract and P6 hardening evidence | 2026-07-12 |
| C01 | 26/30 · partial | Mutable auxiliary actions, no CI concurrency, uneven error/CLI polish | `audit/.lane-c01/C01.md`; all workflow actions SHA-pinned; built-viewer a11y | Pin remaining actions and add CI concurrency | 2026-07-12 |
| C02 | 23/30 · partial | Local-only binding, ingest admission, JSON errors, and actor/action events landed; authenticated remote access and append-only retention remain out of scope | `crates/sl-daemon/src/http.rs` limit/envelope/audit tests; `main.rs` loopback policy test; `docs/ops/local-trust-boundary.md` | Add authenticated proxy policy and durable append-only audit sink before any remote exposure | 2026-07-12 |
| C03 | 34/36 · partial | Role-form FR stories and measured feedback budget remain; journey catalog and .env.example landed | `audit/.lane-c03/C03.md`; traceability lint; journey-to-test mapping | Keep trace artifacts current; add named user journeys | 2026-07-12 |
| C04 | 22/30 · partial | Maintainer 2FA is unproven; commit signing and secret-policy evidence incomplete | `audit/.lane-c04/C04.md`; org settings evidence; signed release verification | Human records 2FA; tighten local secret checks | 2026-07-12 |
| C05 | 26/30 · partial | Trace continuity, profiling, live alert routing, provisioned dashboards, scheduled chaos/load remain | `audit/.lane-c05/C05.md`; OTLP integration test; provisioned dashboard/alert proof | Add endpoint labels/histograms and scheduled operational tests | 2026-07-12 |
| C06 | 22/30 · partial | Linux two-build binary comparison is automated; hermetic builds, cross-host/Windows proof, SLSA-L3, blocking and container provenance remain incomplete | `scripts/repro-check.ps1`; `.github/workflows/ci.yml`; `docs/ops/reproducible-builds.md`; two-build digest match | Pin an immutable offline builder, expand release-target comparisons, then make per-build provenance blocking | 2026-07-12 |
| C07 | 25/30 · partial | Race checks, durable flake tracking, and PR cross-platform coverage remain incomplete | `tests/properties.rs`; `fuzz/`; CI property repeats and 10-second fuzz smoke; `CONTRIBUTING.md` flake policy | Extend WBS-6.2 with race and OS-matrix evidence | 2026-07-12 |
| C08 | 22/30 · partial | No per-PR perf regression gate; compression/token and reproducibility evidence shallow | `audit/.lane-c08/C08.md`; checked baseline + threshold gate | Add stable benchmark baseline and regression policy | 2026-07-12 |
| C09 | 33/45 · partial | Native WebView/live-data, responsive, cognitive, help, and efficiency coverage remain incomplete | `audit/.lane-c09/C09.md`; WCAG suite against all built-viewer tabs at three widths | Add responsive overflow/touch assertions and native integration evidence | 2026-07-12 |
| C10 | 26/36 · partial | Typography/theme/splash and automated golden coverage remain incomplete | `audit/.lane-c10/C10.md`; deterministic screenshots pass against visual spec | Automate viewer golden captures and theme checks | 2026-07-12 |
| C11 | 30/45 · partial | Portable Windows/Linux archives now have release-gating launch smoke and WiX source/docs are published; native installer install/uninstall, signing, channels, parity, and mobile decision remain | `audit/.lane-c11/C11.md`; `.github/workflows/release.yml`; `packaging/README.md`; `docs/ops/distribution.md` | Add clean-host native installer install/launch/uninstall evidence; record signing and mobile decisions | 2026-07-12 |

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
| PLAN-W8-B | done | Wave-12 result is 302/402 (75% B), meeting the B threshold | Independent audit-v38 result is at least 302/402 and >=75% | Hold B; pursue C11 signing/clean-host install and C09 depth for higher grades | 2026-07-13 |
| PLAN-ORG | partial | Registry spine and governance-policy conformance require cross-repo/human evidence | Registry entry links SessionLedger; policy checklist and org controls are recorded | Human owns WBS-9.1..WBS-9.3 | 2026-07-12 |

