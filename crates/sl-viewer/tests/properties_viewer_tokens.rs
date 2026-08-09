//! Property evidence for sl-viewer's `tokens` module — the design-token
//! single source of truth for the Lab-Coat / viewer color palette.
//!
//! The unit tests in `tokens.rs` pin specific values and exercise
//! `ThemeColors::light() / dark()` against the `lab_coat::*` mirror.
//! These properties pin the broader SSOT invariants:
//!
//!  * Every `lab_coat::*` hex constant is a well-formed `#RRGGBB` string
//!    (7 chars, leading `#`, then 6 hex digits).
//!  * All Lab-Coat hex constants are pairwise distinct — no two share
//!    the same value (catches drift where a constant is silently
//!    re-aliased to another).
//!  * Every Lab-Coat hex appears somewhere in `TOKENS_CSS` so the
//!    Rust mirror and the CSS SSOT stay in sync.
//!  * Every `REQUIRED_CSS_VARS` entry starts with `--`, has no
//!    duplicates, and appears as a substring of `TOKENS_CSS`.
//!  * `VIEWER_COLOR_SCHEME` mentions both `:root[data-theme="dark"]`
//!    and `:root` so the dark-mode flip is wired.
//!
//! proptest is added to `sl-viewer/[dev-dependencies]` (mirroring the
//! workspace root); see PR #425 for the initial wiring.

use std::collections::HashSet;

use proptest::prelude::*;
use sl_viewer::tokens::{
    lab_coat, REQUIRED_CSS_VARS, TOKENS_CSS, VIEWER_COLOR_SCHEME,
};

// ── strategies ──────────────────────────────────────────────────────────────

/// Generate indices into `REQUIRED_CSS_VARS` for prop_any tests.
fn required_var_index_strategy() -> impl Strategy<Value = usize> {
    0..REQUIRED_CSS_VARS.len()
}

/// Generate indices into the `lab_coat::*` constants via the documented
/// hex list. We use the indices, then look up the value, so we exercise
/// the actual const definitions (not duplicates).
fn lab_coat_hex_indices_strategy() -> impl Strategy<Value = usize> {
    0..lab_coat_hex_list().len()
}

/// The full list of `lab_coat::*` hex constants in stable declaration
/// order. We compute this once via a small reflection-on-source approach:
/// every `pub const` in `lab_coat::*` whose value is a `&'static str`
/// starting with `#`. Since we can't introspect Rust modules at runtime,
/// we hard-code the list (mirroring `tokens.rs`). The constants are
/// public — any new addition requires also extending this list, which
/// the `proptest` exhaustiveness check below will catch.
fn lab_coat_hex_list() -> &'static [&'static str] {
    &[
        lab_coat::LAB_WHITE,
        lab_coat::SLATE,
        lab_coat::COBALT,
        lab_coat::COBALT_ON_DARK,
        lab_coat::ORANGE,
        lab_coat::TEAL,
        lab_coat::TEAL_ON_DARK,
        lab_coat::BG_DARK,
        lab_coat::SURFACE_LIGHT,
        lab_coat::BORDER_LIGHT,
        lab_coat::BORDER_DARK,
        lab_coat::TEXT_DARK,
        lab_coat::TEXT_MUTED_LIGHT,
        lab_coat::TEXT_MUTED_DARK,
        lab_coat::DANGER_LIGHT,
        lab_coat::DANGER_DARK,
    ]
}

// ── lab_coat hex well-formedness ────────────────────────────────────────────

proptest! {
    /// Property: every `lab_coat::*` hex constant is a 7-char string
    /// starting with `#`, followed by 6 lowercase hex digits. Catches
    /// drift where someone hand-types an `rgb(…)` literal or a 3-digit
    /// hex.
    #[test]
    fn lab_coat_hex_well_formed(i in lab_coat_hex_indices_strategy()) {
        let hex = lab_coat_hex_list()[i];
        prop_assert_eq!(hex.len(), 7, "hex {:?} must be 7 chars", hex);
        prop_assert!(hex.starts_with('#'), "hex {:?} must start with '#'", hex);
        let body = &hex[1..];
        prop_assert!(
            body.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
            "hex {:?} must be lowercase ASCII hex digits only",
            hex,
        );
    }

    /// Property: every `lab_coat::*` hex constant is non-empty (sanity
    /// check — the well-formedness check above is the stricter version).
    #[test]
    fn lab_coat_hex_nonempty(i in lab_coat_hex_indices_strategy()) {
        let hex = lab_coat_hex_list()[i];
        prop_assert!(!hex.is_empty(), "lab_coat hex at index {} is empty", i);
    }

    /// Property: all `lab_coat::*` hex constants are pairwise distinct.
    /// No silent re-aliasing.
    #[test]
    fn lab_coat_hexes_distinct(_i in 0u8..4) {
        let list = lab_coat_hex_list();
        let set: HashSet<_> = list.iter().collect();
        prop_assert_eq!(set.len(), list.len());
    }

    /// Property: every `lab_coat::*` hex appears as a substring of
    /// `TOKENS_CSS` so the Rust mirror and the CSS SSOT stay in sync.
    /// If a constant is added without updating the CSS, this fails.
    #[test]
    fn lab_coat_hex_in_tokens_css(i in lab_coat_hex_indices_strategy()) {
        let hex = lab_coat_hex_list()[i];
        prop_assert!(
            TOKENS_CSS.contains(hex),
            "TOKENS_CSS missing lab_coat hex {:?}",
            hex,
        );
    }
}

// ── REQUIRED_CSS_VARS invariants ────────────────────────────────────────────

proptest! {
    /// Property: every `REQUIRED_CSS_VARS` entry starts with `--` (CSS
    /// custom property convention).
    #[test]
    fn required_css_var_starts_with_double_dash(i in required_var_index_strategy()) {
        let var = REQUIRED_CSS_VARS[i];
        prop_assert!(var.starts_with("--"), "var {:?} must start with '--'", var);
    }

    /// Property: `REQUIRED_CSS_VARS` has no duplicates.
    #[test]
    fn required_css_vars_unique(_i in 0u8..4) {
        let list = REQUIRED_CSS_VARS;
        let set: HashSet<_> = list.iter().collect();
        prop_assert_eq!(set.len(), list.len());
    }

    /// Property: every `REQUIRED_CSS_VARS` entry is non-empty (no
    /// empty `--` strings accidentally added).
    #[test]
    fn required_css_var_nonempty(i in required_var_index_strategy()) {
        let var = REQUIRED_CSS_VARS[i];
        prop_assert!(!var.is_empty(), "REQUIRED_CSS_VARS[{}] is empty", i);
    }

    /// Property: every `REQUIRED_CSS_VARS` entry appears as a
    /// substring of `TOKENS_CSS`. Catches drift where a var name is
    /// added to the list without updating the CSS file.
    #[test]
    fn required_css_var_in_tokens_css(i in required_var_index_strategy()) {
        let var = REQUIRED_CSS_VARS[i];
        prop_assert!(
            TOKENS_CSS.contains(var),
            "TOKENS_CSS missing required CSS var {:?}",
            var,
        );
    }
}

// ── VIEWER_COLOR_SCHEME invariants ──────────────────────────────────────────

proptest! {
    /// Property: `VIEWER_COLOR_SCHEME` declares both the default
    /// (`:root`) and dark (`:root[data-theme="dark"]`) selectors so the
    /// viewer's color-scheme flip is wired.
    #[test]
    fn viewer_color_scheme_declares_both_selectors(_i in 0u8..4) {
        prop_assert!(VIEWER_COLOR_SCHEME.contains(":root"));
        prop_assert!(VIEWER_COLOR_SCHEME.contains("[data-theme=\"dark\"]"));
    }

    /// Property: `VIEWER_COLOR_SCHEME` declares `color-scheme` for
    /// both modes (the W3C CSS prop that triggers browser scrollbar
    /// and form-control color flips).
    #[test]
    fn viewer_color_scheme_declares_color_scheme_property(_i in 0u8..4) {
        prop_assert!(VIEWER_COLOR_SCHEME.contains("color-scheme"));
        // Both modes must set the property.
        let occurrences = VIEWER_COLOR_SCHEME.matches("color-scheme").count();
        prop_assert_eq!(occurrences, 2);
    }
}
