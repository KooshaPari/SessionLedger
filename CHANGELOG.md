# Changelog

Follows [Keep a Changelog](https://keepachangelog.com/); versioning is [SemVer](https://semver.org/).

## [Unreleased]

### Added

- Security pack (audit-v38 C04/C06 P0): `SECURITY.md`, `CODE_OF_CONDUCT.md`, `deny.toml`, `.github/workflows/security.yml` (cargo-deny + gitleaks), `.github/dependabot.yml`, `.pre-commit-config.yaml`.
- `AGENTS.md` — agent entrypoint (build/test/lint/worktree/forbidden-ops).
- `rust-toolchain.toml` — reproducible toolchain (MSRV 1.85 via Cargo).
- `.github/PULL_REQUEST_TEMPLATE.md` and `user-friction` issue template + `docs/friction-log.md`.
- proc-compose deploy stack with sl-viewer + Makefile + quickstart (#39).

<!-- Prior history predates this changelog; reconstruct from git tags as versions are cut. -->
