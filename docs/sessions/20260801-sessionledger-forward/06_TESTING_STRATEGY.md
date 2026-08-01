# Testing Strategy

## Current evidence

| Layer | Command/evidence | Result |
| --- | --- | --- |
| Hosted qgate | Run `30679369252` on current wip lineage | GREEN |
| Visual browser | Playwright visual suite | 15 passed |
| Responsive browser | Playwright responsive suite | 8 passed |
| Accessibility | Playwright axe suite | 46 passed |
| ETL/parser | Focused Rust transform tests | PASS |
| Atomic persistence | No-temp-artifact atomic OKF test | PASS |
| Live feed | `/readyz`, `/api/bundles`, and SSE event proof | PASS |
| CI policy | eval, package, fuzz, rootless matrix/no-net SelfChecks | PASS |
| Native discovery probe (2026-08-01) | 6,124 Codex inputs; 6,323 bundles; `/readyz` 200; `/api/bundles` >5s timeout; ~41% CPU/~919 MB RSS; launch agent stopped | BLOCKED |
| Discovery rescan regression | Commit `73e92c12`; `cargo test --lib ingestion::json_source --no-default-features` (4 passed) and `--locked` (4 passed) | PASS |
| Viewer bounded discovery | `cargo test -p sl-viewer --lib corpus_loader -- --nocapture` (7 passed); retain newest 512 during iteration | PASS (local; installed-app rerun required) |
| Cursor live transcript root | `cargo test --manifest-path crates/sl-daemon/Cargo.toml --bin sl-daemon discovery` (focused discovery tests; includes `~/.cursor/agent-transcripts`) | PASS (source-level; viewer parity pending) |

## Required next matrix

1. **Build/install:** clean generated artifacts, build the real macOS target,
   install `.app` into a temporary Applications path, launch, uninstall, and
   verify rollback. Capture commands, SHA256, logs, and exit codes.
2. **Discovery:** seed representative Codex and normalized JSONL roots plus
   malformed/rotated files; verify auto-discovery, dedupe, provenance,
   consent, bounded errors, and restart recovery.
   Include both Cursor roots (`projects` and `agent-transcripts`) and explicit
   user-provided ChatGPT/Claude/Gemini export fixtures. Do not treat a generic
   JSON fixture as proof of a hosted product's native export schema.
3. **End-to-end UI:** with real discovered data, verify inbox selection,
   detail tabs, transcript/replay chat, keyboard navigation, scrolling, and
   responsive widths in the installed app.
4. **Performance:** record cold/warm launch RSS and CPU, discovery latency,
   ETL throughput, SSE delivery latency, and 30-minute idle stability. Repeat
   after restart and compare against explicit budgets.
   The native probe above is a failure baseline, not an acceptance result;
   repeat it after the viewer bound and `73e92c12` with bounded roots and capture `/readyz`,
   `/api/bundles`, CPU, RSS, and timeout logs.
5. **Release:** verify checksum manifest, signature, artifact download, clean
   install health, and documented rollback before changing release status.

## Failure policy

A failed test blocks its owning gate. Preserve logs and artifacts, add the
   failure to `05_KNOWN_ISSUES.md`, fix forward, rerun the focused test, then
   rerun the affected qgate lane. Pixel/VLM inspection is unnecessary while
   deterministic contracts and browser assertions remain available.
