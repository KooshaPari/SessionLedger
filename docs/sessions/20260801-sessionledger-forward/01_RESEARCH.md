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

## Harness discovery and ingestion audit (2026-08-01)

| Harness/source | Automatic local root | Adapter/parser | Status and boundary |
| --- | --- | --- | --- |
| Codex | `~/.codex/sessions` | `CodexDir` (`session_meta`, `response_item`, `event_msg`; JSONL and optional Zstandard JSONL) | Automatic, local-only; malformed records are counted in `JsonIngestionReport` |
| Claude Code | `~/.claude/projects` | `ClaudeDir` (outer roles plus nested Anthropic text blocks) | Automatic, local-only; malformed records are counted |
| Cursor | `~/.cursor/projects`, `~/.cursor/agent-transcripts` | `CursorDir` (JSON/JSONL direct, nested `messages`/`conversation` records) | Automatic, local-only; the agent-transcripts root is now included by the daemon (`discovery.rs`) |
| Forge | `~/.forge/.forge.db` | `ForgeDb` (read-only SQLite, zstd/plain context) | Viewer-only behind `sqlite`; daemon `--watch` is JSONL-only, so Forge is not yet daemon-auto-ingested |
| ChatGPT web | User-selected export path | `ChatGptExport` + generic JSON mapping extraction | Explicit export file only; no cookies, browser automation, or authenticated scraping |
| Claude web | User-selected export path | `ClaudeExport` + generic JSON extraction | Explicit export file only; native export schema coverage still needs fixture evidence |
| Gemini web | User-selected export path | `GeminiExport` + generic JSON extraction | Explicit export file only; native export schema coverage still needs fixture evidence |

All adapters assign a corpus tag to the normalized `Session`; the OKF exporter
copies that tag and the session id into provenance. Automatic native discovery is
currently a trust-boundary convenience rather than a consent UI: it is limited
to known paths under `$HOME`, but it does not prompt before reading them. The
hosted-web adapters intentionally require an operator-provided export path, so
there is no credential or browser-scraping surface to audit. A future consent
surface should make per-root enablement and retention explicit before adding
more native stores.

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
