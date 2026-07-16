# Optional jemalloc allocator (L8 evidence, soft)

SessionLedger ships counting-allocator and optional `dhat` smokes for library
pipelines ([`allocation-budget.md`](allocation-budget.md),
[`alloc-profile.md`](alloc-profile.md)). This document covers the **soft
optional jemalloc** path for `sl-daemon`: a Cargo feature that installs
`tikv-jemallocator` as the process `#[global_allocator]` on Unix.

Default builds are **unchanged** and **Windows-safe** — jemalloc is never on
the default feature set and is not resolved on non-Unix targets.

## Contract

| Knob | Value | Source |
|------|-------|--------|
| Feature | `jemalloc` (off by default) | [`crates/sl-daemon/Cargo.toml`](../../crates/sl-daemon/Cargo.toml) |
| Crate | `tikv-jemallocator` (optional, `cfg(unix)`) | same |
| Install site | `#[global_allocator]` behind `cfg(all(feature = "jemalloc", unix))` | [`crates/sl-daemon/src/main.rs`](../../crates/sl-daemon/src/main.rs) |
| SelfCheck | docs + Cargo feature + cfg anchors (no jemalloc compile) | [`scripts/jemalloc-check.ps1`](../../scripts/jemalloc-check.ps1) |
| Soft CI | Ubuntu build with `--features jemalloc`, `continue-on-error` | [`.github/workflows/ops-load.yml`](../../.github/workflows/ops-load.yml) job `jemalloc` |

## How to run

### Self-check (hermetic; Windows-safe)

```powershell
pwsh ./scripts/jemalloc-check.ps1 -SelfCheck
```

### Enable jemalloc (Unix)

```bash
cargo build --manifest-path crates/sl-daemon/Cargo.toml --features jemalloc --locked
```

Or:

```bash
cargo run --manifest-path crates/sl-daemon/Cargo.toml --features jemalloc --locked -- serve --watch ./sessions --out ./okf-out
```

Hermetic wiring test (no jemalloc compile on default graph):

```powershell
cargo test --test jemalloc_soft --locked
```

## Soft jemalloc SelfCheck

| Gate | Status |
|------|--------|
| Soft jemalloc SelfCheck | **done** |
| Default / Windows builds unchanged (system allocator) | **done** |
| Soft Ubuntu `--features jemalloc` CI (`continue-on-error`) | **done** |
| Continuous jemalloc profiling / production always-on jemalloc | **unpaid** |

## CI / scheduling

- **PR / push:** `cargo test --test jemalloc_soft` exercises the hermetic
  SelfCheck (no jemalloc compile).
- **Scheduled soft job:** `jemalloc` in
  [`.github/workflows/ops-load.yml`](../../.github/workflows/ops-load.yml)
  (`continue-on-error: true`) runs SelfCheck + `cargo build --features jemalloc`
  on Ubuntu.

## Limitations

- Unix only — Windows (and other non-Unix) keeps the system allocator even if
  the feature flag is passed (dep is `cfg(unix)`).
- Soft evidence only — does not force jemalloc into release artifacts or the
  default feature set.
- Does not replace RSS / allocation-budget / dhat smokes; those remain separate
  L8 companions.
- Continuous profiling push backends and always-on production jemalloc remain
  unpaid.
