//! `sl-viewer` platform entry points.
//!
//! Desktop  : `cargo run -p sl-viewer`             (default feature)
//! Web WASM : `dx serve --platform web -p sl-viewer`  (requires `web` feature)

use sl_viewer::App;

#[cfg(feature = "desktop")]
fn main() {
    dioxus::LaunchBuilder::desktop().launch(App);
}

#[cfg(feature = "web")]
fn main() {
    dioxus::LaunchBuilder::web().launch(App);
}
