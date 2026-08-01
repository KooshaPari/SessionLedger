# Known Issues and Release Holds

## Blocking

1. **ENOSPC:** local Dioxus/browser build artifacts exhausted the candidate
   volume. Only generated `target`, harness `node_modules`, and test-results
   directories may be removed. Source, worktrees, and Git evidence are
   preserved. Reclaim space and record before/after `df` output.
2. **Real macOS install unverified:** hosted web tests and focused Rust tests do
   not prove that the real `.app` bundle builds, installs in Applications,
   launches the daemon, or rolls back cleanly.
3. **Signing/publish deferred:** checksum/signing readiness anchors exist, but
   no signed artifact, release upload, verification, or rollback drill has
   been captured for this candidate.

## Important gaps

- Full installed-app dogfood against automatically discovered local sessions is
  pending; current live proof is a bounded daemon/Codex fixture-style run.
- Discovery coverage for every intended harness and consent/error UX needs a
  real-device matrix, not only parser fixtures.
- Launch/discovery/ingestion RSS, CPU, latency, and restart budgets lack a fresh
  measurement after the viewer and CI changes.
- Hosted qgate run `30679369252` is green, but it is a web/CI gate and cannot
  close the local release holds above.

## Operating rule

Release status remains HOLD until all blocking items have fresh logs and a
repeatable verification command. Do not paper over failures with fixture-only
claims or broad cleanup.
