# SessionLedger Install Channels

This page tracks the supported, partial, and future ways to install
SessionLedger artifacts. It complements the local packaging targets in
[`packaging/README.md`](README.md) and the operational distribution guide in
[`docs/ops/distribution.md`](../docs/ops/distribution.md).

## Channel Status

| Channel | Status | Artifact / command |
|---------|--------|--------------------|
| Cargo from source | **Active for developers** | `cargo install --path crates/sl-daemon --locked`; build `sl-viewer` with local packaging targets |
| GitHub Releases archives | **Active** | Tagged `v*` releases publish `sl-viewer-<tag>-<target>.tar.gz` / `.zip`, checksums, SBOM, and best-effort provenance |
| Repository install script | **Draft, not published** | `scripts/install.sh` installs checksum-verified Linux/macOS `sl-viewer` archives from GitHub Releases |
| Windows installable ZIP | **Partial, CI-smoked** | Release ZIP is smoke-tested; local package adds PowerShell install/uninstall scripts |
| Linux AppImage / `.deb` | **Partial local scaffolds** | Developer-only scripts under `packaging/linux/`; not release channels |
| Scoop bucket | **Future placeholder** | No manifest, bucket, or update automation exists yet |
| Homebrew tap | **Future placeholder** | No formula, tap, bottle, or notarized macOS channel exists yet |
| crates.io | **Future placeholder** | No crates are published to crates.io yet |

## Cargo Install Path

The daemon/CLI crate can be installed from a checkout for local operations:

```bash
cargo install --path crates/sl-daemon --locked
```

This installs the `sl-daemon` binary into Cargo's configured bin directory
(`~/.cargo/bin` by default). Start the long-running daemon with explicit input
and output paths:

```bash
sl-daemon serve \
  --watch "$HOME/.forge/sessions" \
  --out "$HOME/.local/share/sessionledger/out" \
  --http-bind 127.0.0.1:8080
```

`cargo install --path` is a developer/source channel. It does not provide
automatic updates, package-manager metadata, desktop integration, or platform
signing.

## GitHub Releases Archives

Tagged releases (`v*`) publish portable `sl-viewer` archives:

```text
sl-viewer-<tag>-x86_64-unknown-linux-gnu.tar.gz
sl-viewer-<tag>-x86_64-apple-darwin.tar.gz
sl-viewer-<tag>-aarch64-apple-darwin.tar.gz
sl-viewer-<tag>-x86_64-pc-windows-msvc.zip
```

Each Release also publishes `SHA256SUMS`, `session-ledger.cdx.json`, and a
best-effort `SHA256SUMS.sigstore.json`. Verify archives with the checksum and
provenance flow in [`docs/ops/distribution.md`](../docs/ops/distribution.md).

GitHub Releases are the current user-facing archive channel for `sl-viewer`.
The daemon remains installable from source or container/local process-compose
until a dedicated daemon release artifact is added.

## Draft And Future Channels

[`scripts/install.sh`](../scripts/install.sh) is a documented draft for
Linux/macOS archive installation:

```bash
curl -fsSL https://raw.githubusercontent.com/KooshaPari/SessionLedger/main/scripts/install.sh | sh
```

Set `SL_VERSION=v0.1.0` to pin a release and `SL_INSTALL_DIR=/desired/bin` to
change the destination. Review the script before piping it to a shell.

Scoop and Homebrew are explicit placeholders only. Do not describe them as
available until this repository contains the corresponding manifest/formula,
release automation, and validation path.
