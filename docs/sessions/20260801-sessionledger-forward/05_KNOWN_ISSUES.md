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
4. **Native discovery saturation:** the 2026-08-01 device probe discovered
   6,124 Codex inputs and produced 6,323 bundles. `/readyz` returned HTTP 200,
   but `/api/bundles` exceeded a five-second probe timeout while the launch
   agent reached approximately 41% CPU and 919 MB RSS. The launch agent was
   stopped to restore host stability; this is a release blocker until bounded
   discovery and ingestion are re-probed.

## Important gaps

- Full installed-app dogfood against automatically discovered local sessions is
  pending; current live proof is a bounded daemon/Codex fixture-style run.
- Discovery coverage for every intended harness and consent/error UX needs a
  real-device matrix, not only parser fixtures.
- Launch/discovery/ingestion RSS, CPU, latency, and restart budgets lack a fresh
  measurement after the viewer and CI changes.
- Commit `73e92c12` removes the O(N^2) transcript-index rescan in
  `JsonCorpusSource::load_with_report`; it is a forward fix, but the live
  6,124-input workload still needs a post-fix daemon/viewer measurement.
- Hosted qgate run `30679369252` is green, but it is a web/CI gate and cannot
  close the local release holds above.

## Operating rule

Release status remains HOLD until all blocking items have fresh logs and a
repeatable verification command. Do not paper over failures with fixture-only
claims or broad cleanup.
