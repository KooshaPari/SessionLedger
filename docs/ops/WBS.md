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
| WBS-8.2 | Wave-8 close evidence-backed gaps to at least B (>=75%) | done | machine | `docs/ops/GAP_QA_MATRIX.md`; `.github/workflows/release.yml`; `packaging/README.md`; `docs/ops/distribution.md`; `scripts/repro-check.ps1`; `docs/ops/reproducible-builds.md`; Wave-8/9 re-audit evidence | C00-C11; C11 portable artifact smoke + repro evidence landed; Wave-12 re-audit landed at 302/402 (75% B) |
| WBS-8.3 | Wave-8 independent re-audit and scorecard refresh | done | human | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 273/402 (68% C) |
| WBS-8.4 | Wave-9 evidence packages (C02 trust, C06 repro, C07 property/fuzz, C11 archive smoke) | done | machine | `.github/workflows/release.yml`; `scripts/repro-check.ps1`; `docs/ops/local-trust-boundary.md`; `tests/properties.rs`; `fuzz/` | C02, C06, C07, C11; PRs #120-#123 |
| WBS-8.5 | Wave-9 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 280/402 (70% C) |
| WBS-8.6 | Wave-10 evidence packages (C01 CI hygiene, C03 journeys, C05 load/alerts, C08 bench gate) | done | machine | `.github/workflows/a11y.yml`; `.github/workflows/ops-load.yml`; `.github/workflows/bench-gate.yml`; `docs/USER_JOURNEYS.md`; `.env.example` | C01, C03, C05, C08; PRs #125-#128 |
| WBS-8.7 | Wave-10 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 285/402 (71% C) |
| WBS-8.8 | Wave-11 evidence packages (C02 audit sink, C06 hermetic, C07 race/flake, C11 mobile ADR) | done | machine | `crates/sl-daemon/src/audit.rs`; `.github/workflows/hermetic.yml`; `tests/race_smoke.rs`; `docs/adr/0002-mobile-presence.md` | C02, C06, C07, C11; PRs #130-#133 |
| WBS-8.9 | Wave-11 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 294/402 (73% C) |
| WBS-8.10 | Wave-12 evidence packages (C05 obs, C07 matrix/seed, C08 compression, C10 goldens) | done | machine | `.github/workflows/ops-dashboards.yml`; `docs/ops/cross-platform-ci.md`; `tests/compression_eval.rs`; `tests/visual/golden/` | C05, C07, C08, C10; PRs #135-#138 |
| WBS-8.11 | Wave-12 independent re-audit and scorecard refresh | done | machine | `audit/SCORECARD.md`; phenotype-org-audits audit output | audit-v38; 302/402 (75% B) |

## Organization-level control plane

SessionLedger consumes, but does not redefine, the organization SSOTs:

| ID | Organization work package | Status | Owner | Evidence paths | Cross-refs |
|---|---|---|---|---|---|
| WBS-9.1 | Apply the canonical [audit-v38 rubric](https://github.com/KooshaPari/phenotype-org-audits/tree/main/audit-v38) and publish repo evidence back through an independent audit | partial | human | `audit/SCORECARD.md`; `audit/.lane-c00/` … `audit/.lane-c11/` | WBS-7.1, WBS-8.3; C00-C11 |
| WBS-9.2 | Keep SessionLedger identity, boundaries, and audit links aligned with the [phenotype-registry spine](https://github.com/KooshaPari/phenotype-registry/blob/main/docs/SPINE-INDEX.md) | todo | human | `docs/DESIGN.md` §6; registry `docs/SPINE-INDEX.md` | DESIGN composition map; WBS-3.2 |
| WBS-9.3 | Conform repository controls to phenotype-org-governance [`POLICY.md`](https://github.com/KooshaPari/phenotype-org-governance/blob/main/POLICY.md) | partial | human | `CONTRIBUTING.md`; `SECURITY.md`; `CODEOWNERS`; `.github/workflows/` | C01, C02, C04, C06 |

