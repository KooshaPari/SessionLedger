# Changelog

Follows [Keep a Changelog](https://keepachangelog.com/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

### Added

- SLSA L3 environment isolation SelfCheck (C06 L53): `scripts/slsa-isolation-check.ps1 -SelfCheck`, isolated container rebuild evidence row in `hermetic-builds.md`, `repro-check.ps1 -PolicyOnly` isolation hook, soft CI in `hermetic.yml` (not a full L3 attestation).

- ADR 0006: explicit no MCP host/server / pin list (C06 L57) + `mcp-scope` SelfCheck.

- Go OKF adapter stub (`adapters/go`) beside Python for C08 L75 cross-language parity (validate/emit CLI; SelfCheck skips runtime when `go` absent).

- Soft Alertmanager packaging sample + SelfCheck (C05; local placeholder only, live webhook unpaid).

- Soft shuttle SelfCheck evidence (C00 L7): `docs/ops/shuttle-soft.md`, `scripts/shuttle-soft-check.ps1 -SelfCheck`, `tests/shuttle_soft.rs` (full shuttle permutation coverage remains unpaid).

- Loom permutation checkers (C00 L7): expanded `tests/loom_model.rs` (bounded `try_send`, broadcast epoch, watcher pipeline), `scripts/loom-permutation-check.ps1 -SelfCheck`, blocking `.github/workflows/loom-permutation.yml` (full tokio broadcast / daemon graph remains unpaid).

- Soft continuous-profiling HTTP push (`push_backend: http_soft` + optional `SL_PROFILE_PUSH_URL`; DryRun / continue-on-error) (C05 L45).

- OTLP metrics export (`otel-metrics` + `SL_OTLP_METRICS_ENDPOINT` / `OTEL_EXPORTER_OTLP_ENDPOINT`; MetricExporter + SdkMeterProvider; RED bridge unpaid) (C05 L43).

- Soft multi-locale i18n: `locales/es.json` + `SL_LOCALE` / `t_locale` selection (C01 L16; Fluent/ICU still deferred).

- Soft envelope helper (`SL_ENVELOPE_KEY` + SHA-256 keystream) in `src/envelope.rs` (C02 L22; not a KMS).

- Viewer `ErrorState` non-color cues: warning glyph + `aria-invalid` (C09 L81.15).

- Versioning policy SSOT + CHANGELOG tagged-section SelfCheck (C11 L119).
- ADR 0005: explicit no Workers/Vercel/edge deploy target (C11 L114) + `edge-deploy-scope` SelfCheck.
- Blocking `sandbox-boundary` SelfCheck job in `security.yml` (C04 L40; hard no-net/rootless still unpaid).

## [0.2.0] - 2026-07-04

Initial public release tag (`v0.2.0`).

### Added

- Desktop viewer release workflow + launch instructions.
- Packaging scaffolds for macOS `.app` and Linux portable binaries.
- Session list / search selection in the viewer.
- Domain mutation-targeted state machine and boundary tests.

<!-- Earlier history was Unreleased-aggregated; tag sections start at 0.2.0. -->
