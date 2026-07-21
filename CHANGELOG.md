# Changelog

Follows [Keep a Changelog](https://keepachangelog.com/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

### Fixed

- Viewer first-run corpus CTA (C09): wire “Open corpus…” to a web Forge DB file picker (`corpus_cta.rs`) or open the quick-start runbook on desktop; `cargo test -p sl-viewer`.

- Commit signing header scan (C04 L34): `commit-signing-check.ps1` reads bounded commit headers via line-scanner (no unbounded `git cat-file` buffers or `(?ms)` regex); `-SelfCheck` + `tests/commit_signing_check.rs`.

- Loom permutation CI timeout (P0 stability): split blocking `loom-permutation.yml` into core + per-daemon `loom_model` jobs with `LOOM_MAX_PREEMPTIONS` on broadcast/pipeline/shutdown; mirror in soft `loom-smoke.yml` so Wave-40 tokio-shaped daemon graph tests no longer exceed single-job ceilings.

### Added

- Wave-43 scope (396/402): consolidated `WAVE43_SCOPE.md` + `docs/ops/WAVE43_PERT.md` — five parallel carry-forward lanes (`w43-daemon-graph-hard`, `w43-jemalloc-default-on`, `w43-load-macro-gate`, `w43-sl-viewer-help`, `w43-socket-posture`) from Wave-42 deferred gaps.

- Blocking alloc-profile / dhat PR gate (C00 L8): `.github/workflows/alloc-profile-hard.yml`, expanded `alloc-profile-check.ps1 -SelfCheck` anchors, `tests/alloc_profile_hard.rs` (soft `ops-load` job retained).

- SLSA protected-environment gate promotion (C06 L53): `slsa-protected-env-check.ps1 -SelfCheck` moved to blocking `security.yml` job (removed soft `hermetic.yml` bypass).

- SBOM schema validation + pinned cargo-cyclonedx (C04 L32): `docs/ops/sbom-policy.json`, `scripts/sbom-validate-check.ps1 -SelfCheck`, post-generation validation in `qgate.yml`/`release.yml`, blocking `security.yml` SBOM policy job, `tests/sbom_validate.rs`.

- Wave-42 scope (396/402): consolidated `WAVE42_SCOPE.md` + `docs/ops/WAVE42_PERT.md` — five parallel carry-forward lanes (`w42-signing-check-bound`, `w42-sbom-validate`, `w42-slsa-promote`, `w42-alloc-gate-promote`, `w42-first-run-cta`) from Wave-41 deferred gaps.

- P95 baseline refresh (C00 L6 / C08 L74): `bench-gate.ps1 -UpdateBaseline` writes `p95_source` per benchmark; `perf-baseline.json` refreshed from Criterion `sample.json` (replaces provisional mean×1.15 values).

- Source provenance traceability wrapper (C06 L59): `tests/source_provenance.rs` hermetic cargo test for `scripts/source-provenance-check.ps1 -SelfCheck`, closing TRACEABILITY.json gap at `09cc968`.

- CI job timeouts (P0 stability): `timeout-minutes` on heavy `ci.yml` jobs (`build-test` 45m, `fuzz-smoke` 15m, `coverage` 30m), `scripts/ci-timeout-check.ps1 -SelfCheck`, and `ci-timeout-policy` anchor smoke in `ci.yml` (security.yml scan jobs remain lightweight).

- Wave-41 scope (396/402): consolidated `WAVE41_SCOPE.md` + `docs/ops/WAVE41_PERT.md` — five parallel lanes (`w41-daemon-url-unify`, `w41-ci-timeout`, `w41-check-regex-bound`, `w41-source-provenance`, `w41-p95-baseline`) from stability, DX/UX, governance, and perf audits.

- User-initiated update check (C11 L111): `sl-daemon check-update` (GitHub release tag compare; no download/install), `docs/ops/update-check.md`, `scripts/update-check-check.ps1 -SelfCheck`, `tests/update_check.rs`, `crates/sl-daemon/tests/check_update.rs`, blocking `.github/workflows/update-check-hard.yml` + soft `update-check-soft.yml` (SelfCheck + hermetic `--latest` smoke; auto-install remains unpaid).

- Rootless-only OCI runner matrix scaffold (C04 L40): `scripts/rootless-matrix-check.ps1 -SelfCheck`, blocking `.github/workflows/rootless-matrix.yml`, `tests/rootless_matrix.rs`, `security.yml`/`ci.yml` anchors, `sandbox-boundary.md` matrix limits (live rootless runners + OCI build/smoke unpaid).

- Wave-40 tokio-shaped mpsc/broadcast/SSE daemon graph loom ports (C00 L7): expanded `tests/loom_model.rs` (mpsc watcher→consumer, mpsc drain→broadcast publish, triple SSE fan-out, full mpsc→broadcast→SSE pipeline, shutdown stops mpsc enqueue), updated `scripts/loom-permutation-check.ps1 -SelfCheck` and `docs/ops/concurrency-safety.md` done/unpaid rows (full live `sl-daemon` tokio broadcast graph remains unpaid).

- Wave-40 C11: blocking signing-readiness gate (#326): `scripts/signing-hard-check.ps1 -SelfCheck`, blocking `.github/workflows/signing-hard.yml`, `tests/signing_hard.rs` (Authenticode/notarization credentials remain unpaid).

- Blocking jemalloc CI (C00 L8): `scripts/jemalloc-check.ps1` hard gate anchors, `tests/jemalloc_hard.rs`, blocking `.github/workflows/jemalloc-hard.yml` (SelfCheck + `cargo build --features jemalloc` on Ubuntu PRs; soft `ops-load` job retained; always-on production jemalloc + Windows parity remain unpaid).

- Cargo-fetch no-net policy evidence (C04 L40): `scripts/cargo-nonet-check.ps1 -SelfCheck`, blocking `cargo-nonet` anchor in `security.yml`, `tests/cargo_nonet.rs`, `sandbox-boundary.md` cargo-fetch section (live runner no-net unpaid).

- Loom daemon-graph broadcast/SSE epoch permutations (C00 L7): expanded `tests/loom_model.rs` (multi-bump epoch fan-out, watcher→SSE pipeline, cancel-guarded conservation), updated `scripts/loom-permutation-check.ps1 -SelfCheck` and `docs/ops/concurrency-safety.md` done/unpaid rows (full tokio `sl-daemon` broadcast graph remains unpaid).

- TSan permutation checkers (C00 L7): `scripts/tsan-permutation-check.ps1 -SelfCheck`, `tests/tsan_permutation.rs`, blocking `.github/workflows/tsan-permutation.yml` (`cargo +nightly test --test race_model` under `-Zsanitizer=thread` on ubuntu x86_64; full tokio broadcast / daemon SSE graph ports remain unpaid).

- Source provenance policy SSOT + SelfCheck (C06 L59): `docs/ops/source-provenance.md`, `scripts/source-provenance-check.ps1 -SelfCheck`, `branch-protection-check.ps1 -PolicyOnly` hermetic hook, CONTRIBUTING cross-link (signed commits + CODEOWNERS + human org gates; live Settings remain NOT_VERIFIABLE_IN_REPO).

- SLSA L3 environment isolation SelfCheck (C06 L53): `scripts/slsa-isolation-check.ps1 -SelfCheck`, isolated container rebuild evidence row in `hermetic-builds.md`, `repro-check.ps1 -PolicyOnly` isolation hook, soft CI in `hermetic.yml` (not a full L3 attestation).

- ADR 0006: explicit no MCP host/server / pin list (C06 L57) + `mcp-scope` SelfCheck.

- Go OKF adapter stub (`adapters/go`) beside Python for C08 L75 cross-language parity (validate/emit CLI; SelfCheck skips runtime when `go` absent).

- Soft Alertmanager packaging sample + SelfCheck (C05; local placeholder only, live webhook unpaid).

- Soft shuttle SelfCheck evidence (C00 L7): `docs/ops/shuttle-soft.md`, `scripts/shuttle-soft-check.ps1 -SelfCheck`, `tests/shuttle_soft.rs` (full shuttle permutation coverage remains unpaid).

- Miri permutation checkers (C00 L7): `scripts/miri-permutation-check.ps1 -SelfCheck`, blocking `.github/workflows/miri-permutation.yml` (`cargo miri test --test race_model` on PR); soft `miri-smoke.yml` nightly retained (`loom_model` under Miri remains unpaid).

- Loom permutation checkers (C00 L7): expanded `tests/loom_model.rs` (bounded `try_send`, broadcast epoch, watcher pipeline), `scripts/loom-permutation-check.ps1 -SelfCheck`, blocking `.github/workflows/loom-permutation.yml` (full tokio broadcast / daemon graph remains unpaid).

- Soft continuous-profiling HTTP push (`push_backend: http_soft` + optional `SL_PROFILE_PUSH_URL`; DryRun / continue-on-error) (C05 L45).

- OTLP metrics export (`otel-metrics` + `SL_OTLP_METRICS_ENDPOINT` / `OTEL_EXPORTER_OTLP_ENDPOINT`; MetricExporter + SdkMeterProvider; RED bridge unpaid) (C05 L43).

- Soft multi-locale i18n: `locales/es.json` + `SL_LOCALE` / `t_locale` selection (C01 L16; Fluent/ICU still deferred).

- Fluent catalog stub (C01 L16 Phase-1): `locales/en.ftl` + `locales/es.ftl`, optional `fluent-catalog` feature (`fluent-bundle` + `unic-langid`), `src/i18n_fluent.rs` (`t_fluent` with JSON fallback), `scripts/fluent-i18n-check.ps1 -SelfCheck` (viewer migration still deferred).

- Soft envelope helper (`SL_ENVELOPE_KEY` + SHA-256 keystream) in `src/envelope.rs` (C02 L22; not a KMS).
- Hard envelope-crypto CI evidence (C02 L22): `scripts/envelope-crypto-check.ps1 -SelfCheck`, blocking `.github/workflows/envelope-crypto.yml`, `tests/envelope_crypto.rs` (`envelope-crypto` marker feature; KMS/sealed-secrets/KEK wrap unpaid).

- Blocking sustained fuzz (C07 L67): extended `docs/ops/fuzz-cadence.md` blocking vs soft matrix, `scripts/fuzz-cadence-check.ps1` done/unpaid rows, blocking `.github/workflows/fuzz-blocking.yml` (SelfCheck + 30 s / target `cargo fuzz` on PR); soft `fuzz-cadence.yml` nightly (120 s) retained; auto corpus promotion remains unpaid.

- Viewer `ErrorState` non-color cues: warning glyph + `aria-invalid` (C09 L81.15).

- Versioning policy SSOT + CHANGELOG tagged-section SelfCheck (C11 L119).
- ADR 0005: explicit no Workers/Vercel/edge deploy target (C11 L114) + `edge-deploy-scope` SelfCheck.
- Blocking `sandbox-boundary` SelfCheck job in `security.yml` (C04 L40; hard no-net/rootless still unpaid).
- Hard rootless/no-net CI evidence (C04 L40): `scripts/rootless-nonet-check.ps1 -SelfCheck`, blocking `.github/workflows/rootless-nonet.yml`, `tests/rootless_nonet.rs`, `security.yml`/`ci.yml` anchors (live runner matrix + cargo-fetch no-net still unpaid).

### Fixed

- Loom CI timeout (C00 L7): refactor concurrent mpsc consumer permutations to sequential drain (Wave-39 pattern) and raise `loom-permutation.yml` / `loom-smoke.yml` suite timeout to 45m (aligns with miri-permutation).

## [0.2.0] - 2026-07-04

Initial public release tag (`v0.2.0`).

### Added

- Desktop viewer release workflow + launch instructions.
- Packaging scaffolds for macOS `.app` and Linux portable binaries.
- Session list / search selection in the viewer.
- Domain mutation-targeted state machine and boundary tests.

<!-- Earlier history was Unreleased-aggregated; tag sections start at 0.2.0. -->
