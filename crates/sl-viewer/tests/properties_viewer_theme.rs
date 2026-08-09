//! Property evidence for sl-viewer's `theme::Theme` / `ThemeColors`
//! reducers.
//!
//! The theme module is the SSOT for the design-token palette bridge:
//! every Lab-Coat hex flows through `ThemeColors::dark` / `light` /
//! `for_theme`. If a hex is swapped, a label is renamed, or the
//! `System` fallback drift-discovers, the entire viewer colour
//! contract breaks silently. Every visible property is pinned here.
//!
//! `theme::Theme` invariants:
//!  * `Default::default()` is `Theme::System` (the documented fallback).
//!  * JSON round-trip preserves the variant.
//!  * Serialised kebab-case form is the lowercase variant name
//!    (`"light"` / `"dark"` / `"system"`).
//!
//! `theme::ThemeColors::dark` invariants (6 properties):
//!  * `bg` / `text` / `accent` / `focus` / `danger` / `muted` /
//!    `secondary` / `border` / `surface` are all non-empty and
//!    match the documented `lab_coat::*` constants.
//!  * `focus == accent` (the focus ring is the brand cobalt across
//!    chrome that uses the dark palette).
//!
//! `theme::ThemeColors::light` invariants (6 properties):
//!  * Same shape: every field is non-empty and matches the documented
//!    `lab_coat::*` constant.
//!  * `focus == accent` (light-theme mirror of the dark invariant).
//!
//! `theme::ThemeColors::for_theme` invariants (3 properties):
//!  * `for_theme(Dark) == dark()`.
//!  * `for_theme(Light) == light()`.
//!  * `for_theme(System) == dark()` (desktop fallback documented in
//!    the module).

use proptest::prelude::*;
use sl_viewer::theme::{Theme, ThemeColors};
use sl_viewer::tokens::lab_coat;

// ── Theme ───────────────────────────────────────────────────────────────────

proptest! {
    /// `Theme::default()` is `Theme::System` (the documented fallback).
    #[test]
    fn theme_default_is_system(_seed in any::<u32>()) {
        prop_assert_eq!(Theme::default(), Theme::System);
    }

    /// JSON round-trip preserves the `Theme` variant for every variant.
    #[test]
    fn theme_json_round_trips(variant in prop::sample::select(vec![
        Theme::Light, Theme::Dark, Theme::System,
    ])) {
        let json = serde_json::to_string(&variant).expect("serialize");
        let back: Theme = serde_json::from_str(&json).expect("deserialize");
        prop_assert_eq!(back, variant);
    }

    /// The serialised form is the lowercase variant name.
    #[test]
    fn theme_json_uses_lowercase(variant in prop::sample::select(vec![
        Theme::Light, Theme::Dark, Theme::System,
    ])) {
        let json = serde_json::to_string(&variant).expect("serialize");
        let expected = format!("\"{variant:?}\"").to_lowercase();
        prop_assert!(json.contains(&expected), "expected {expected:?} in {json}");
    }
}

// ── ThemeColors::dark ───────────────────────────────────────────────────────

proptest! {
    /// `ThemeColors::dark().bg` is the documented `lab_coat::BG_DARK`.
    #[test]
    fn dark_bg_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().bg, lab_coat::BG_DARK);
    }

    /// `ThemeColors::dark().text` is the documented `lab_coat::TEXT_DARK`.
    #[test]
    fn dark_text_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().text, lab_coat::TEXT_DARK);
    }

    /// `ThemeColors::dark().accent` is the documented `lab_coat::COBALT_ON_DARK`.
    #[test]
    fn dark_accent_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().accent, lab_coat::COBALT_ON_DARK);
    }

    /// `ThemeColors::dark().focus` is the documented `lab_coat::COBALT_ON_DARK`.
    #[test]
    fn dark_focus_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().focus, lab_coat::COBALT_ON_DARK);
    }

    /// `ThemeColors::dark().danger` is the documented `lab_coat::DANGER_DARK`.
    #[test]
    fn dark_danger_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().danger, lab_coat::DANGER_DARK);
    }

    /// `ThemeColors::dark().secondary` is the documented `lab_coat::TEAL_ON_DARK`.
    #[test]
    fn dark_secondary_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().secondary, lab_coat::TEAL_ON_DARK);
    }

    /// `ThemeColors::dark().border` is the documented `lab_coat::BORDER_DARK`.
    #[test]
    fn dark_border_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().border, lab_coat::BORDER_DARK);
    }

    /// `ThemeColors::dark().surface` is the documented `lab_coat::SLATE`.
    #[test]
    fn dark_surface_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().surface, lab_coat::SLATE);
    }

    /// `ThemeColors::dark().muted` is the documented `lab_coat::TEXT_MUTED_DARK`.
    #[test]
    fn dark_muted_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::dark().muted, lab_coat::TEXT_MUTED_DARK);
    }

    /// `focus == accent` so the dark palette uses a single brand color
    /// for both accent and focus rings.
    #[test]
    fn dark_focus_equals_accent(_seed in any::<u32>()) {
        let d = ThemeColors::dark();
        prop_assert_eq!(d.focus, d.accent);
    }

    /// Every dark field is non-empty (no accidental empty-string hex).
    #[test]
    fn dark_fields_nonempty(_seed in any::<u32>()) {
        let d = ThemeColors::dark();
        prop_assert!(!d.bg.is_empty());
        prop_assert!(!d.surface.is_empty());
        prop_assert!(!d.text.is_empty());
        prop_assert!(!d.accent.is_empty());
        prop_assert!(!d.secondary.is_empty());
        prop_assert!(!d.border.is_empty());
        prop_assert!(!d.focus.is_empty());
        prop_assert!(!d.danger.is_empty());
        prop_assert!(!d.muted.is_empty());
    }
}

// ── ThemeColors::light ──────────────────────────────────────────────────────

proptest! {
    /// `ThemeColors::light().bg` is the documented `lab_coat::LAB_WHITE`.
    #[test]
    fn light_bg_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().bg, lab_coat::LAB_WHITE);
    }

    /// `ThemeColors::light().text` is the documented `lab_coat::SLATE`.
    #[test]
    fn light_text_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().text, lab_coat::SLATE);
    }

    /// `ThemeColors::light().accent` is the documented `lab_coat::COBALT`.
    #[test]
    fn light_accent_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().accent, lab_coat::COBALT);
    }

    /// `ThemeColors::light().focus` is the documented `lab_coat::COBALT`.
    #[test]
    fn light_focus_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().focus, lab_coat::COBALT);
    }

    /// `ThemeColors::light().danger` is the documented `lab_coat::DANGER_LIGHT`.
    #[test]
    fn light_danger_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().danger, lab_coat::DANGER_LIGHT);
    }

    /// `ThemeColors::light().secondary` is the documented `lab_coat::TEAL`.
    #[test]
    fn light_secondary_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().secondary, lab_coat::TEAL);
    }

    /// `ThemeColors::light().border` is the documented `lab_coat::BORDER_LIGHT`.
    #[test]
    fn light_border_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().border, lab_coat::BORDER_LIGHT);
    }

    /// `ThemeColors::light().surface` is the documented `lab_coat::SURFACE_LIGHT`.
    #[test]
    fn light_surface_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().surface, lab_coat::SURFACE_LIGHT);
    }

    /// `ThemeColors::light().muted` is the documented `lab_coat::TEXT_MUTED_LIGHT`.
    #[test]
    fn light_muted_matches_lab_coat(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::light().muted, lab_coat::TEXT_MUTED_LIGHT);
    }

    /// `focus == accent` for the light palette too.
    #[test]
    fn light_focus_equals_accent(_seed in any::<u32>()) {
        let l = ThemeColors::light();
        prop_assert_eq!(l.focus, l.accent);
    }

    /// Every light field is non-empty.
    #[test]
    fn light_fields_nonempty(_seed in any::<u32>()) {
        let l = ThemeColors::light();
        prop_assert!(!l.bg.is_empty());
        prop_assert!(!l.surface.is_empty());
        prop_assert!(!l.text.is_empty());
        prop_assert!(!l.accent.is_empty());
        prop_assert!(!l.secondary.is_empty());
        prop_assert!(!l.border.is_empty());
        prop_assert!(!l.focus.is_empty());
        prop_assert!(!l.danger.is_empty());
        prop_assert!(!l.muted.is_empty());
    }
}

// ── ThemeColors::for_theme ──────────────────────────────────────────────────

proptest! {
    /// `for_theme(Dark) == dark()`.
    #[test]
    fn for_theme_dark_matches_dark(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::for_theme(Theme::Dark), ThemeColors::dark());
    }

    /// `for_theme(Light) == light()`.
    #[test]
    fn for_theme_light_matches_light(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::for_theme(Theme::Light), ThemeColors::light());
    }

    /// `for_theme(System) == dark()` (desktop fallback documented in
    /// the module).
    #[test]
    fn for_theme_system_falls_back_to_dark(_seed in any::<u32>()) {
        prop_assert_eq!(ThemeColors::for_theme(Theme::System), ThemeColors::dark());
    }
}
