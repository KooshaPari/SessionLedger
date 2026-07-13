# Local trust boundary and HTTP admission

`sl-daemon` is a single-user desktop companion, not a network service. Its HTTP
listener accepts only loopback addresses (`127.0.0.0/8` or `::1`). `sl serve`
fails at startup if `--http-bind` is a wildcard or non-loopback address. Put an
authenticated, policy-enforcing proxy in front of a separately reviewed build
instead of exposing this API directly.

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

API transport errors use this small JSON envelope where the endpoint can adopt
it without changing successful response contracts:

```json
{"error":{"code":"payload_too_large","message":"ingest payload exceeds 1048576 bytes"}}
```

Validation failures remain `422` with the existing validation-result JSON so
current clients retain actionable field diagnostics.

## Structured audit events

Mutating or persistence-adjacent local operations emit `tracing` events and append
the same actor/action record to a local JSONL audit file. The durable sink is:

- `$SL_DATA_DIR/audit/events.jsonl` when `SL_DATA_DIR` is set.
- `<serve --out>/audit/events.jsonl` for `sl serve` when `SL_DATA_DIR` is unset.
- `<data_dir>/audit/events.jsonl` for archive/restore commands that already take
  a `--data-dir`.

Each line is a standalone JSON object written with append/create semantics and a
flush plus `sync_data` call. The daemon never truncates or rewrites this file.
Records include:

- `event_kind="audit"`
- `actor="local"`
- `action` such as `ingest`, `export`, `archive`, or `restore`
- `outcome` and a non-payload `reason` or `resource`
- `request_id` and Unix-millisecond `timestamp`

Enable newline-delimited JSON with the `json-logs` feature and
`SL_LOG_FORMAT=json`. Events deliberately omit transcript and ingest-body
contents. Operators own retention, rotation, backup, and access control for the
JSONL audit path; rotation should move or copy the file without asking
`sl-daemon` to rewrite historical records.
