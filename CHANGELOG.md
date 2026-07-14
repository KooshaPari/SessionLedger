# Changelog

Follows [Keep a Changelog](https://keepachangelog.com/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

### Added

- Versioning policy SSOT + CHANGELOG tagged-section SelfCheck (C11 L119).

- Blocking `sandbox-boundary` SelfCheck job in `security.yml` (C04 L40; hard no-net/rootless still unpaid).

- `GET /readyz` readiness probe + process-compose probe switch; SLO stubs in observability docs.

- Security pack (audit-v38 C04/C06 P0): `SECURITY.md`, `CODE_OF_CONDUCT.md`, `deny.toml`, `.github/workflows/security.yml` (cargo-deny + gitleaks), `.github/dependabot.yml`, `.pre-commit-config.yaml`.
- Agent-readiness docs: `docs/functional_requirements.md` (FR catalog), `PLAN.md`, `WORK_DAG.md`, `llms.txt`.
- Ops stubs: `docs/ops/runbook.md` (`make dev` / healthz :8080) and `docs/ops/observability.md` (metrics + OTel soft goal).
- `AGENTS.md` ΓÇö agent entrypoint (build/test/lint/worktree/forbidden-ops); Key files links to FR/PLAN/llms/runbook.
- `rust-toolchain.toml` ΓÇö reproducible toolchain (MSRV 1.85 via Cargo).
- `.github/PULL_REQUEST_TEMPLATE.md` and `user-friction` issue template + `docs/friction-log.md`.
- proc-compose deploy stack with sl-viewer + Makefile + quickstart (#39).

<!-- Prior history predates this changelog; reconstruct from git tags as versions are cut. -->

