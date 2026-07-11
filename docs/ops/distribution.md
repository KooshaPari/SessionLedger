# Distribution — SessionLedger

How SessionLedger is packaged, where data lives, how to uninstall cleanly, and
how release integrity signing works, and what remains **deferred** for
platform code-signing / notarization. Complements
[`packaging/README.md`](../../packaging/README.md) and the tag-driven release
workflow [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

Issue tracker: [#66](https://github.com/KooshaPari/SessionLedger/issues/66)
(signing + installer path).

---

## Release channels (current)

| Channel | Status | Notes |
|---------|--------|-------|
| GitHub Releases (`v*` tags) | **Active** | `release.yml` builds archives, publishes `SHA256SUMS` + a CycloneDX SBOM, and attempts GitHub provenance attestation and keyless cosign signing |
| Local packaging scaffold | **Active** | `make -C packaging package-macos` / `package-linux` / `package-windows` |
| Installer script | **Draft, not published** | `scripts/install.sh` installs checksum-verified Linux/macOS GitHub Release artifacts |
| brew / crates.io / MSI / DMG / AppImage | Deferred | Soft distribution goals |
| Tray / menubar / auto-update | Soft / N-A | Deliberate daemon + foreground viewer scope; see [ADR 0001](../adr/0001-desktop-companion-scope.md) |

### Release matrix (CI)

From `.github/workflows/release.yml` (on `push` tags `v*`):

| Target triple | Runner | Archive | Status |
|---------------|--------|---------|--------|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `.tar.gz` | Shipped |
| `x86_64-apple-darwin` | `macos-latest` | `.tar.gz` | Shipped |
| `aarch64-apple-darwin` | `macos-latest` | `.tar.gz` | Shipped |
| `x86_64-pc-windows-msvc` | `windows-latest` | `.zip` (`sl-viewer.exe`) | **Shipped** |

Asset names:

```text
sl-viewer-<tag>-x86_64-unknown-linux-gnu.tar.gz
sl-viewer-<tag>-x86_64-apple-darwin.tar.gz
sl-viewer-<tag>-aarch64-apple-darwin.tar.gz
sl-viewer-<tag>-x86_64-pc-windows-msvc.zip
SHA256SUMS
session-ledger.cdx.json
SHA256SUMS.sigstore.json
```

`session-ledger.cdx.json` is the CycloneDX SBOM and is required for the Release
job to succeed. GitHub build provenance and `SHA256SUMS.sigstore.json` are
best-effort. OIDC, attestation, cosign installation, signing, or upload failures
do not fail or retract the otherwise valid unsigned Release.

**Windows matrix status:** release CI produces an unsigned
`x86_64-pc-windows-msvc` zip. On a Windows host, the local
`package-windows` target produces an equivalent versioned portable ZIP via
`scripts/package-windows.ps1`. Day-to-day `ci.yml` remains Linux-only
(Phenotype billing policy); Windows coverage is release-tag scoped, not PR CI.
An MSI/NSIS installer and Authenticode signing remain explicitly deferred.

---

## Data directories

SessionLedger does **not** yet declare XDG / AppData config homes in code.
Paths are env- and flag-driven. Prefer one explicit root so uninstall can wipe
a single tree.

### Local compose (`make dev`)

Root [`process-compose.yaml`](../../process-compose.yaml) sets:

| Variable | Default | Role |
|----------|---------|------|
| `SL_DATA_DIR` | `./.sl-data` | Local data root for the compose stack (bundles / out dir readiness) |
| `SL_PORT` | `8080` | Daemon HTTP bind port |

Create the directory before readiness probes succeed:

```bash
mkdir -p "${SL_DATA_DIR:-./.sl-data}"
export SL_DATA_DIR="${SL_DATA_DIR:-./.sl-data}"
```

`/readyz` expects the configured out / data directory to exist (see
[`runbook.md`](runbook.md)).

### Daemon watch / out (crate compose & CLI)

The daemon crate’s own compose file and CLI use explicit watch/out paths:

| Variable / flag | Typical default | Role |
|-----------------|-----------------|------|
| `SL_WATCH_DIR` / `--watch` | `./sessions` or `~/.forge/sessions` | Incoming `*.jsonl` transcripts |
| `SL_OUT_DIR` / `--out` | `./okf-out` | Written `<session-id>.okf.json` bundles |
| `--data-dir` (archive / restore / validate) | `.` | Bundle + `archive/<year>/<month>/` tree |

When aligning with `SL_DATA_DIR`, a common layout is:

```text
$SL_DATA_DIR/
  sessions/          # optional watch input
  out/               # OKF bundles (*.okf.json)
  archive/YYYY/MM/   # gzip archives from `sl archive`
```

Map with:

```bash
export SL_DATA_DIR="${SL_DATA_DIR:-$PWD/.sl-data}"
export SL_WATCH_DIR="${SL_WATCH_DIR:-$SL_DATA_DIR/sessions}"
export SL_OUT_DIR="${SL_OUT_DIR:-$SL_DATA_DIR/out}"
mkdir -p "$SL_WATCH_DIR" "$SL_OUT_DIR"
```

### Container / OCI

[`crates/sl-daemon/Containerfile`](../../crates/sl-daemon/Containerfile) mounts:

| Path in image | Host mount example |
|---------------|--------------------|
| `/data/sessions` | `$HOME/.forge/sessions` (ro) |
| `/data/out` | `$PWD/okf-out` or a named volume |

Volumes are owned by non-root user `sl` (uid `10001`).

### Future XDG / AppData (not implemented)

Documented intent only — no code yet:

| OS | Config (future) | Data (future) |
|----|-----------------|---------------|
| Linux | `$XDG_CONFIG_HOME/sessionledger` | `$XDG_DATA_HOME/sessionledger` |
| macOS | `~/Library/Application Support/SessionLedger` | same or `~/Library/Caches/SessionLedger` |
| Windows | `%APPDATA%\SessionLedger` | `%LOCALAPPDATA%\SessionLedger` |

Until then, treat `SL_DATA_DIR` (and `--out` / `--data-dir`) as the SSOT.

---

## Install

### From GitHub Release

1. Download the archive for your platform from the latest `v*` Release.
2. Extract; run `./sl-viewer` (Unix) or `sl-viewer.exe` (Windows).
3. Start `sl-daemon` separately (from source or a future daemon artifact) with
   an explicit `--watch` / `--out` (or compose + `SL_DATA_DIR`).

### From source (local packaging)

```bash
make -C packaging package-macos   # → packaging/dist/SessionLedger.app
make -C packaging package-linux   # → packaging/dist/linux/SessionLedger
# On Windows:
make -C packaging package-windows # → packaging/dist/sl-viewer-v<version>-x86_64-pc-windows-msvc.zip
```

See [`packaging/README.md`](../../packaging/README.md).

### Installer script draft (Linux / macOS)

The repository includes a documented but unpublished install-channel stub:

```bash
curl -fsSL https://raw.githubusercontent.com/KooshaPari/SessionLedger/main/scripts/install.sh | sh
```

It resolves the latest GitHub Release, downloads the matching archive and
`SHA256SUMS`, verifies the archive, and installs `sl-viewer` to
`~/.local/bin`. Set `SL_VERSION=v0.1.0` to pin a release or `SL_INSTALL_DIR`
to change the destination. Review the script before piping it to a shell.

This is not a Homebrew formula or hosted installer service, and it does not
enable automatic updates.

---

## Uninstall / cleanliness

There is no MSI/pkg uninstaller yet. Manual cleanup:

1. **Stop processes** — `make dev-down` or kill `sl-daemon` / `sl-viewer`.
2. **Remove binaries** — delete extracted Release folders, `packaging/dist/`,
   and any copied `.app` / `.exe`.
3. **Remove data** — delete the data root you configured:

```bash
# Local compose default
rm -rf ./.sl-data

# If you used a custom root
rm -rf "$SL_DATA_DIR"

# Common CLI outs (only if you used these paths)
rm -rf ./okf-out ./sessions
```

4. **macOS quarantine leftovers** — if you copied an `.app` into
   `/Applications`, remove that copy as well.
5. **Container volumes** — `podman volume rm …` / `docker volume rm …` for any
   named `sl-data` / out volumes you created.

Do **not** leave orphaned `.sl-data` trees in clone roots after experiments;
prefer a single `SL_DATA_DIR` outside the repo when testing long-lived data.

---

## Release integrity signing (cosign)

On a `v*` tag, release CI publishes `SHA256SUMS`, then a dependent best-effort
job requests a short-lived GitHub OIDC identity and runs cosign keyless
`sign-blob`. When successful, the same Release gains
`SHA256SUMS.sigstore.json`, which contains the signature, certificate, and
transparency-log verification material.

The signing job has `id-token: write` and `contents: write`, but each external
operation fails soft. A cosign installation, OIDC permission, signing, or
upload failure emits a workflow notice while leaving the unsigned archives and
checksums available.

### Verify a Release

Download the archive, `SHA256SUMS`, and `SHA256SUMS.sigstore.json` into one
directory. Replace `<tag>` with the exact Release tag:

```bash
cosign verify-blob \
  --bundle SHA256SUMS.sigstore.json \
  --certificate-identity "https://github.com/KooshaPari/SessionLedger/.github/workflows/release.yml@refs/tags/<tag>" \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  SHA256SUMS

sha256sum --check SHA256SUMS
```

On Windows, use `Get-FileHash -Algorithm SHA256` to compare an archive with its
entry in `SHA256SUMS` after cosign verifies the checksums file.

Cosign proves the checksums file was signed by this repository's tag workflow;
the checksum comparison then binds the downloaded archive to that signed file.

### Verify GitHub build provenance

Install and authenticate [GitHub CLI](https://cli.github.com/), then download
the archive you intend to run. Verify that GitHub's attestation store contains
provenance issued by this repository:

```bash
gh attestation verify \
  sl-viewer-<tag>-x86_64-unknown-linux-gnu.tar.gz \
  --repo KooshaPari/SessionLedger
```

Substitute the exact downloaded archive name on macOS or Windows. A successful
result verifies the artifact digest and its GitHub Actions provenance. It does
not replace the cosign + `SHA256SUMS` checks above, platform code signing, or
review of the source.

Provenance is intentionally fail-soft during this rollout. If
`gh attestation verify` reports that no attestation exists, treat provenance as
unavailable for that Release and rely on cosign/checksum verification; do not
interpret the absence as a successful provenance check.

## Platform code-signing & notarization — DEFERRED

Release and packaging scaffolds still ship **unsigned binaries / `.app`
bundles**. The following platform trust paths remain deferred under #66:

- Apple Developer ID signing, `notarytool` notarization, and stapling
- Windows Authenticode (`signtool`) for `sl-viewer.exe`

### macOS Gatekeeper notes (unsigned builds)

Until notarization lands, Gatekeeper will treat downloaded builds as untrusted:

| Symptom | Mitigation (dev / internal only) |
|---------|----------------------------------|
| “App can’t be opened because Apple cannot check it for malicious software” | System Settings → Privacy & Security → Open Anyway; or right-click → Open |
| Quarantine attribute on download | `xattr -dr com.apple.quarantine /path/to/SessionLedger.app` |
| `spctl --assess` fails | Expected for unsigned artifacts |

Do **not** instruct end users to disable Gatekeeper globally. Prefer waiting for
signed+notarized Releases, or build from source (`cargo build -p sl-viewer`).

### Windows SmartScreen

Unsigned `sl-viewer.exe` from GitHub Releases may show SmartScreen warnings.
“More info → Run anyway” is acceptable for internal testing; Authenticode is
the production path (deferred).

---

## Related

- [`packaging/README.md`](../../packaging/README.md) — local Make targets
- [`runbook.md`](runbook.md) — `make dev`, health probes
- [`observability.md`](observability.md) — `/healthz`, `/readyz`, metrics
- [`SECURITY.md`](../../SECURITY.md) — supply chain / SBOM
- Issue [#66](https://github.com/KooshaPari/SessionLedger/issues/66) — signing + installers
