# SessionLedger Forward

## Goal

Drive the candidate from hosted CI success to a production-grade local release: verify the real macOS bundle and install path, dogfood automatic local-session discovery and ingestion, validate inbox/detail/replay UX and accessibility, then publish signed artifacts with rollback evidence.

## Current state

- Latest hosted qgate: `30679369252` (green evidence for the current release-gate lane).
- Candidate commit: `436e1618`.
- Source changes are preserved on `fix/sessionledger-forward-candidate`; do not reset, clean, or force-push.

## Active blocker

Local verification/build work is currently blocked by critically low disk space (`ENOSPC`) after generated Dioxus and browser-build artifacts. Remove only generated candidate artifacts (`target`, harness `node_modules`, and test results) before rebuilding; preserve source and Git evidence.

## Next gates

1. Restore disk headroom and rerun the real macOS bundle/install smoke.
2. Start the daemon and prove automatic discovery, ingestion, OKF publication, and live SSE against real local sessions.
3. Dogfood inbox/detail/replay/chat rendering, scrolling, responsive behavior, and accessibility on the installed app.
4. Capture bounded performance/memory evidence, then produce signed release artifacts and rollback/operations documentation.

