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
pub mod async_states;
pub mod bundle_diff;
pub mod bundle_list;
pub mod corpus_loader;
pub mod detail_pane;
pub mod history_tab;
pub mod live_feed;
pub mod memory_tab;
pub mod mock_data;
pub mod replay_view;
pub mod search_view;
pub mod session_list;
pub mod theme;
pub mod timeline;

pub use app::App;
pub use async_states::{ErrorState, LoadingState};
pub use corpus_loader::{load_sessions, DataSource};
pub use session_list::SessionList;
