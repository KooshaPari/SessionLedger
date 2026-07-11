#!/bin/sh
set -eu

REPO="${SL_REPO:-KooshaPari/SessionLedger}"
INSTALL_DIR="${SL_INSTALL_DIR:-$HOME/.local/bin}"

case "$(uname -s)" in
    Linux) os_target="x86_64-unknown-linux-gnu" ;;
    Darwin)
        case "$(uname -m)" in
            arm64|aarch64) os_target="aarch64-apple-darwin" ;;
            x86_64) os_target="x86_64-apple-darwin" ;;
            *) echo "Unsupported macOS architecture: $(uname -m)" >&2; exit 1 ;;
        esac
        ;;
    *) echo "This installer supports Linux and macOS; use the Windows ZIP on Windows." >&2; exit 1 ;;
esac

if [ "$(uname -s)" = "Linux" ] && [ "$(uname -m)" != "x86_64" ]; then
    echo "Unsupported Linux architecture: $(uname -m)" >&2
    exit 1
fi

version="${SL_VERSION:-}"
if [ -z "$version" ]; then
    latest_url="$(curl -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest")"
    version="${latest_url##*/}"
fi
case "$version" in
    v*) ;;
    *) version="v$version" ;;
esac

archive="sl-viewer-$version-$os_target.tar.gz"
base_url="https://github.com/$REPO/releases/download/$version"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

curl -fsSL "$base_url/$archive" -o "$tmp_dir/$archive"
curl -fsSL "$base_url/SHA256SUMS" -o "$tmp_dir/SHA256SUMS"

expected="$(awk -v file="$archive" '$2 == file { print $1; exit }' "$tmp_dir/SHA256SUMS")"
if [ -z "$expected" ]; then
    echo "No checksum found for $archive." >&2
    exit 1
fi
if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$tmp_dir/$archive" | awk '{print $1}')"
else
    actual="$(shasum -a 256 "$tmp_dir/$archive" | awk '{print $1}')"
fi
if [ "$actual" != "$expected" ]; then
    echo "Checksum mismatch for $archive." >&2
    exit 1
fi

tar -xzf "$tmp_dir/$archive" -C "$tmp_dir"
mkdir -p "$INSTALL_DIR"
install -m 0755 "$tmp_dir/sl-viewer-$version-$os_target/sl-viewer" "$INSTALL_DIR/sl-viewer"

echo "Installed sl-viewer $version to $INSTALL_DIR/sl-viewer"
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "Add $INSTALL_DIR to PATH to run sl-viewer." ;;
esac
