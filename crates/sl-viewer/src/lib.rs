//! Shared UI components for the `sl-viewer` Dioxus app.
//!
//! Compiled for both desktop (`dioxus-desktop`) and web (`dioxus-web`) via
//! Cargo feature flags.  The platform-specific entry points in `main.rs` call
//! [`App`] regardless of renderer.

pub mod app;
pub mod bundle_list;
pub mod detail_pane;
pub mod mock_data;

pub use app::App;
