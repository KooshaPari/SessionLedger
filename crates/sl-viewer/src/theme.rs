use serde::{Deserialize, Serialize};
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
    /// Focus-ring / keyboard-highlight color (Lab-Coat cobalt).
    pub focus: &'static str,
    /// Error / destructive text.
    pub danger: &'static str,
    /// Muted secondary text.
    pub muted: &'static str,
}
impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            bg: "#111827",
            surface: "#1f2937",
            text: "#f3f4f6",
            accent: "#2563eb",
            secondary: "#14b8a6",
            border: "#374151",
            focus: "#2563eb",
            danger: "#f87171",
            muted: "#b6bfcc",
        }
    }
    pub fn light() -> Self {
        Self {
            bg: "#f6f8fa",
            surface: "#ffffff",
            text: "#1f2937",
            accent: "#2563eb",
            secondary: "#14b8a6",
            border: "#d8dee8",
            focus: "#2563eb",
            danger: "#b91c1c",
            muted: "#5c5f6e",
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
    #[test]
    fn dark_accent_is_lab_coat_cobalt() {
        assert_eq!(ThemeColors::dark().accent, "#2563eb");
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
    fn secondary_stays_lab_coat_teal() {
        assert_eq!(ThemeColors::dark().secondary, "#14b8a6");
        assert_eq!(ThemeColors::light().secondary, "#14b8a6");
    }
    #[test]
    fn focus_is_lab_coat_cobalt() {
        assert_eq!(ThemeColors::dark().focus, "#2563eb");
        assert_eq!(ThemeColors::light().focus, "#2563eb");
    }
    #[test]
    fn danger_and_muted_present() {
        assert!(ThemeColors::dark().danger.starts_with('#'));
        assert!(ThemeColors::dark().muted.starts_with('#'));
    }
}
