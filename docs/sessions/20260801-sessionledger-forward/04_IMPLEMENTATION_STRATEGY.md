# Implementation Strategy

## Boundaries

```text
local roots -> discovery/parser -> normalized session -> OKF repository
                                             |             |
                                             +--> SSE -----+--> viewer API
                                                               |
                                                               +--> inbox/detail/replay
```

- Keep filesystem discovery and format parsing in daemon ETL adapters; emit a
  normalized session model with source path, harness, timestamps, and parser
  diagnostics.
- Write OKF to a unique temporary sibling, flush/close, then rename atomically.
  Readers and SSE subscribers observe only complete documents.
- Keep the SSE bridge after persistence, with bounded subscriber queues and
  explicit disconnect/reconnect diagnostics.
- Keep viewer state as one selected session plus independently scrollable list,
  detail, transcript, and replay regions. Use semantic tabs and stable direct
  child selectors; avoid layout rules that depend on generated class names.

## Performance and resilience

- Bound discovery concurrency and file reads; use incremental offsets or file
  identity to avoid rescanning unchanged sessions.
- Apply bounded caches for metadata and transformed records, with invalidation
  on file identity/mtime changes and no unbounded in-memory transcript copies.
- Measure launch RSS, steady-state RSS, CPU, discovery latency, ETL latency,
  SSE delivery, and restart recovery with a repeatable local harness.
- On daemon restart, replay persisted OKF state idempotently; surface malformed
  sources without taking down healthy ingestion.

## Security and release

- Record consent/provenance for each root and session; redact secrets in logs
  and keep source paths scoped to the local operator.
- Use least-privilege packaging, pinned CI actions, checksum verification, and
  signing only after the unsigned bundle passes install/dogfood checks.
- Publish a signed manifest plus rollback instructions; retain prior artifact
  until a post-install health and ingestion check succeeds.
