# SessionLedger work breakdown structure

Status vocabulary is machine-stable: `done`, `partial`, `todo`, `blocked`, or
`na`. Owner is either `machine` (an agent may claim and execute it) or `human`
(a policy, credential, platform-signing, or product decision is required).
[`TRACEABILITY.json`](TRACEABILITY.json) is the machine-readable mirror.

## Update contract

Any agent that changes delivery status MUST update this file,
[`GAP_QA_MATRIX.md`](GAP_QA_MATRIX.md) where the affected gap is represented,
and [`TRACEABILITY.json`](TRACEABILITY.json) in the same change. A package may
be marked `done` only when its evidence paths exist and its referenced
acceptance criteria pass. Do not revise [`audit/SCORECARD.md`](../../audit/SCORECARD.md)
without a new audit.

## DESIGN P0-P6

| ID | Phase / work package | Status | Owner | Evidence paths | FR / PLAN / audit cross-refs |
|---|---|---|---|---|---|
| WBS-0.1 | P0 discovery: design, domain model, ports, repository and CI skeleton | done | machine | `docs/DESIGN.md`; `src/domain/`; `src/ports/`; `.github/workflows/ci.yml` | FR-012; T-001; C00, C03 |
| WBS-0.2 | P0 OKF contract and conformance corpus | done | machine | `docs/reference/OKF-SPEC.md`; `docs/reference/conformance/`; `tests/okf_roundtrip.rs` | FR-002, FR-013; T-002, T-003; C01, C08 |
| WBS-1.1 | P1 domain compilers, token estimator, and injection renderer | done | machine | `src/distill/`; `src/inject.rs`; `src/domain/bundle.rs` | FR-012; T-030, T-031; C00 |
| WBS-2.1 | P2 forge SQLite ingestion adapter | done | machine | `src/ingestion/forge.rs`; `Cargo.toml` | FR-001; T-032; C00 |
| WBS-2.2 | P2 Codex, Claude, and Cursor JSONL adapters | done | machine | `src/ingestion/`; `tests/skeleton.rs` | FR-001; T-033; C00, C08 |
| WBS-3.1 | P3 MemoryStore, Compressor, and TraceSink adapters | done | machine | `src/ports/adapters.rs`; `src/distill/memory_writer.rs` | FR-015; T-034; C00, C05 |
| WBS-3.2 | P3 LLM intent extractor and curation convergence | todo | machine | `docs/DESIGN.md` §6-7; `src/ports/` | DESIGN P3 residual; C00 |
| WBS-4.1 | P4 viewer history, replay, live feed, search, and unfinished surface | done | machine | `crates/sl-viewer/src/`; `crates/sl-viewer/src/unfinished_tab.rs` | FR-007..011; T-008, T-011, T-036; C03, C09, C10 |
| WBS-4.2 | P4 FTS recall via context-mode and explicit TUI decision | partial | human | `docs/DESIGN.md` §3, §7; `crates/sl-viewer/` | DESIGN P4 residual; C00, C11 |
| WBS-5.1 | P5 deterministic dedup merge and crash/lost-work recovery E2E | done | machine | `src/domain/merge.rs`; `src/domain/worklog.rs`; `tests/merge_recovery.rs` | FR-011; T-024, T-035; C03 |
| WBS-6.1 | P6 85% coverage gate and deterministic golden corpus | done | machine | `.github/workflows/ci.yml`; `tests/okf_golden.rs`; `tests/fixtures/okf/` | T-037, T-038; C01, C08 |
| WBS-6.2 | P6 property tests, fuzzing, race checks, and enforced performance budgets | partial | machine | `tests/properties.rs`; `fuzz/fuzz_targets/okf_roundtrip.rs`; `.github/workflows/ci.yml`; `CONTRIBUTING.md`; `benches/pipeline.rs` | DESIGN P6 residual; C00 L6-L8; C07 L66-L68 |

## audit-v38 waves

These rows record delivery waves; scores remain owned by the audit evidence and
are not recomputed here.

| ID | Audit wave / work package | Status | Owner | Evidence paths | FR / PLAN / audit cross-refs |
|---|---|---|---|---|---|
| WBS-7.1 | audit-v38 P0 baseline and cluster evidence capture | done | machine | `audit/SCORECARD.md`; `audit/.lane-c00/` … `audit/.lane-c11/` | audit-v38 C00-C11 |
| WBS-7.2 | Wave-2 governance, docs, CI, security, and visual baseline | done | machine | `audit/SCORECARD.md`; `docs/THREAT_MODEL.md`; `docs/VISUAL_SPEC.md` | T-002..T-025; C00-C11 |
| WBS-7.3 | Wave-3 OTLP traces, signing, reduced motion, visual harness | done | machine | `crates/sl-daemon/src/otel.rs`; `tests/visual/`; `.github/workflows/release.yml` | FR-015; C04, C05, C09, C10 |
| WBS-7.4 | Wave-4/5 product-depth PLAN closure | done | machine | `PLAN.md`; `WORK_DAG.md`; `src/ingestion/`; `src/domain/merge.rs` | T-030..T-038; FR-001, FR-011, FR-012, FR-015 |
| WBS-7.5 | Wave-6 observability, eval, packaging, and a11y uplift | done | machine | `docs/ops/dashboards/`; `scripts/load-smoke.ps1`; `tests/visual/`; `docs/ops/distribution.md` | C05, C08, C09, C11 |
| WBS-7.6 | Wave-7 supply-chain provenance and scorecard refresh | done | machine | `.github/workflows/release.yml`; `audit/SCORECARD.md` | C01, C04, C06; 268/402 |
| WBS-8.1 | Wave-8 machine traceability and in-document status governance | done | machine | `docs/ops/WBS.md`; `docs/ops/GAP_QA_MATRIX.md`; `docs/ops/TRACEABILITY.json`; `docs/ops/traceability_lint.ps1` | C03; FR-001..FR-015; PLAN |
| WBS-8.2 | Wave-8 close evidence-backed gaps to at least B (>=75%) | done | machine | `docs/ops/GAP_QA_MATRIX.md`; `.github/workflows/release.yml`; `packaging/README.md`; `docs/ops/distribution.md`; `scripts/repro-check.ps1`; `docs/ops/reproducible-builds.md`; Wave-8/9 re-audit evidence | C00-C11; C11 portable artifact smoke + repro evidence landed; Wave-25 re-audit landed at 353/402 (88% B) |
| WBS-8.3 | Wave-8 independent re-audit and scorecard refresh | done | human | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 273/402 (68% C) |
| WBS-8.4 | Wave-9 evidence packages (C02 trust, C06 repro, C07 property/fuzz, C11 archive smoke) | done | machine | `.github/workflows/release.yml`; `scripts/repro-check.ps1`; `docs/ops/local-trust-boundary.md`; `tests/properties.rs`; `fuzz/` | C02, C06, C07, C11; PRs #120-#123 |
| WBS-8.5 | Wave-9 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 280/402 (70% C) |
| WBS-8.6 | Wave-10 evidence packages (C01 CI hygiene, C03 journeys, C05 load/alerts, C08 bench gate) | done | machine | `.github/workflows/a11y.yml`; `.github/workflows/ops-load.yml`; `.github/workflows/bench-gate.yml`; `docs/USER_JOURNEYS.md`; `.env.example` | C01, C03, C05, C08; PRs #125-#128 |
| WBS-8.7 | Wave-10 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 285/402 (71% C) |
| WBS-8.8 | Wave-11 evidence packages (C02 audit sink, C06 hermetic, C07 race/flake, C11 mobile ADR) | done | machine | `crates/sl-daemon/src/audit.rs`; `.github/workflows/hermetic.yml`; `tests/race_smoke.rs`; `docs/adr/0002-mobile-presence.md` | C02, C06, C07, C11; PRs #130-#133 |
| WBS-8.9 | Wave-11 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 294/402 (73% C) |
| WBS-8.10 | Wave-12 evidence packages (C05 obs, C07 matrix/seed, C08 compression, C10 goldens) | done | machine | `.github/workflows/ops-dashboards.yml`; `docs/ops/cross-platform-ci.md`; `tests/compression_eval.rs`; `tests/visual/golden/` | C05, C07, C08, C10; PRs #135-#138 |
| WBS-8.11 | Wave-12 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 302/402 (75% B) |
| WBS-8.12 | Wave-13 evidence packages (C02 API key, C09 responsive, C11 channels/systemd) | done | machine | `crates/sl-daemon/src/http.rs`; `tests/visual/harness/responsive.spec.js`; `packaging/channels.md`; `packaging/systemd/` | C02, C09, C11; PRs #140-#142 |
| WBS-8.13 | Wave-13 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 306/402 (76% B) |
| WBS-8.14 | Wave-14 evidence packages (C01 CLI/envelope, C05 pprof, C09 inclusive/hotkeys) | done | machine | `crates/sl-daemon/src/main.rs`; `crates/sl-daemon/src/http.rs`; `.vale.ini`; `docs/viewer-hotkeys.md` | C01, C05, C09; PRs #144-#146 |
| WBS-8.15 | Wave-14 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 311/402 (77% B) |
| WBS-8.16 | Wave-15 evidence packages (C00 OpenAPI/idempotency, C04 gitleaks pre-commit, C10 tokens/theme/AA) | done | machine | `docs/api/openapi.yaml`; `crates/sl-daemon/src/http.rs`; `.pre-commit-config.yaml`; `assets/tokens.css`; `crates/sl-viewer/src/theme.rs` | C00, C04, C10; PRs #148-#150 |
| WBS-8.17 | Wave-15 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 316/402 (79% B) |
| WBS-8.18 | Wave-16 evidence packages (C00 OpenAPI drift, C04 trufflehog, C09 responsive/touch, C10 spacing/motion/live-feed, C11 HEALTHCHECK/install smoke) | done | machine | `scripts/openapi-drift-check.ps1`; `.github/workflows/security.yml`; `tests/visual/harness/responsive.spec.js`; `assets/tokens.css`; `scripts/installer-lifecycle-smoke.ps1`; `Containerfile` | C00, C04, C09, C10, C11; PRs #152-#156 |
| WBS-8.19 | Wave-16 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 320/402 (80% B) |
| WBS-8.20 | Wave-17 evidence packages (C06 hermetic builder pin, C08 eval manifest, C10 caption/splash/banner, C11 signing ADR) | done | machine | `docs/ops/hermetic-builder.json`; `docs/ops/eval-manifest.json`; `assets/tokens.css`; `crates/sl-daemon/src/banner.rs`; `docs/adr/0003-platform-code-signing.md` | C06, C08, C10, C11; PRs #158-#161 |
| WBS-8.21 | Wave-17 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 324/402 (81% B) |
| WBS-8.22 | Wave-18 evidence packages (C00 schema scaffold, C06 per-matrix provenance contract, C07 race OS-matrix, C10 skeletons/L1 golden) | done | machine | `src/schema/`; `scripts/provenance-contract-check.ps1`; `.github/workflows/race-smoke.yml`; `crates/sl-viewer/src/async_states.rs`; `tests/visual/golden/l1-content-skeleton.png` | C00, C06, C07, C10; PRs #163-#166 |
| WBS-8.23 | Wave-18 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 326/402 (81% B) |
| WBS-8.24 | Wave-19 evidence packages (C00 SqliteMemoryStore, C05 per-route histograms, C07 flake tracker/rerun stats, C10 stream skeletons) | done | machine | `src/ports/sqlite_memory.rs`; `crates/sl-daemon/src/metrics.rs`; `scripts/flake-tracker-check.ps1`; `scripts/flake-rerun-stats.ps1`; `crates/sl-viewer/src/live_feed.rs`; `crates/sl-viewer/src/replay_view.rs` | C00, C05, C07, C10; PRs #168-#171 |
| WBS-8.25 | Wave-19 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 327/402 (81% B) |
| WBS-8.26 | Wave-20 evidence packages (C00 daemon memory wiring, C05 Grafana route panels, C07 cross-platform build + flake quarantine, C10 empty/error goldens) | done | machine | `crates/sl-daemon/src/main.rs`; `crates/sl-daemon/src/http.rs`; `docs/ops/dashboards/sessionledger-red.json`; `scripts/flake-quarantine-apply.ps1`; `.github/workflows/cross-platform-build.yml`; `tests/visual/golden/e2-history-empty.png`; `tests/visual/golden/e4-search-empty.png`; `tests/visual/golden/r1-search-error.png` | C00, C05, C07, C10; PRs #173-#176 |
| WBS-8.27 | Wave-20 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 329/402 (82% B) |
| WBS-8.28 | Wave-21 evidence packages (C01 CLI/CI, C02 audit sink, C04 commit signing, C05 chaos smoke, C06 SLSA materials, C08 eval corpus, C09 help overlay, C11 clean-host) | done | machine | `.github/workflows/ci.yml`; `crates/sl-daemon/src/audit.rs`; `docs/adr/0004-commit-signing-policy.md`; `.github/workflows/ops-chaos-smoke.yml`; `docs/ops/fixtures/slsa-materials-contract.sample.json`; `docs/ops/eval-manifest.json`; `crates/sl-viewer/src/help_overlay.rs`; `scripts/installer-lifecycle-smoke.ps1` | C01, C02, C04, C05, C06, C08, C09, C11; PRs #177-#185 |
| WBS-8.29 | Wave-21 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 333/402 (83% B) |
| WBS-8.30 | Wave-22 evidence packages (C00 CancellationToken, C02 audit retention, C05 game-day/SLO, C08 corpus-14, C09 status/cognitive a11y, C10 E5/R2/R3 goldens) | done | machine | `crates/sl-daemon/src/shutdown.rs`; `scripts/audit-review.ps1`; `.github/workflows/ops-gameday.yml`; `docs/ops/eval-manifest.json`; `docs/a11y/status-regions-and-native-smoke.md`; `tests/visual/golden/e5-first-run-empty.png` | C00, C02, C05, C08, C09, C10; PRs #187-#192 |
| WBS-8.31 | Wave-22 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 337/402 (84% B) |
| WBS-8.32 | Wave-23 evidence packages (C00 RSS budget, C03 FR roles + feedback budgets, C06 SOURCE_DATE_EPOCH, C09 SR procedure, C10 splash goldens) | done | machine | `tests/memory_budget.rs`; `docs/functional_requirements.md`; `docs/ops/feedback-budgets.md`; `scripts/repro-check.ps1`; `docs/a11y/screen-reader-smoke.md`; `tests/visual/golden/s1-launch-splash.png` | C00, C03, C06, C09, C10; PRs #194-#199 |
| WBS-8.33 | Wave-23 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 343/402 (85% B) |
| WBS-8.34 | Wave-24 evidence packages (C00 race_model, C01 env-example CI, C02 non-loopback API key, C05 pprof smoke, C07 jsonl_ingest fuzz, C09 error-field a11y) | done | machine | `tests/race_model.rs`; `scripts/env-example-check.ps1`; `docs/ops/local-trust-boundary.md`; `scripts/pprof-smoke.ps1`; `fuzz/fuzz_targets/jsonl_ingest.rs`; `crates/sl-viewer/src/search_view.rs` | C00, C01, C02, C05, C07, C09; PRs #201-#206 |
| WBS-8.35 | Wave-24 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 349/402 (87% B) |
| WBS-8.36 | Wave-25 evidence packages (C11 just/task, runtime facade, curl/irm+brew/winget, MSI/PKG) | done | machine | `justfile`; `Taskfile.yml`; `scripts/runtime-up.sh`; `scripts/runtime-up.ps1`; `docs/ops/runtime-facade.md`; `scripts/install.sh`; `scripts/install.ps1`; `packaging/homebrew/sessionledger.rb`; `packaging/winget/`; `packaging/macos/`; `scripts/package-msi.ps1`; `.github/workflows/release.yml` | C11; PRs #208-#211 |
| WBS-8.37 | Wave-25 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 353/402 (88% B) |

## Organization-level control plane

SessionLedger consumes, but does not redefine, the organization SSOTs:

| ID | Organization work package | Status | Owner | Evidence paths | Cross-refs |
|---|---|---|---|---|---|
| WBS-9.1 | Apply the canonical [audit-v38 rubric](https://github.com/KooshaPari/phenotype-org-audits/tree/main/audit-v38) and publish repo evidence back through an independent audit | partial | human | `audit/SCORECARD.md`; `audit/.lane-c00/` … `audit/.lane-c11/` | WBS-7.1, WBS-8.3; C00-C11 |
| WBS-9.2 | Keep SessionLedger identity, boundaries, and audit links aligned with the [phenotype-registry spine](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/SPINE-INDEX.md) | todo | human | `docs/DESIGN.md` §6; registry `docs/SPINE-INDEX.md` | DESIGN composition map; WBS-3.2 |
| WBS-9.3 | Conform repository controls to phenotype-org-governance [`POLICY.md`](https://github.com/KooshaPari/phenotype-org-governance/blob/main/POLICY.md) | partial | human | `CONTRIBUTING.md`; `SECURITY.md`; `CODEOWNERS`; `.github/workflows/` | C01, C02, C04, C06 |

