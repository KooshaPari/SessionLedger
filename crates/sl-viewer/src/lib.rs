//! Shared UI components for the `sl-viewer` Dioxus app.
//!
//! Three tabs:
//! - **Bundles** — browse compiled continuation bundles
//! - **History** — session history timeline
//! - **Memory** — wiki/docs view of distilled memories
//!
//! Compiled for both desktop (`dioxus-desktop`) and web (`dioxus-web`) via
//! Cargo feature flags.  The platform-specific entry points in `main.rs` call
//! [`App`] regardless of renderer.

pub mod app;
pub mod bundle_list;
pub mod detail_pane;
pub mod history_tab;
pub mod memory_tab;
pub mod mock_data;

pub use app::App;
