# PLAN.md — SessionLedger claimable task DAG

Claimable tasks for autonomous agents. Reference FR-IDs from
[`docs/functional_requirements.md`](docs/functional_requirements.md). Effort:
**S** ≤ ~1 session, **M** ~1–2 sessions, **L** multi-session / multi-crate.

Do not claim a task already marked `done`. Prefer worktrees per `AGENTS.md`.

---

## Done (recent / v38 ladder)

| ID | Task | FR | Effort | Status | Notes |
|----|------|----|--------|--------|-------|
| T-001 | Domain skeleton + DESIGN + ports | FR-012 | L | done | P0 Discovery (DESIGN §7) |
| T-002 | OKF v1.0 spec + examples + conformance fixtures | FR-002, FR-013 | M | done | `docs/reference/OKF-*`, #51 |
| T-003 | OKF roundtrip smoke test | FR-013 | M | done | `tests/okf_roundtrip.rs`, #52 |
| T-004 | Brand / Lab-Coat identity + viewer theme | — | M | done | L101–L107 ladder |
| T-005 | L128 sl-daemon ETL + HTTP CLI | FR-001, FR-014 | L | done | #61 @ 4971260 |
| T-006 | POST /api/ingest + OKF validate | FR-002 | M | done | #45 |
| T-007 | Search/filter flags + GET /api/search | FR-003 | M | done | filter/tag |
| T-008 | Replay SSE + ReplayView | FR-004, FR-009 | M | done | #44 |
| T-009 | GET /api/metrics | FR-005 | S | done | #47 |
| T-010 | Archive/restore gzip | FR-006 | M | done | #43 |
| T-011 | Viewer Timeline / Search / LiveFeed | FR-007, FR-008, FR-010 | M | done | sl-viewer tabs |
| T-012 | process-compose + Makefile `dev` | FR-014 | S | done | #46 / #39 |
| T-013 | Agent entrypoint AGENTS.md + friction log | — | S | done | #40 |
| T-014 | FR catalog + PLAN/WORK_DAG + llms.txt + ops stubs | FR-015 | M | done | this lane (agent-obs) |
| T-020 | Link FR-IDs in key tests (smoke + daemon HTTP) | FR-013, FR-002, FR-014 | S | done | `tests/okf_roundtrip.rs`; `crates/sl-daemon` tests |
| T-021 | Structured tracing (`tracing` subscriber) in sl-daemon | FR-015 | M | done | `crates/sl-daemon`; `docs/ops/observability.md` |
| T-022 | Document + enforce log-level discipline (env) | FR-015 | S | done | `RUST_LOG`; `docs/ops/observability.md` |
| T-023 | Soft-goal OTel sketch (no SDK required yet) | FR-015 | S | done | optional `otel` feature in `crates/sl-daemon`; `docs/ops/observability.md` |
| T-025 | FR-gap / traceability note in CI docs | — | S | done | `docs/ops/runbook.md` |

---

## Todo — P0 (agent readiness / observability)

| ID | Task | FR | Effort | Status | Depends |
|----|------|----|--------|--------|---------|
| T-024 | Crash-detector MVP → unfinished Worklog projection | FR-011 | M | todo | T-005, T-001 |

---

## Todo — P1 (product depth from DESIGN)

| ID | Task | FR | Effort | Status | Depends |
|----|------|----|--------|--------|---------|
| T-030 | Contract / Dedup compilers + token estimator | FR-012 | M | todo | T-001 |
| T-031 | Bundle inject renderer (prompt form) | FR-012 | M | todo | T-030 |
| T-032 | rusqlite forge adapter (zstd, classify) | FR-001 | L | todo | T-005 |
| T-033 | Codex / Claude / Cursor JSONL adapters | FR-001 | L | todo | T-005 |
| T-034 | MemoryStore + Compressor + TraceSink adapters | FR-015 | L | todo | T-032 |
| T-035 | Dedup merge executor + lost-work localizer E2E | FR-011 | L | todo | T-024, T-030 |
| T-036 | Viewer unfinished / in-progress section | FR-011 | M | todo | T-024 |
| T-037 | Golden OKF corpus + snapshot tests | FR-013 | M | todo | T-003 |
| T-038 | Coverage ratchet toward DESIGN 85% gate | — | L | todo | T-032 |

---

## Claim protocol

1. Open a worktree from `origin/main` (or this branch if continuing).
2. Mark the task in your PR description: `Claims: T-0xx (FR-0yy)`.
3. Land acceptance refs (test path or endpoint) before flipping status to `done`.
