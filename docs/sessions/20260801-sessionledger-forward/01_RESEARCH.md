# Research and Evidence

## Scope

This note records the repository and hosted-gate findings used for the
SessionLedger forward release lane. It is intentionally evidence-first:
fixture or probe success is not treated as installed-app or release proof.

## Repository findings

- `crates/sl-daemon/src/etl.rs` now recognizes plain Codex `session_meta`
  JSONL alongside compressed Codex streams and normalized JSONL inputs.
- OKF publication is staged to a process/sequence-specific temporary file and
  renamed into place. This prevents readers from observing partial documents
  (`5124878c`, test `atomic_okf_write_publishes_complete_document_without_temp_artifacts`).
- The daemon SSE bridge subscribes before transformation and publishes bundle
  events after persistence. A live local proof observed `/readyz=200`, one
  discovered bundle, one OKF document, and three SSE bundle events.
- Viewer contracts cover bounded bundle panes, direct-child panel sizing,
  scrollable list/transcript content, replay chat content, and stable tab-panel
  selectors (`72bf4a23`, `1cafdc89`, `72c667b2`, `bd77e09b`).
- CI SelfChecks restored the evaluation manifest, fuzz smoke, rootless matrix,
  rootless/no-net policy, and packaging target-suffix protections
  (`168033ba`, `5b4597dd`, `067b8200`, `fec41bcc`, `df21656d`).
- The visual token contract now matches the intentional system UI font
  (`ba541170`); this removes a stale serif assertion rather than changing the
  production token.

## Hosted evidence

- qgate run `30679369252` is green on
  `wip/20260801T0207-18c78c4ef45ab4f0` at candidate `436e1618`.
- Browser coverage: visual `15` passed, responsive `8` passed, axe/a11y `46`
  passed. This proves the hosted web surface and gate anchors, not a signed
  macOS install.

## Research commands and constraints

Focused Rust ETL and SSE tests, the eval/package/rootless/fuzz SelfChecks, and
the hosted qgate are the current reproducible evidence. The local Dioxus and
browser builds exhausted disk (`ENOSPC`); generated artifacts must be removed
before another build. No source, worktree, or Git evidence should be deleted.

## Open verification

The real macOS bundle/install path, installed-app dogfood, automatic discovery
across all intended harnesses, bounded memory/performance measurements, and
artifact signing/publish/rollback proof remain open gates.
