# Optional jemalloc allocator (L8 evidence, soft + hard)

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
| Hard CI | Ubuntu SelfCheck + `--features jemalloc` build, blocking PR gate | [`.github/workflows/jemalloc-hard.yml`](../../.github/workflows/jemalloc-hard.yml) |

## Soft vs hard gates

| Gate | Soft (scheduled) | Hard (PR blocking) |
|------|------------------|-------------------|
| SelfCheck (`jemalloc-check.ps1 -SelfCheck`) | `ops-load` job `jemalloc` (`continue-on-error: true`) | `jemalloc-hard.yml` SelfCheck job |
| `cargo build --features jemalloc` | `ops-load` job (`continue-on-error: true`) | `jemalloc-hard.yml` build job |
| Default / Windows builds unchanged (system allocator) | **done** | **done** |
| Continuous jemalloc profiling / production always-on jemalloc | **unpaid** | **unpaid** |
| Windows jemalloc parity | **unpaid** | **unpaid** |

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

Hermetic wiring tests (no jemalloc compile on default graph); see
[`tests/jemalloc_soft.rs`](../../tests/jemalloc_soft.rs) and
[`tests/jemalloc_hard.rs`](../../tests/jemalloc_hard.rs):

```powershell
cargo test --test jemalloc_soft --locked
cargo test --test jemalloc_hard --locked
```

Or filter both wrappers:

```powershell
cargo test jemalloc --locked
```

## Gate status

| Gate | Status |
|------|--------|
| Soft jemalloc SelfCheck | **done** |
| Default / Windows builds unchanged (system allocator) | **done** |
| Soft Ubuntu `--features jemalloc` CI (`continue-on-error`) | **done** |
| Blocking jemalloc-hard CI workflow | **done** |
| Continuous jemalloc profiling / production always-on jemalloc | **unpaid** |
| Windows jemalloc parity | **unpaid** |

## CI / scheduling

| Job | Workflow | Blocking? | Notes |
|-----|----------|-----------|-------|
| `jemalloc` | [`ops-load.yml`](../../.github/workflows/ops-load.yml) | **soft** (`continue-on-error: true`) | Scheduled SelfCheck + `--features jemalloc` build |
| `jemalloc-hard-selfcheck` / `jemalloc-hard-build` | [`jemalloc-hard.yml`](../../.github/workflows/jemalloc-hard.yml) | **blocking** | PR SelfCheck + `--features jemalloc` build on Ubuntu |

- **PR / push:** `cargo test --test jemalloc_soft` and `cargo test --test jemalloc_hard`
  exercise hermetic SelfCheck anchors (no jemalloc compile in the default test graph).
- **Scheduled soft job:** `jemalloc` in
  [`.github/workflows/ops-load.yml`](../../.github/workflows/ops-load.yml)
  (`continue-on-error: true`) runs SelfCheck + `cargo build --features jemalloc`
  on Ubuntu.
- **Blocking hard job:** [`jemalloc-hard.yml`](../../.github/workflows/jemalloc-hard.yml)
  runs the same SelfCheck + feature build on every PR without `continue-on-error`.

## Limitations

- Unix only — Windows (and other non-Unix) keeps the system allocator even if
  the feature flag is passed (dep is `cfg(unix)`).
- Soft + hard evidence only — does not force jemalloc into release artifacts or the
  default feature set.
- Does not replace RSS / allocation-budget / dhat smokes; those remain separate
  L8 companions.
- Continuous profiling push backends, always-on production jemalloc, and Windows
  jemalloc parity remain unpaid.
