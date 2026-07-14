# Continuous profiling agent stub (L45 evidence)

SessionLedger ships **on-demand** loopback CPU profiling when
`SL_ENABLE_PPROF=1` (Wave-27 [#232](https://github.com/KooshaPari/SessionLedger/pull/232)).
This document describes the **soft continuous-profiling agent stub**: operator
docs, a one-shot agent loop script, and optional scheduled CI evidence that
reuses [`pprof-smoke.ps1`](../../scripts/pprof-smoke.ps1) without requiring
Windows CPU sampling.

## What exists today

| Layer | Status | Evidence |
|-------|--------|----------|
| On-demand unix pprof | **landed** | `GET /debug/pprof/profile` protobuf via `pprof-rs` when gated (#232) |
| Operator smoke | **landed** | [`scripts/pprof-smoke.ps1`](../../scripts/pprof-smoke.ps1) |
| Scheduled pprof smoke | **landed** | `pprof-smoke` job in [`.github/workflows/ops-load.yml`](../../.github/workflows/ops-load.yml) |
| Continuous agent sidecar | **stub** | [`scripts/continuous-profiling-agent.ps1`](../../scripts/continuous-profiling-agent.ps1) |
| Pyroscope / OTLP push | **unpaid** | `push_backend: none` in [`continuous-profiling.json`](continuous-profiling.json) |
| Windows CPU sampler | **unpaid** | `501` platform stub — SIGPROF sampler is unix-only |

See also the gated operator contract in
[`observability.md`](observability.md#local-pprof-style-profiling).

## Agent loop (intended)

The future continuous agent is a **loopback sidecar** (or in-process task) that
polls the same surface operators use manually:

```text
┌─────────────┐   SL_ENABLE_PPROF=1    ┌──────────────────┐
│ sl-daemon   │◄──── loopback only ───►│ profiling agent  │
│ /debug/pprof│   GET profile?seconds=N │ (stub today)     │
└─────────────┘                        └────────┬─────────┘
                                                │
                     unpaid ────────────────────┼──► Pyroscope / OTLP backend
                                                │
                     stub retains ──────────────┴──► local samples dir
```

1. **Discover** — attach to `http://127.0.0.1:<port>` (daemon must be loopback-bound).
2. **Sample** — `GET /debug/pprof/profile?seconds=N` on a fixed interval.
3. **Retain** — keep the last *N* protobuf files locally (stub only).
4. **Push** — upload to a profiling backend (**unpaid**; config keeps `push_backend: none`).

The stub script runs **one** poll cycle for CI/operator proof. A long-running
deployment would wrap the same steps in a `while` loop with backoff and disk
rotation.

## Stub configuration

[`continuous-profiling.json`](continuous-profiling.json) pins the agent knobs:

| Field | Default | Meaning |
|-------|---------|---------|
| `sample_seconds` | `1` | CPU window per `GET /debug/pprof/profile` |
| `poll_interval_seconds` | `30` | Intended spacing between agent polls (documented; stub runs once) |
| `retain_samples` | `3` | Local rotation ceiling for retained `.pb` files |
| `push_backend` | `none` | Profiling export target — **unpaid** until wired |

## How to run

### Self-check (hermetic; CI default)

```powershell
pwsh ./scripts/continuous-profiling-agent.ps1 -SelfCheck
```

Validates the JSON schema, doc anchors, and that `pprof-smoke.ps1` is present.

### One-shot agent cycle (unix)

Requires a built `sl-daemon`. On **Windows**, the script exits `0` with an
explicit skip — Windows pprof is not required for this evidence path.

```powershell
pwsh ./scripts/continuous-profiling-agent.ps1 -RunOnce -DaemonPath path/to/sl-daemon
```

On Linux/macOS the stub:

1. Starts a loopback daemon with `SL_ENABLE_PPROF=1`.
2. Fetches one CPU protobuf sample.
3. Writes it under a temp `samples/` directory and asserts non-empty bytes.
4. Delegates contract checks to `pprof-smoke.ps1`.

### Attach to a running daemon

```powershell
SL_ENABLE_PPROF=1 sl serve --http-bind 127.0.0.1:8080
pwsh ./scripts/continuous-profiling-agent.ps1 -AttachOnly -BaseUrl http://127.0.0.1:8080
```

When the gate is off (`404`), attach mode exits `0` with `skip` — same as
`pprof-smoke.ps1 -AttachOnly`.

## CI / scheduling

| Job | Workflow | Blocking? | Platform |
|-----|----------|-----------|----------|
| `pprof-smoke` | [`ops-load.yml`](../../.github/workflows/ops-load.yml) | yes | `ubuntu-latest` |
| `continuous-profiling-agent` | [`ops-load.yml`](../../.github/workflows/ops-load.yml) | **soft** (`continue-on-error: true`) | `ubuntu-latest` |

The soft `continuous-profiling-agent` job runs weekly (same cadence as other
`ops-load` smokes) plus `workflow_dispatch`. It exercises the stub agent loop
without gating merges.

## Unpaid gaps (explicit)

These remain **out of scope** for the stub; score stays partial until addressed:

1. **Pyroscope / OTLP profiling export** — no push client, auth, or backend wiring.
2. **Long-running agent deployment** — no systemd/compose unit; operators run the script manually.
3. **Windows CPU sampler parity** — `pprof-rs` SIGPROF path is unix-only; Windows CI does not assert protobuf profiles.
4. **Production enablement** — `SL_ENABLE_PPROF` stays off by default; continuous profiling is local-only evidence.

When export lands, update `push_backend` in
[`continuous-profiling.json`](continuous-profiling.json) and extend the agent
script with upload + retry semantics. Until then, on-demand unix pprof (#232)
plus this stub document the boundary.
