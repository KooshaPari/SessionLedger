# Changelog

Follows [Keep a Changelog](https://keepachangelog.com/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

### Added

- Viewer `ErrorState` non-color cues: warning glyph + `aria-invalid` (C09 L81.15).

- `GET /readyz` readiness probe + process-compose probe switch; SLO stubs in observability docs.

- Security pack (audit-v38 C04/C06 P0): `SECURITY.md`, `CODE_OF_CONDUCT.md`, `deny.toml`, `.github/workflows/security.yml` (cargo-deny + gitleaks), `.github/dependabot.yml`, `.pre-commit-config.yaml`.
- Agent-readiness docs: `docs/functional_requirements.md` (FR catalog), `PLAN.md`, `WORK_DAG.md`, `llms.txt`.
- Ops stubs: `docs/ops/runbook.md` (`make dev` / healthz :8080) and `docs/ops/observability.md` (metrics + OTel soft goal).
- `AGENTS.md` — agent entrypoint (build/test/lint/worktree/forbidden-ops); Key files links to FR/PLAN/llms/runbook.
- `rust-toolchain.toml` — reproducible toolchain (MSRV 1.85 via Cargo).
- `.github/PULL_REQUEST_TEMPLATE.md` and `user-friction` issue template + `docs/friction-log.md`.
- proc-compose deploy stack with sl-viewer + Makefile + quickstart (#39).

<!-- Prior history predates this changelog; reconstruct from git tags as versions are cut. -->

