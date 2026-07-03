# SessionLedger Packaging

Build and package the sl-viewer desktop app for distribution.

## Prerequisites

- Rust toolchain (rustup)
- For macOS: Xcode command line tools
- For Linux: standard build essentials

## Usage

```sh
# macOS .app bundle
make -C packaging package-macos

# Linux binary
make -C packaging package-linux

# Both
make -C packaging package-all
```

## Output

| Platform | Output |
|----------|--------|
| macOS    | `packaging/dist/SessionLedger.app` |
| Linux    | `packaging/dist/linux/SessionLedger` |

## Notes

- Binaries are built with `cargo build --release`
- macOS bundle includes a minimal `Info.plist`
- No codesigning in this scaffold — add `codesign` invocations for distribution
