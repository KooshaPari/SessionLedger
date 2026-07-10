# Distribution — SessionLedger

How SessionLedger is packaged, where data lives, how to uninstall cleanly, and
what is **deferred** for code-signing / notarization. Complements
[`packaging/README.md`](../../packaging/README.md) and the tag-driven release
workflow [`.github/workflows/release.yml`](../../.github/workflows/release.yml).

Issue tracker: [#66](https://github.com/KooshaPari/SessionLedger/issues/66)
(signing + installer path).

---

## Release channels (current)

| Channel | Status | Notes |
|---------|--------|-------|
| GitHub Releases (`v*` tags) | **Active** | `release.yml` builds archives and attaches them via `softprops/action-gh-release` |
| Local packaging scaffold | **Active** | `make -C packaging package-macos` / `package-linux` |
| brew / crates.io / MSI / DMG / AppImage | Deferred | Soft goals; not required for Wave-2 C11 lift |
| Tray / menubar / auto-update | Soft / N-A | Product remains daemon + desktop viewer |

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
```

**Windows matrix status:** release CI already produces an unsigned
`x86_64-pc-windows-msvc` zip. The local `packaging/Makefile` scaffold covers
macOS `.app` + Linux binary only — there is no `package-windows` Make target
yet. Day-to-day `ci.yml` remains Linux-only (Phenotype billing policy); Windows
coverage is release-tag scoped, not PR CI.

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
```

See [`packaging/README.md`](../../packaging/README.md).

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

## Code-signing & notarization — DEFERRED

**Status:** explicitly deferred (C11 L112). Release and packaging scaffolds ship
**unsigned** binaries / `.app` bundles. No `codesign`, `notarytool`,
`signtool`, Authenticode, or cosign steps run in CI today.

### Soft goals (tracked in #66)

- CI Apple Developer ID signing + notarization + staple for macOS archives / `.app`
- Windows Authenticode (`signtool`) for `sl-viewer.exe`
- Optional cosign / Sigstore attestation of Release assets (checksums in the
  Release body are the interim integrity signal)

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
