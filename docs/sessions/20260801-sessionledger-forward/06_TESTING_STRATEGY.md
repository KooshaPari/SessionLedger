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

## Required next matrix

1. **Build/install:** clean generated artifacts, build the real macOS target,
   install `.app` into a temporary Applications path, launch, uninstall, and
   verify rollback. Capture commands, SHA256, logs, and exit codes.
2. **Discovery:** seed representative Codex and normalized JSONL roots plus
   malformed/rotated files; verify auto-discovery, dedupe, provenance,
   consent, bounded errors, and restart recovery.
3. **End-to-end UI:** with real discovered data, verify inbox selection,
   detail tabs, transcript/replay chat, keyboard navigation, scrolling, and
   responsive widths in the installed app.
4. **Performance:** record cold/warm launch RSS and CPU, discovery latency,
   ETL throughput, SSE delivery latency, and 30-minute idle stability. Repeat
   after restart and compare against explicit budgets.
5. **Release:** verify checksum manifest, signature, artifact download, clean
   install health, and documented rollback before changing release status.

## Failure policy

A failed test blocks its owning gate. Preserve logs and artifacts, add the
   failure to `05_KNOWN_ISSUES.md`, fix forward, rerun the focused test, then
   rerun the affected qgate lane. Pixel/VLM inspection is unnecessary while
   deterministic contracts and browser assertions remain available.
