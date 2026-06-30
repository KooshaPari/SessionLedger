# sl-viewer — SessionLedger bundle viewer

A Dioxus 0.6 single-codebase viewer for compiled SessionLedger bundles —
desktop (native) and web (WASM) from one source tree.

## Platform targets

### Desktop (default)

```bash
cargo run -p sl-viewer
```

Opens a native window with the bundle list + detail pane.

### Web (WASM)

The web target requires the `web` feature and the Dioxus CLI (`dx`):

```bash
# Install the Dioxus CLI (one time)
cargo install dioxus-cli

# Serve on the web
dx serve --platform web -p sl-viewer
```

This compiles `sl-viewer` to WASM and serves it on `http://localhost:8080`.

## Cargo features

| Feature    | Default | Enables                     |
| ---------- | ------- | --------------------------- |
| `desktop`  | yes     | `dioxus/desktop` — native   |
| `web`      | no      | `dioxus/web` — WASM browser |

The entry point in `src/main.rs` uses `#[cfg]` gates to select the correct
renderer at compile time.
