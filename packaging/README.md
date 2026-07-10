# SessionLedger Packaging

Build and package the `sl-viewer` desktop app for distribution. Full ops
guide (data dirs, uninstall, signing deferral, Windows matrix):
[`docs/ops/distribution.md`](../docs/ops/distribution.md).

## Prerequisites

- Rust toolchain (rustup)
- For macOS: Xcode command line tools
- For Linux: standard build essentials
- For Windows Release artifacts: produced by CI only (see matrix below) — no
  local `package-windows` Make target yet

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

## Notes

- Binaries are built with `cargo build --release`
- macOS bundle includes a minimal `Info.plist`
- **Code-signing / notarization: DEFERRED** — see
  [`docs/ops/distribution.md`](../docs/ops/distribution.md#code-signing--notarization--deferred)
  (Gatekeeper / SmartScreen notes for unsigned builds)
- Data root for local compose: `SL_DATA_DIR` (default `./.sl-data`); uninstall
  steps documented in the distribution guide
- Cosign / Sigstore attestation of Release assets remains a soft goal (#66)
