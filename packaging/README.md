# SessionLedger Packaging

Build and package the `sl-viewer` desktop app for distribution. Full ops
guide (data dirs, uninstall, signing deferral, Windows matrix):
[`docs/ops/distribution.md`](../docs/ops/distribution.md).

## Prerequisites

- Rust toolchain (rustup)
- For macOS: Xcode command line tools
- For Linux: standard build essentials
- For Windows Release artifacts: produced by CI only (see matrix below)

## Usage

```sh
# macOS .app bundle
make -C packaging package-macos

# Linux binary
make -C packaging package-linux

# Both (local scaffold)
make -C packaging package-all
```

## Output (local Make)

| Platform | Output |
|----------|--------|
| macOS    | `packaging/dist/SessionLedger.app` |
| Linux    | `packaging/dist/linux/SessionLedger` |

## Release matrix (GitHub Actions)

Tag push (`v*`) builds via [`.github/workflows/release.yml`](../.github/workflows/release.yml):

| Target | Archive | Local Make? |
|--------|---------|-------------|
| Linux `x86_64-unknown-linux-gnu` | `.tar.gz` | `package-linux` |
| macOS Intel `x86_64-apple-darwin` | `.tar.gz` | `package-macos` |
| macOS ARM `aarch64-apple-darwin` | `.tar.gz` | `package-macos` (on Apple Silicon host) |
| Windows `x86_64-pc-windows-msvc` | `.zip` (`sl-viewer.exe`) | **CI only** — shipped on tag; not in Makefile |

### Windows installer status

The Windows download shipped today is a portable `.zip` containing
`sl-viewer.exe`. It is not an installer and the executable is not
Authenticode-signed.

- **MSI:** not built or published
- **NSIS `.exe` installer:** not built or published
- **Local packaging target:** no `package-windows` Make target

Users extract the Release zip and run `sl-viewer.exe` directly. Uninstall by
stopping the process and deleting the extracted directory; see the distribution
guide for data-directory cleanup. MSI or NSIS support remains future installer
work and should not be inferred from the Windows release target.

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
