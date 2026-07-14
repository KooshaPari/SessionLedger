use serde::{Deserialize, Serialize};

use crate::tokens::lab_coat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub bg: &'static str,
    pub surface: &'static str,
    pub text: &'static str,
    pub accent: &'static str,
    pub secondary: &'static str,
    pub border: &'static str,
    /// Focus-ring / keyboard-highlight color (Lab-Coat cobalt family).
    pub focus: &'static str,
    /// Error / destructive text.
    pub danger: &'static str,
    /// Muted secondary text.
    pub muted: &'static str,
}
impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            bg: lab_coat::BG_DARK,
            surface: lab_coat::SLATE,
            text: lab_coat::TEXT_DARK,
            // AA (≥4.5:1) on slate + 16% accent color-mix chrome — brand cobalt is 2.84:1.
            accent: lab_coat::COBALT_ON_DARK,
            // Lighter teal for badge/chrome text on 18% teal mixes (~#1d434b).
            secondary: lab_coat::TEAL_ON_DARK,
            border: lab_coat::BORDER_DARK,
            focus: lab_coat::COBALT_ON_DARK,
            danger: lab_coat::DANGER_DARK,
            muted: lab_coat::TEXT_MUTED_DARK,
        }
    }
    pub fn light() -> Self {
        Self {
            bg: lab_coat::LAB_WHITE,
            surface: lab_coat::SURFACE_LIGHT,
            text: lab_coat::SLATE,
            accent: lab_coat::COBALT,
            secondary: lab_coat::TEAL,
            border: lab_coat::BORDER_LIGHT,
            focus: lab_coat::COBALT,
            danger: lab_coat::DANGER_LIGHT,
            muted: lab_coat::TEXT_MUTED_LIGHT,
        }
    }
    pub fn for_theme(t: Theme) -> Self {
        match t {
            Theme::Dark => Self::dark(),
            Theme::Light => Self::light(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::lab_coat;
    #[test]
    fn dark_accent_meets_aa_on_slate() {
        assert_eq!(ThemeColors::dark().accent, lab_coat::COBALT_ON_DARK);
    }
    #[test]
    fn light_bg() {
        assert!(ThemeColors::light().bg.starts_with("#f"));
    }
    #[test]
    fn for_dark() {
        assert_eq!(ThemeColors::for_theme(Theme::Dark).accent, ThemeColors::dark().accent);
    }
    #[test]
    fn for_light() {
        assert_eq!(ThemeColors::for_theme(Theme::Light).bg, ThemeColors::light().bg);
    }
    #[test]
    fn secondary_stays_lab_coat_teal_family() {
        assert_eq!(ThemeColors::dark().secondary, lab_coat::TEAL_ON_DARK);
        assert_eq!(ThemeColors::light().secondary, lab_coat::TEAL);
    }
    #[test]
    fn light_focus_is_lab_coat_cobalt() {
        assert_eq!(ThemeColors::light().focus, lab_coat::COBALT);
        assert_eq!(ThemeColors::light().accent, lab_coat::COBALT);
    }
    #[test]
    fn dark_focus_matches_on_dark_accent() {
        assert_eq!(ThemeColors::dark().focus, ThemeColors::dark().accent);
    }
    #[test]
    fn danger_and_muted_present() {
        assert!(ThemeColors::dark().danger.starts_with('#'));
        assert!(ThemeColors::dark().muted.starts_with('#'));
    }
}
