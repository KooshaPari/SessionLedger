# SessionLedger Packaging

Build and package the `sl-viewer` desktop app for distribution. Full ops
guide (data dirs, uninstall, signing deferral, Windows matrix):
[`docs/ops/distribution.md`](../docs/ops/distribution.md).

## Prerequisites

- Rust toolchain (rustup)
- For macOS: Xcode command line tools
- For Linux: standard build essentials
- For Windows: PowerShell 5.1+ and the MSVC Rust target/toolchain

## Usage

```sh
# macOS .app bundle
make -C packaging package-macos

# Linux binary
make -C packaging package-linux

# Windows portable ZIP (run on Windows)
make -C packaging package-windows
# Equivalent:
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/package-windows.ps1

# macOS + Linux (host-local scaffold)
make -C packaging package-all
```

## Output (local Make)

| Platform | Output |
|----------|--------|
| macOS    | `packaging/dist/SessionLedger.app` |
| Linux    | `packaging/dist/linux/SessionLedger` |
| Windows  | `packaging/dist/sl-viewer-v<version>-x86_64-pc-windows-msvc.zip` |

## Release matrix (GitHub Actions)

Tag push (`v*`) builds via [`.github/workflows/release.yml`](../.github/workflows/release.yml):

| Target | Archive | Local Make? |
|--------|---------|-------------|
| Linux `x86_64-unknown-linux-gnu` | `.tar.gz` | `package-linux` |
| macOS Intel `x86_64-apple-darwin` | `.tar.gz` | `package-macos` |
| macOS ARM `aarch64-apple-darwin` | `.tar.gz` | `package-macos` (on Apple Silicon host) |
| Windows `x86_64-pc-windows-msvc` | `.zip` (`sl-viewer.exe`) | `package-windows` (on Windows) |

### Windows installer status

The Windows download and local package are portable `.zip` files containing
`sl-viewer.exe`, licenses, and a launch note. They are not installers and the
executable is not Authenticode-signed.

- **MSI:** not built or published
- **NSIS `.exe` installer:** not built or published
- **Local packaging target:** `package-windows` creates the same named layout
  used by release CI

Users extract the Release zip and run `sl-viewer.exe` directly. Uninstall by
stopping the process and deleting the extracted directory; see the distribution
guide for data-directory cleanup. MSI or NSIS support remains future installer
work and should not be inferred from the Windows release target.

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
  the Release
- Data root for local compose: `SL_DATA_DIR` (default `./.sl-data`); uninstall
  steps documented in the distribution guide
