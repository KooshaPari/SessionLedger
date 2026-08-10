//! Property evidence for sl-viewer's `menu` module IDs and structure
//! (desktop only).
//!
//! The desktop menu is wired up by [`App`] and dispatched via
//! `MenuId::as_str()`. Each menu item needs a stable, well-formed id
//! so the JS bridge in `app.rs` can match on it without falling back
//! to a stringly-typed default. Every id contract is pinned here.
//!
//! `menu` invariants:
//!  * Every id is non-empty.
//!  * Every id is kebab-case ASCII so it round-trips through muda's
//!    `MenuId::new` and the JS event bridge without escaping.
//!  * Every id carries the documented `sl-viewer.` prefix.
//!  * Every id is unique across the documented set so a single
//!    muda event resolves to one DOM action.
//!  * The number of documented ids matches the menu taxonomy (9:
//!    2 App, 2 File, 1 Edit, 3 View, 1 Help).
//!
//! Test is compiled only on `desktop` (mirrors the source module's
//! `#![cfg(feature = "desktop")]` gate).

#![cfg(feature = "desktop")]

use proptest::prelude::*;
use sl_viewer::menu::{
    ID_APP_ABOUT, ID_APP_SETTINGS, ID_EDIT_FIND, ID_FILE_RELOAD_DISCOVERY,
    ID_FILE_SETTINGS, ID_HELP_TOGGLE, ID_VIEW_COMMAND_PALETTE, ID_VIEW_RELOAD,
    ID_VIEW_TOGGLE_THEME,
};

const MENU_IDS: &[&str] = &[
    ID_APP_ABOUT,
    ID_APP_SETTINGS,
    ID_FILE_RELOAD_DISCOVERY,
    ID_FILE_SETTINGS,
    ID_EDIT_FIND,
    ID_VIEW_RELOAD,
    ID_VIEW_TOGGLE_THEME,
    ID_VIEW_COMMAND_PALETTE,
    ID_HELP_TOGGLE,
];

proptest! {
    /// Every menu id is non-empty.
    #[test]
    fn menu_ids_nonempty(_seed in any::<u32>()) {
        for id in MENU_IDS {
            prop_assert!(!id.is_empty(), "menu id {id:?} is empty");
        }
    }

    /// Every menu id is kebab-case ASCII so muda / JS / serde
    /// round-trips are safe.
    #[test]
    fn menu_ids_kebab_case_ascii(_seed in any::<u32>()) {
        for id in MENU_IDS {
            let valid = id.chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '.');
            prop_assert!(valid, "menu id {id:?} is not kebab-case ASCII");
        }
    }

    /// Every menu id carries the documented `sl-viewer.` prefix so
    /// the JS bridge can dispatch without conflicting with other
    /// muda-installed ids.
    #[test]
    fn menu_ids_prefixed(_seed in any::<u32>()) {
        for id in MENU_IDS {
            prop_assert!(id.starts_with("sl-viewer."), "menu id {id:?} missing sl-viewer. prefix");
        }
    }

    /// Every menu id is unique across the documented set so a muda
    /// event resolves to one DOM action.
    #[test]
    fn menu_ids_unique(_seed in any::<u32>()) {
        let mut sorted = MENU_IDS.to_vec();
        sorted.sort();
        sorted.dedup();
        prop_assert_eq!(sorted.len(), MENU_IDS.len());
    }

    /// The menu taxonomy is stable: 9 documented ids (2 App, 2 File,
    /// 1 Edit, 3 View, 1 Help). If this drifts the operator
    /// documentation needs to be re-aligned.
    #[test]
    fn menu_ids_count_stable(_seed in any::<u32>()) {
        prop_assert_eq!(MENU_IDS.len(), 9);
    }
}
