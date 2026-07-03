# SessionLedger Contributing Guide

## Prerequisites
- Rust stable toolchain (latest stable)
- Cargo (bundled with Rust)

## Build
```bash
cargo build
```

## Testing
**IMPORTANT**: sl-daemon must be tested in isolation because sl-viewer depends on webkit2gtk-sys which cannot resolve on macOS. Run:
```bash
cargo test -p sl-daemon
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

## Governance
This repository follows governance guidelines defined in ~/.claude/CLAUDE.md at a high level.