# SessionLedger Packaging

Build and package the `sl-viewer` desktop app for distribution. Full ops
guide (data dirs, uninstall, signing deferral, Windows matrix):
[`docs/ops/distribution.md`](../docs/ops/distribution.md).
Mobile presence is intentionally Soft / N-A for the current desktop-plus-daemon
scope; see [`docs/adr/0002-mobile-presence.md`](../docs/adr/0002-mobile-presence.md).
Install channel status is tracked in [`channels.md`](channels.md). A traditional
Linux service unit lives at
[`systemd/sessionledger-daemon.service`](systemd/sessionledger-daemon.service).

## Prerequisites

- Rust toolchain (rustup)
- For macOS: Xcode command line tools
- For Linux: standard build essentials; `appimagetool` for AppImage or
  `dpkg-deb` for the Debian scaffold
- For Windows: PowerShell 5.1+ and the MSVC Rust target/toolchain; WiX v4 only
  when evaluating the MSI scaffold

## Usage

```sh
# macOS .app bundle
make -C packaging package-macos

# Linux binary
make -C packaging package-linux

# Windows installable/portable ZIP (run on Windows)
make -C packaging package-windows
# Equivalent:
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/package-windows.ps1

# Linux installer candidates (developer scaffolds, not published)
./packaging/linux/package-appimage.sh
./packaging/linux/package-deb.sh

# macOS + Linux (host-local scaffold)
make -C packaging package-all
```

## Output (local Make)

| Platform | Output |
|----------|--------|
| macOS    | `packaging/dist/SessionLedger.app` |
| Linux    | `packaging/dist/linux/SessionLedger` |
| Windows  | `packaging/dist/sl-viewer-v<version>-x86_64-pc-windows-msvc.zip` (includes per-user install/uninstall scripts) |

## Release matrix (GitHub Actions)

Tag push (`v*`) builds via [`.github/workflows/release.yml`](../.github/workflows/release.yml):

| Target | Archive | CI validation | Local Make? |
|--------|---------|---------------|-------------|
| Linux `x86_64-unknown-linux-gnu` | `.tar.gz` | Download, extract, binary `--version` smoke on `ubuntu-latest` | `package-linux` |
| macOS Intel `x86_64-apple-darwin` | `.tar.gz` | Build/archive only | `package-macos` |
| macOS ARM `aarch64-apple-darwin` | `.tar.gz` | Build/archive only | `package-macos` (on Apple Silicon host) |
| Windows `x86_64-pc-windows-msvc` | `.zip` (`sl-viewer.exe`) | Download, extract, binary `--version` smoke on `windows-latest` | `package-windows` (on Windows) |
| Windows MSI scaffold | `.zip` (`Product.wxs` + build notes; no MSI) | Archive presence enforced by release build | WiX v4 developer build |

## Installer status matrix

| Platform / format | Status | Scope |
|-------------------|--------|-------|
| Windows installable ZIP | **Partial, CI-smoked** | Release CI downloads, extracts, and executes `sl-viewer.exe --version`; `package-windows.ps1` adds per-user install/uninstall scripts and a Start Menu shortcut locally |
| Windows MSI (WiX v4) | **Partial, scaffold published** | Release CI publishes `Product.wxs` and [`scripts/package-msi.md`](../scripts/package-msi.md) as a source/documentation archive, not an MSI |
| Linux AppImage | **Partial** | `packaging/linux/package-appimage.sh` builds a local candidate with `appimagetool` |
| Linux `.deb` | **Partial** | `packaging/linux/package-deb.sh` builds a local candidate with `dpkg-deb` |
| macOS `.app` | **Partial** | Host-local unsigned app bundle; DMG/notarization deferred |

Release CI publishes and smoke-tests the portable Windows ZIP and Linux
`.tar.gz`. It also publishes the WiX source and build notes as an explicitly
non-installable scaffold archive. `Install.ps1` can copy the local Windows
package below LocalAppData, register an uninstall entry, and create a Start
Menu shortcut. No MSI, AppImage, or `.deb` is a supported release target.
`scripts/installer-lifecycle-smoke.ps1` dry-runs scaffold and lifecycle-doc
assertions on any host with PowerShell; a full clean-host MSI install/uninstall
test still requires Windows plus WiX tooling.
Linux details and limitations are in
[`packaging/linux/README.md`](linux/README.md).

## Installer script draft (not published)

[`scripts/install.sh`](../scripts/install.sh) downloads the latest Linux/macOS
archive from GitHub Releases, verifies it against `SHA256SUMS`, and installs
`sl-viewer` to `~/.local/bin` by default. It is a repository draft, not a
published package-manager channel:

```sh
curl -fsSL https://raw.githubusercontent.com/KooshaPari/SessionLedger/main/scripts/install.sh | sh
```

Pin a release with `SL_VERSION=v0.1.0`; override the destination with
`SL_INSTALL_DIR=/desired/bin`. Review the script before piping it to a shell.

## Notes

- Binaries are built with `cargo build --release`
- macOS bundle includes a minimal `Info.plist`
- **Platform code-signing / Apple notarization: DEFERRED** — see
  [`docs/ops/distribution.md`](../docs/ops/distribution.md#platform-code-signing--notarization--deferred)
  (Gatekeeper / SmartScreen notes for unsigned builds)
- Release CI publishes `SHA256SUMS` and attempts a best-effort GitHub OIDC
  cosign signature (`SHA256SUMS.sigstore.json`); signing failures do not block
  the Release. This existing
  [cosign and attestation path](../docs/ops/distribution.md#release-integrity-signing-cosign)
  also applies when installer candidates are attached for internal evaluation,
  but does not replace platform signing
- Data root for local compose: `SL_DATA_DIR` (default `./.sl-data`); uninstall
  steps documented in the distribution guide
