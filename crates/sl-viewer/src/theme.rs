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
}
impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            bg: "#0f172a",
            surface: "#1e293b",
            text: "#e2e8f0",
            accent: "#7c3aed",
            secondary: "#06b6d4",
            border: "#334155",
        }
    }
    pub fn light() -> Self {
        Self {
            bg: "#f8fafc",
            surface: "#ffffff",
            text: "#0f172a",
            accent: "#7c3aed",
            secondary: "#0891b2",
            border: "#e2e8f0",
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
    fn dark_accent() {
        assert_eq!(ThemeColors::dark().accent, "#7c3aed");
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
    fn secondary_differs() {
        assert_ne!(ThemeColors::dark().secondary, ThemeColors::light().secondary);
    }
}
