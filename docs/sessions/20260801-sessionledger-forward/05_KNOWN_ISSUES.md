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
   stopped to restore host stability. The viewer now bounds retained startup
   sessions to 128 newest records by default (256 hard maximum for an explicit
   override) while still counting all discovered inputs; this is a release
   blocker until the updated artifact is re-probed on the same corpus.

   The pre-window installed viewer (candidate `f9c1ddec`, default 128 retained
   sessions) reproduced the host-memory failure: PID 97520 measured roughly
   85 MB RSS at 5 s, 283 MB at 10 s, 956 MB peak at 15 s, 592 MB at 40 s, and
   91 MB at 60 s. A physical-footprint sample reported 343.7 MB current and
   1.4 GB peak. This is baseline evidence only; the windowed build must be
   installed and measured on the same corpus before the hold can move.

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
- The viewer bound is implemented in `crates/sl-viewer/src/corpus_loader.rs`
  by retaining only the newest 128 sessions during iteration and surfacing a
  visible count notice. `SESSION_LEDGER_VIEWER_MAX_SESSIONS` accepts an
  explicit positive override but clamps at 256. It is not release evidence
  until a fresh installed app measurement confirms the host memory budget.
- Native JSON/JSONL discovery now enumerates transcript paths as an index,
  ranks them by filesystem modification time (with a deterministic ID
  tie-break), and parses only the newest bounded window. `discovered_count`
  therefore reports indexed transcript records, including records not parsed
  in this startup window; it does not claim that historical payloads were
  validated. Empty/malformed records in the parsed window remain warnings.
- Hosted qgate run `30679369252` is green, but it is a web/CI gate and cannot
  close the local release holds above.

## Operating rule

Release status remains HOLD until all blocking items have fresh logs and a
repeatable verification command. Do not paper over failures with fixture-only
claims or broad cleanup.
