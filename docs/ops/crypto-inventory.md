# Cryptography inventory and remote TLS guidance

Status: **C02 L22 aspirational (1→2)** — documents what cryptography SessionLedger
uses today and how operators terminate TLS for a remote-style daemon deploy.
This is **not** a claim of full key-management service (KMS), envelope encryption,
or encryption-at-rest for application data.

Related: [`SECURITY.md`](../../SECURITY.md) (reporting + API-key rotation),
[`docs/THREAT_MODEL.md`](../THREAT_MODEL.md) (STRIDE-lite surfaces),
[`local-trust-boundary.md`](local-trust-boundary.md) (bind + `SL_API_KEY` policy).

## Cryptography inventory

| Use | Mechanism | Where | Notes |
|-----|-----------|-------|-------|
| Content dedup key | **SHA-256** (`sha2` crate) | [`src/domain/dedup.rs`](../../src/domain/dedup.rs) | Stable hash over normalized scope + topic slug; **hashing, not encryption** |
| Archive transport | **gzip** (`flate2` / archive path) | `crates/sl-daemon` archive/restore | Compression only; archives are **not** encrypted at rest |
| Ingest idempotency fingerprint | `std::collections::hash_map::DefaultHasher` | [`crates/sl-daemon/src/http.rs`](../../crates/sl-daemon/src/http.rs) | Process-local replay guard; **not** a cryptographic MAC |
| Outbound HTTP (CLI / viewer) | **rustls** via `reqwest` | `crates/sl-daemon`, optional `sl-viewer` | TLS to remote HTTPS endpoints when configured; loopback daemon URL stays plain HTTP |
| Release / supply-chain integrity | **SHA-256** checksums + optional **cosign** keyless signing | [`docs/ops/distribution.md`](distribution.md#release-integrity-signing-cosign) | Verifies downloaded binaries; separate from runtime app KMS |
| Commit / branch governance | GPG or SSH commit signatures | [`docs/ops/commit-signing.md`](commit-signing.md) | Repository hygiene; not daemon data encryption |

### Explicit non-goals (today)

- **No encryption-at-rest** for OKF bundles (`*.okf.json`), gzip archives, audit
  JSONL/SQLite, or episodic memory DB files. Protect data with OS filesystem ACLs,
  full-disk encryption, and backup policy on the host.
- **No in-process TLS** on `sl-daemon` HTTP. The daemon speaks plain HTTP on its
  bind address; operators own TLS termination.
- **No KMS, sealed secrets, or HSM integration** in-tree. `SL_API_KEY` is a
  single shared secret in the process environment — treat it like a host-local
  credential, not a managed vault record.

## Remote daemon deploy: TLS at the edge

SessionLedger remains a **single-user** companion. Exposing the HTTP API beyond
loopback requires `SL_API_KEY` and a **TLS-terminating reverse proxy** in front
of a loopback-bound daemon.

### Recommended layout

```text
[Client] --TLS--> [Caddy or nginx :443] --plain HTTP--> [sl-daemon 127.0.0.1:8080]
```

1. **Bind the daemon to loopback** — keep `SL_HTTP_BIND=127.0.0.1:8080` (systemd
   unit default in [`packaging/systemd/sessionledger-daemon.service`](../../packaging/systemd/sessionledger-daemon.service)).
   Do not publish `:8080` on a public interface when the proxy owns `:443`.
2. **Set a strong `SL_API_KEY`** — required for non-loopback binds; see
   [API key and secret handling](#api-key-and-secret-handling) below.
3. **Terminate TLS at the proxy** — use one of the in-repo samples:

| Proxy | Sample config | Operator notes |
|-------|---------------|----------------|
| Caddy | [`packaging/caddy/Caddyfile`](../../packaging/caddy/Caddyfile) | Automatic HTTPS (ACME) when DNS points at the host; set `SESSIONLEDGER_HOST` or edit the site address |
| nginx | [`packaging/nginx/sessionledger.conf`](../../packaging/nginx/sessionledger.conf) | HTTP→HTTPS redirect + `proxy_pass` to `127.0.0.1:8080`; supply cert paths |

4. **Enable and order services** — install `sessionledger-daemon`, then install
   and reload Caddy/nginx. Full walkthrough:
   [`distribution.md` § Traditional server TLS reverse proxy](distribution.md#traditional-server-tls-reverse-proxy-caddy--nginx).

Clients must send the shared secret on protected routes:

```text
Authorization: Bearer <SL_API_KEY>
X-API-Key: <SL_API_KEY>
```

`/healthz` and `/readyz` stay unauthenticated for load-balancer probes.

## API key and secret handling

| Topic | Guidance |
|-------|----------|
| Generation | `openssl rand -hex 32` or equivalent CSPRNG output; never reuse release checksums or commit SHAs as keys |
| Storage | Environment variable or service manager secret file **outside** the repo; see [`.env.example`](../../.env.example) placeholders only |
| Rotation | Stop callers → replace `SL_API_KEY` → restart `sl-daemon` → update client headers; burn the old value ([`SECURITY.md` § API keys](../../SECURITY.md#api-keys-and-secret-rotation)) |
| Repo hygiene | `scripts/env-example-check.ps1`, gitleaks, and TruffleHog in CI; do not commit live keys |
| Threat model | Non-loopback without a key is a **startup deny**; with a key, all `/api/*` require it — details in [`local-trust-boundary.md`](local-trust-boundary.md) |

## Machine verification (SelfCheck)

Hermetic doc anchor check (no daemon, no network):

```powershell
pwsh ./scripts/crypto-inventory-check.ps1 -SelfCheck
```

`-SelfCheck` asserts this page keeps the inventory table, the no-KMS disclaimer,
TLS sample paths, and cross-links to `SECURITY.md` / `local-trust-boundary.md`.
