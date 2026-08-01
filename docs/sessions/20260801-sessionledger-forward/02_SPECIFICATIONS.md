# Forward Release Specifications

## Objective

Ship a trustworthy SessionLedger macOS build that discovers local harness
sessions without manual selection, persists complete OKF records, renders an
inbox/detail/replay experience, and has reproducible signed artifacts.

## Functional acceptance

1. **Discovery:** daemon scans supported local session roots, identifies Codex
   `session_meta` streams and normalized JSONL, deduplicates by stable source
   identity, and reports provenance plus parse failures.
2. **Persistence:** every accepted session transforms to one complete OKF
   document; publication is atomic; restart/replay does not create partial or
   duplicate records.
3. **Live feed:** SSE subscribers receive deterministic bundle events after
   persistence; disconnect/reconnect is bounded and observable.
4. **Viewer:** inbox list, selected detail, transcript/replay chat, and tabs
   remain scrollable at desktop and responsive widths; no permanently empty
   pane or overflow outside the workspace.
5. **Accessibility:** keyboard navigation, semantic labels, focus visibility,
   contrast, and axe checks remain green.
6. **Release:** macOS `.app` bundle installs into Applications, launches the
   real daemon path, and can be rolled back. Checksums, provenance, and signing
   metadata are published with artifacts.

## Quality gates and current state

| Gate | Evidence | State |
| --- | --- | --- |
| Hosted qgate | Run 30679369252; visual 15, responsive 8, axe 46 | PASS |
| ETL/parser | Focused Rust ETL tests; Codex plain/compressed fixtures | PASS |
| Atomic OKF/SSE | Atomic-write test plus live local SSE proof | PASS |
| Viewer contracts | Scroll, replay, panel sizing, responsive tests | PASS (hosted) |
| Installed macOS dogfood | Real bundle, daemon, local sessions | OPEN; ENOSPC |
| Performance/memory | Bounded launch, discovery, ingestion measurements | OPEN |
| Signing/publish/rollback | Release artifacts and recovery drill | OPEN |

## Non-functional constraints

- Preserve Git provenance; no reset, force-push, or broad cleanup.
- Keep generated cleanup limited to candidate `target`, browser `node_modules`,
  and test-result directories when disk pressure requires it.
- Do not treat fixtures, probes, or hosted web checks as installed-app proof.
- Surface consent, source path, timestamp, parser version, and error state for
  every discovered session.
