//! `sl-viewer` platform entry points.
//!
//! Desktop  : `cargo run -p sl-viewer`             (default feature)
//! Web WASM : `dx serve --platform web -p sl-viewer`  (requires `web` feature)

use sl_viewer::App;

/// Human-readable window title — surfaced via the OS window chrome on
/// desktop and the browser tab label on web (via the `[web.title]` field
/// in `Dioxus.toml`).
const VIEWER_TITLE: &str = "Session Ledger Viewer";

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::{Config, WindowBuilder};

    if let Some(argument) = std::env::args().nth(1) {
        match argument.as_str() {
            "--version" | "-V" => {
                println!("sl-viewer {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--help" | "-h" => {
                println!("SessionLedger desktop viewer\n\nUsage: sl-viewer [--help] [--version]");
                return;
            }
            _ => {}
        }
    }

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_title(VIEWER_TITLE)))
        .launch(App);
}

#[cfg(feature = "web")]
fn main() {
    // The web launcher reads the title from `Dioxus.toml` (see
    // `app_title()` in `dioxus_cli_config`); if we ever need to set it
    // programmatically we can switch to a wrapper component that emits a
    // `<Title>` element from `dioxus::document`.
    let _ = VIEWER_TITLE;
    dioxus::LaunchBuilder::web().with_cfg(dioxus::web::Config::default()).launch(App);
}
