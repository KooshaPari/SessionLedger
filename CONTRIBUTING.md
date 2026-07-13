# SessionLedger Contributing Guide

## Prerequisites
- Rust stable toolchain (latest stable)
- Cargo (bundled with Rust)
- pre-commit for local repository hooks

## Build
```bash
cargo build
```

## Testing
**IMPORTANT**: sl-daemon must be tested in isolation because sl-viewer depends on webkit2gtk-sys which cannot resolve on macOS. Run:
```bash
cargo test -p sl-daemon
```

Property tests run with the normal root test suite. Before submitting changes to
dedup, merge, ingestion, or OKF code, repeat the focused suite locally:

```bash
for run in 1 2 3; do PROPTEST_CASES=64 cargo test --test properties; done
```

A failed test is not retried to turn CI green. Preserve the proptest seed or
regression case from the failure output, open an issue for any confirmed flake,
and quarantine a test only with an owner, linked issue, and removal deadline.
The CI repeat is a short detection signal; it does not replace a root-cause fix.
Use the [flake tracker](docs/ops/flake-tracker.md) to record confirmed flakes
and any temporary quarantine.

The `fuzz-smoke` CI job runs the committed OKF corpus for 10 seconds. Longer
local campaigns use `cargo fuzz run okf_roundtrip`.

## Inclusive Language Checks

Run the lightweight seed gate before documentation-heavy changes:

```powershell
pwsh -NoProfile -File scripts/inclusive-language-check.ps1
```

The script scans Markdown under `docs/` plus `CONTRIBUTING.md` for a small
deny-list. It is intentionally dependency-free so CI can add it as a warning or
blocking check later.

Vale is also configured as a warning-level style for the same scope. After
installing Vale locally, run:

```powershell
vale docs CONTRIBUTING.md
```

Use the PowerShell script as the seed gate today. Upgrade the Vale style over
time with project-specific substitutions, then wire `vale docs CONTRIBUTING.md`
into CI as an optional warning before making it blocking.

## Local Hooks

Install local hooks once per checkout:

```bash
pre-commit install
```

The hook set includes gitleaks secret scanning for staged changes. To run it
manually before committing, stage the intended files and run:

```bash
pre-commit run gitleaks
```

## Branch Discipline
- Always create feature branches off main
- Never commit directly to main
- Use conventional-commits style for commit messages

## PR Workflow
- Create one focused PR per change
- Ensure all tests pass (0-failed)
- Verify no regressions
- CI runs Linux-only (per billing constraints)
- Reference the existing [qgate.yml](.github/workflows/qgate.yml) quality gate

## Developer Certificate of Origin (DCO)
All contributions must be signed off under the [Developer Certificate of Origin](https://developercertificate.org/).
Each commit must include a `Signed-off-by:` trailer matching the commit author:

```bash
git commit -s -m "your message"
```

By signing off, you certify that you have the right to submit the work under this
repository's dual MIT OR Apache-2.0 license. PRs should confirm the DCO checkbox
in the pull request template.

## Governance
This repository follows governance guidelines defined in ~/.claude/CLAUDE.md at a high level.
