# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

Security fixes land on the latest release line first. Older tags are not backported unless a maintainer explicitly commits to a patch release.

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security reports.

Preferred channels (in order):

1. **GitHub Security Advisories** ΓÇö [Report a vulnerability](https://github.com/KooshaPari/SessionLedger/security/advisories/new) on this repository.
2. **Email** ΓÇö contact maintainer **KooshaPari** via the address listed on their [GitHub profile](https://github.com/KooshaPari) (placeholder until a dedicated security inbox is published).

Include:

- Affected version / commit
- Reproduction steps or proof-of-concept
- Impact assessment (confidentiality / integrity / availability)
- Whether the issue is already public

## Disclosure Timeline

| Stage                         | Target                          |
| ----------------------------- | ------------------------------- |
| Initial acknowledgement       | within **3 business days**      |
| Triage / severity assessment  | within **7 days** of report     |
| Fix or mitigation plan        | within **30 days** (typical)    |
| Public disclosure / advisory  | coordinated after a fix ships   |

We follow coordinated disclosure. If a fix cannot ship within 90 days, we will discuss interim guidance with the reporter before any public write-up.

Critical supply-chain or remote-code issues may be accelerated at maintainer discretion.

## Supply Chain & SBOM

- Dependency policy is enforced by [`deny.toml`](deny.toml) via `cargo deny check` (see [`.github/workflows/security.yml`](.github/workflows/security.yml)).
- Secret scanning runs with gitleaks and TruffleHog (dual-scan) in the same workflow on PRs and pushes to `main`. Local pre-commit hooks use gitleaks only.
- CycloneDX SBOMs are produced in the qgate path as `target/sbom.cdx.json` and per-crate `*.cdx.json` artifacts (see [`.github/workflows/qgate.yml`](.github/workflows/qgate.yml) header comments). Packaging notes: [`packaging/README.md`](packaging/README.md).

- Advisory scanning: `cargo audit` job in .github/workflows/security.yml.
- SBOM upload: qgate uploads `sbom-cyclonedx` artifact from `target/sbom.cdx.json`.

