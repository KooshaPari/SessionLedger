# Local trust boundary and HTTP admission

`sl-daemon` is a single-user desktop companion, not a network service. Its HTTP
listener accepts only loopback addresses (`127.0.0.0/8` or `::1`). `sl serve`
fails at startup if `--http-bind` is a wildcard or non-loopback address. Put an
authenticated, policy-enforcing proxy in front of a separately reviewed build
instead of exposing this API directly.

## Optional API-key gate

Loopback is the default trust boundary. If `SL_API_KEY` is set to a non-empty
value, mutating HTTP endpoints additionally require one of these headers:

```text
Authorization: Bearer <SL_API_KEY>
X-API-Key: <SL_API_KEY>
```

Unauthenticated mutating requests return `401` with the API error envelope. Read
endpoints such as `/healthz`, `/api/bundles`, and `/api/stream` remain governed
by the loopback bind restriction. Leave `SL_API_KEY` unset for the default local
desktop workflow; set it when a local automation, proxy, or browser extension
needs an explicit shared secret for writes.

## Ingest admission controls

`POST /api/ingest` has process-local body-size and concurrency limits:

| Environment variable | Default | Meaning |
|---|---:|---|
| `SL_INGEST_MAX_BODY_BYTES` | `1048576` (1 MiB) | Maximum HTTP request-body bytes |
| `SL_INGEST_MAX_CONCURRENCY` | `8` | Maximum in-flight ingest handlers |

Both values must be positive integers; invalid values stop server startup.
Requests above the body limit return `413`. Requests arriving while all ingest
permits are occupied return `429`. These are admission controls, not tenant
quotas or distributed rate limits.

Clients may send `Idempotency-Key` on `POST /api/ingest` to safely retry a
successful request while the daemon process is still running. The daemon stores
the key, raw-body hash, and success response in memory only: same key plus same
body returns the prior `200` validation response, while same key plus different
body returns `409` with the API error envelope. The cache is not persisted,
shared across processes, or retained after restart.

API transport errors use this small JSON envelope where the endpoint can adopt
it without changing successful response contracts:

```json
{"error":{"code":"payload_too_large","message":"ingest payload exceeds 1048576 bytes"}}
```

Validation failures remain `422` with the existing validation-result JSON so
current clients retain actionable field diagnostics.

## Structured audit events

Mutating or persistence-adjacent local operations emit `tracing` events and append
the same actor/action record to a durable local audit sink. The sink is append-only:
the daemon never truncates, updates, or deletes historical records.

### Backend selection

| Environment variable | Default | Meaning |
|---|---|---|
| `SL_AUDIT_BACKEND` | `jsonl` | `jsonl` writes newline-delimited JSON; `sqlite` uses an append-only SQLite table (requires `sl-daemon` built with `--features sqlite`) |

### Storage paths

When `SL_DATA_DIR` is set, audit files live under that directory. Otherwise paths
are relative to the command output directory (`serve --out`, archive/restore
`--data-dir`, etc.).

| Backend | Path |
|---|---|
| `jsonl` (default) | `<data_dir>/audit/events.jsonl` |
| `sqlite` | `<data_dir>/audit/events.db` |

Concrete examples:

- `$SL_DATA_DIR/audit/events.jsonl` when `SL_DATA_DIR` is set and backend is `jsonl`.
- `<serve --out>/audit/events.jsonl` for `sl serve` when `SL_DATA_DIR` is unset.
- `<data_dir>/audit/events.jsonl` for archive/restore commands that already take
  a `--data-dir`.

### Record shape and durability

Each JSONL line is a standalone JSON object written with append/create semantics,
a flush, and `sync_data`. The SQLite backend uses WAL mode and `INSERT` only into
`audit_events`; there are no daemon-driven `UPDATE` or `DELETE` statements.
Records include:

- `event_kind="audit"` in structured logs (not duplicated in the JSONL/SQLite payload)
- `actor="local"`
- `action` such as `ingest`, `export`, `archive`, or `restore`
- `outcome` and a non-payload `reason` or `resource`
- `request_id` and Unix-millisecond `timestamp`

Enable newline-delimited JSON with the `json-logs` feature and
`SL_LOG_FORMAT=json`. Events deliberately omit transcript and ingest-body
contents. Operators own retention, rotation, backup, and access control for the
audit store; rotation should move or copy files without asking `sl-daemon` to
rewrite historical records.
