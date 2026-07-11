# Functional requirements — SessionLedger

Machine-readable FR catalog for agent readiness (C03 / v38). Derived from
[`DESIGN.md`](DESIGN.md) and [`README.md`](../README.md). Each FR has a stable
ID, status, and acceptance references (tests and/or API surfaces).

Status: `done` = implemented and covered; `partial` = surface exists, gaps remain;
`todo` = not yet delivered.

---

| ID | Title | Status | Acceptance refs |
|----|-------|--------|-----------------|
| FR-001 | JSONL / corpus ingest into normalized Session | done | `crates/sl-daemon` ETL (`etl.rs`, `watcher.rs`); `tests/skeleton.rs`; `crates/sl-daemon/tests/pipeline.rs` |
| FR-002 | OKF bundle schema validation on ingest | done | `POST /api/ingest`; `crates/sl-daemon/src/validation.rs`; `sl validate` CLI |
| FR-003 | Bundle list + search/filter | done | `GET /api/bundles`, `GET /api/search`; `--since/--until/--model/--min-tokens/--tag/--limit`; `filter.rs`, `tag.rs` |
| FR-004 | Session replay via SSE | done | `GET /api/replay/:id`; `crates/sl-daemon/tests/sse_bridge.rs`; `sl replay`; `crates/sl-viewer/src/replay_view.rs` |
| FR-005 | Aggregated session metrics | done | `GET /api/metrics`; `crates/sl-daemon/src/metrics.rs` (unit tests) |
| FR-006 | Archive / restore (gzip) | done | `sl archive` / `sl restore`; `crates/sl-daemon/src/archive.rs` |
| FR-007 | Viewer Timeline | done | `crates/sl-viewer/src/timeline.rs` |
| FR-008 | Viewer Search | done | `crates/sl-viewer/src/search_view.rs` → `GET /api/search` |
| FR-009 | Viewer Replay | done | `crates/sl-viewer/src/replay_view.rs` → `GET /api/replay/:id` |
| FR-010 | Viewer LiveFeed | done | `crates/sl-viewer/src/live_feed.rs` → `GET /api/stream`; `docs/OKF-ROUNDTRIP.md` |
| FR-011 | Crash recovery / unfinished work surface | partial | DESIGN §5.1; T-024 detector + serializable projection in `src/domain/worklog.rs`; viewer integration remains T-036 |
| FR-012 | ContinuationBundle compile + inject gate | done | `src/domain/bundle.rs`; `src/distill`; Acceptance slice gate (DESIGN §2) |
| FR-013 | OKF roundtrip smoke (JSONL→daemon→OKF→viewer contract) | done | `tests/okf_roundtrip.rs`; `docs/OKF-ROUNDTRIP.md`; fixtures under `tests/fixtures/okf/` |
| FR-014 | Daemon liveness + local ops stack | done | `GET /healthz + GET /readyz`; `process-compose.yaml`; `make dev`; `docs/ops/runbook.md` |
| FR-015 | Observability stubs (metrics + future OTel) | partial | `/healthz`, `/api/metrics` live; OTel soft goal — `docs/ops/observability.md` |

---

## API surface (daemon)

| Method | Path | FR |
|--------|------|----|
| GET | `/healthz` | FR-014 |
| GET | `/api/bundles` | FR-003 |
| GET | `/api/search` | FR-003, FR-008 |
| GET | `/api/stream` | FR-010 |
| GET | `/api/replay/:id` | FR-004, FR-009 |
| POST | `/api/ingest` | FR-002 |
| GET | `/api/metrics` | FR-005, FR-015 |

Default listen: `SL_PORT=8080` (see `process-compose.yaml`).

## Notes

- FR-011's T-024 MVP is complete: normalized sessions are conservatively
  classified from transcript completion signals and projected into serialized
  `WorklogProjection` items. The FR remains partial until T-036 displays those
  items in the viewer's unfinished section.
- FR-015 documents current HTTP metrics; OpenTelemetry is an intentional soft
  goal, not a P0 blocker.
