# SessionLedger Packaging

Build and package the `sl-viewer` desktop app for distribution. Full ops
guide (data dirs, uninstall, signing deferral, Windows matrix):
[`docs/ops/distribution.md`](../docs/ops/distribution.md).

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

| Target | Archive | Local Make? |
|--------|---------|-------------|
| Linux `x86_64-unknown-linux-gnu` | `.tar.gz` | `package-linux` |
| macOS Intel `x86_64-apple-darwin` | `.tar.gz` | `package-macos` |
| macOS ARM `aarch64-apple-darwin` | `.tar.gz` | `package-macos` (on Apple Silicon host) |
| Windows `x86_64-pc-windows-msvc` | `.zip` (`sl-viewer.exe`) | `package-windows` (on Windows) |

## Installer status matrix

| Platform / format | Status | Scope |
|-------------------|--------|-------|
| Windows installable ZIP | **Partial** | `package-windows.ps1` emits portable files plus per-user install/uninstall scripts and a Start Menu shortcut |
| Windows MSI (WiX v4) | **Partial** | `packaging/windows/Product.wxs` and [`scripts/package-msi.md`](../scripts/package-msi.md) are local-build scaffolds |
| Linux AppImage | **Partial** | `packaging/linux/package-appimage.sh` builds a local candidate with `appimagetool` |
| Linux `.deb` | **Partial** | `packaging/linux/package-deb.sh` builds a local candidate with `dpkg-deb` |
| macOS `.app` | **Partial** | Host-local unsigned app bundle; DMG/notarization deferred |

None of these installer formats is published by release CI. The Windows ZIP
remains usable portably, or `Install.ps1` can copy it below LocalAppData,
register an uninstall entry, and create a Start Menu shortcut. The WiX source is
an MSI starting point, not a supported release target. Linux details and
limitations are in [`packaging/linux/README.md`](linux/README.md).

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
