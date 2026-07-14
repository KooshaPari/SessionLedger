# Sandbox boundary checklist (process isolation)

Operational companion to [`docs/THREAT_MODEL.md`](../THREAT_MODEL.md) and
[`local-trust-boundary.md`](local-trust-boundary.md). Records **in-tree
evidence** for how `sl-daemon` is isolated at runtime (native, OCI container,
and CI) and which unpaid hardening gates remain open. This document does **not** claim maintainer 2FA enforcement (see [`branch-protection.md`](branch-protection.md)
— org-level 2FA is not verifiable from checkout).

## Trust boundaries (summary)

| Boundary | Untrusted side | Trusted side | Primary mitigations |
|----------|----------------|--------------|---------------------|
| File-watch ingest | `*.jsonl` on disk | ETL → `*.okf.json` | Owner-only watch dir; parser fail-closed |
| HTTP API | Local HTTP clients | Axum handlers → data dirs | Loopback bind default; `SL_API_KEY` for non-loopback |
| OCI runtime | Host-mounted session dirs | `sl` user inside container | Non-root `USER`, explicit `VOLUME` mounts |
| CI runners | PR/push code | Ephemeral `ubuntu-latest` jobs | `permissions: contents: read`; no `privileged: true` |

Full STRIDE-lite context: [`docs/THREAT_MODEL.md`](../THREAT_MODEL.md) §1 and
§5. Bind/auth matrix: [`local-trust-boundary.md`](local-trust-boundary.md).

## Process isolation evidence

### OCI image (`crates/sl-daemon/Containerfile`)

| Control | Status | Evidence |
|---------|--------|----------|
| Non-root runtime user | **done** | `useradd … sl` + `USER sl` (uid `10001`) |
| Data-dir volume contract | **done** | `VOLUME ["/data/sessions", "/data/out"]`; host mounts sessions read-only, out read-write |
| Loopback health probe | **done** | `HEALTHCHECK` curls `http://127.0.0.1:8080/healthz` (daemon default bind stays loopback) |
| Seccomp / AppArmor profile | **unpaid** | No custom seccomp JSON in-repo; rely on runtime defaults |
| Rootless-only enforcement | **unpaid** | Operator must choose rootless `container` / podman; not CI-gated |

Example run (host session dir in, OKF out):

```bash
container build -t sl-daemon:latest -f crates/sl-daemon/Containerfile .
container run --rm \
  -v "$HOME/.forge/sessions:/data/sessions:ro" \
  -v "$PWD/okf-out:/data/out" \
  sl-daemon:latest serve --watch /data/sessions --out /data/out
```

### Native / systemd (preferred deploy path)

| Control | Status | Evidence |
|---------|--------|----------|
| Dedicated service user | **done** | [`packaging/systemd/sessionledger-daemon.service`](../../packaging/systemd/sessionledger-daemon.service) `User=sessionledger` |
| Loopback HTTP bind | **done** | `SL_HTTP_BIND=127.0.0.1:8080` + `--http-bind ${SL_HTTP_BIND}` |
| TLS at edge only | **done** | [`packaging/nginx/sessionledger.conf`](../../packaging/nginx/sessionledger.conf), [`packaging/caddy/Caddyfile`](../../packaging/caddy/Caddyfile) reverse-proxy to loopback |

The daemon rejects non-loopback `--http-bind` without a non-empty `SL_API_KEY`
(startup deny). See [`local-trust-boundary.md`](local-trust-boundary.md).

### CI / GitHub Actions

| Control | Status | Evidence |
|---------|--------|----------|
| Least-privilege workflow permissions | **done** | [`.github/workflows/security.yml`](../../.github/workflows/security.yml) `permissions: contents: read` |
| No privileged containers on runners | **done** | No `privileged: true` in `.github/workflows/*` |
| Dedicated no-network job sandbox | **unpaid** | Hosted runners still reach the network for `cargo install` / registry fetches |
| Seccomp for CI steps | **unpaid** | Not configured; standard GitHub-hosted isolation only |

## Sandbox boundary checklist

| Gate | Status | Evidence / prerequisite |
|------|--------|-------------------------|
| THREAT_MODEL trust-boundary diagram | **done** | [`docs/THREAT_MODEL.md`](../THREAT_MODEL.md) §1 |
| Loopback bind + API-key policy documented | **done** | [`local-trust-boundary.md`](local-trust-boundary.md) |
| Containerfile non-root `USER` | **done** | [`crates/sl-daemon/Containerfile`](../../crates/sl-daemon/Containerfile) |
| Containerfile data `VOLUME` contract | **done** | same Containerfile |
| Systemd loopback bind sample | **done** | [`packaging/systemd/sessionledger-daemon.service`](../../packaging/systemd/sessionledger-daemon.service) |
| Sandbox boundary SelfCheck | **done** | `scripts/sandbox-boundary-check.ps1 -SelfCheck` |
| Custom seccomp profile for `sl-daemon` image | **unpaid** | Operator/runtime default only |
| Rootless-only OCI policy in CI | **unpaid** | Requires runner capability matrix |
| No-network CI sandbox for security jobs | **unpaid** | Would block `cargo install` / advisory DB refresh |

## SelfCheck (machine proof)

Docs + Containerfile + workflow anchors only — no container build, no network,
no `cargo`:

```powershell
pwsh ./scripts/sandbox-boundary-check.ps1 -SelfCheck
```

The script asserts:

- This checklist documents non-root `USER`, loopback bind, and data-dir `VOLUME`
- `crates/sl-daemon/Containerfile` retains `USER sl`, `VOLUME`, and loopback
  `HEALTHCHECK`
- Trust-boundary cross-links to `THREAT_MODEL.md` and `local-trust-boundary.md`
- CI workflows keep least-privilege defaults (no `privileged: true`)

Soft CI may run the same SelfCheck with `continue-on-error: true` from
[`.github/workflows/security.yml`](../../.github/workflows/security.yml).

## Related

- [`docs/THREAT_MODEL.md`](../THREAT_MODEL.md) — STRIDE-lite surfaces
- [`local-trust-boundary.md`](local-trust-boundary.md) — bind modes and API key
- [`hermetic-builds.md`](hermetic-builds.md) — SLSA L3 environment isolation (C06)
- [`crates/sl-daemon/README.md`](../../crates/sl-daemon/README.md) — run modes
