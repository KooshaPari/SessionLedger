# macOS packaging (unsigned)

Scripts here produce an unsigned `.app` and a `productbuild` `.pkg`.
Apple Developer ID signing and notarization remain deferred
([ADR 0003](../../docs/adr/0003-platform-code-signing.md)).

## Prerequisites

- Xcode command line tools (`productbuild`, `pkgutil`)
- A release `sl-viewer` binary (`cargo build -p sl-viewer --release`)

## Usage

```sh
# From repo root, after building for the host (or a cross target):
export VERSION=0.1.0
export BINARY=target/aarch64-apple-darwin/release/sl-viewer
export ARCH_LABEL=aarch64
./packaging/macos/package-app.sh
./packaging/macos/package-pkg.sh
```

Outputs land under `packaging/dist/`:

- `SessionLedger.app`
- `SessionLedger-<version>-<arch>.pkg` when `ARCH_LABEL` is set

## Local install

After building an app bundle, install it into the current Mac's Applications
folder without elevating privileges:

```sh
./packaging/macos/install-local.sh
```

The script validates the bundle executable, preserves an existing install as
`SessionLedger.app.previous`, and never creates a background service with an
implicit watch root. To install a locally-built daemon as well, opt in:

```sh
INSTALL_DAEMON=1 ./packaging/macos/install-local.sh
```

Start the daemon explicitly with the session root you intend to ingest; see
the command printed by the installer. This keeps local session data and HTTP
exposure operator-controlled.

Release CI builds at least the `aarch64-apple-darwin` PKG (and `x86_64` when
the matrix target runs) and attaches them as Release assets.
