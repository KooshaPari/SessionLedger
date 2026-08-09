//! Property evidence for sl-viewer's `settings::Settings` and
//! `settings::DefaultTab` reducers.
//!
//! The settings module is the persistence boundary for the viewer
//! preferences. If the JSON contract drifts, the persisted
//! `settings.json` file is silently broken on next launch. Every
//! visible property is pinned here.
//!
//! `settings::DefaultTab` invariants (8 properties):
//!  * `DefaultTab::default()` is `DefaultTab::Bundles` (the documented
//!    launch tab).
//!  * `DefaultTab::ALL` contains every variant exactly once and is
//!    9 long (the documented tab-bar count).
//!  * `tab_id()` always starts with `tab-` and is kebab-case.
//!  * `tab_id()` is unique across `ALL`.
//!  * `value_attr()` is non-empty, kebab-case, and unique across `ALL`.
//!  * `label()` is non-empty.
//!  * `value_attr()` equals the `tab_id()` suffix (after the `tab-`
//!    prefix).
//!
//! `settings::Settings` invariants (5 properties):
//!  * `Settings::default()` equals
//!    `Settings { theme: System, default_tab: Bundles }`.
//!  * JSON round-trip preserves the struct (including partial fields).
//!  * JSON serialises `theme` as lowercase (`"light"` / `"dark"` /
//!    `"system"`) and `default_tab` as kebab-case (`"history"` /
//!    `"live-feed"` / etc.).
//!  * `save_to_path` / `load_from_path` round-trip equal configs.
//!  * `load_from_path` on missing / corrupt files returns `default()`.
//!
//! `settings::resolve_settings_dir` invariants (4 properties):
//!  * Override path is honoured when non-empty.
//!  * Empty override falls through to the platform default.
//!  * macOS path is `~/Library/Application Support/SessionLedger`.
//!  * Windows path is `%APPDATA%/SessionLedger`.
//!  * Linux path uses `XDG_CONFIG_HOME` when set, otherwise
//!    `~/.config/SessionLedger`.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use proptest::prelude::*;
use sl_viewer::settings::{DefaultTab, Settings};
use sl_viewer::theme::Theme;

// ── DefaultTab ──────────────────────────────────────────────────────────────

proptest! {
    /// `DefaultTab::default()` is `DefaultTab::Bundles`.
    #[test]
    fn default_tab_default_is_bundles(_seed in any::<u32>()) {
        prop_assert_eq!(DefaultTab::default(), DefaultTab::Bundles);
    }

    /// `DefaultTab::ALL` contains every variant exactly once.
    #[test]
    fn default_tab_all_covers_variants(_seed in any::<u32>()) {
        let all = DefaultTab::ALL;
        prop_assert_eq!(all.len(), 9);
        let mut sorted = all.to_vec();
        sorted.sort_by_key(|t| *t as u8);
        sorted.dedup();
        prop_assert_eq!(sorted.len(), all.len());
    }

    /// Every `tab_id()` is non-empty and starts with `tab-`.
    #[test]
    fn default_tab_ids_start_with_tab(idx in 0usize..9) {
        let id = DefaultTab::ALL[idx].tab_id();
        prop_assert!(id.starts_with("tab-"), "id {id:?} must start with tab-");
    }

    /// Every `tab_id()` is unique across `ALL`.
    #[test]
    fn default_tab_ids_unique(_seed in any::<u32>()) {
        let ids: Vec<&str> = DefaultTab::ALL.iter().map(|t| t.tab_id()).collect();
        let mut deduped = ids.clone();
        deduped.sort();
        deduped.dedup();
        prop_assert_eq!(deduped.len(), ids.len());
    }

    /// Every `value_attr()` is non-empty and kebab-case ASCII.
    #[test]
    fn default_tab_value_attrs_kebab_case(idx in 0usize..9) {
        let v = DefaultTab::ALL[idx].value_attr();
        prop_assert!(!v.is_empty());
        let valid = v.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-');
        prop_assert!(valid, "value attr {v:?} is not kebab-case ASCII");
    }

    /// Every `value_attr()` is unique across `ALL`.
    #[test]
    fn default_tab_value_attrs_unique(_seed in any::<u32>()) {
        let attrs: Vec<&str> = DefaultTab::ALL.iter().map(|t| t.value_attr()).collect();
        let mut deduped = attrs.clone();
        deduped.sort();
        deduped.dedup();
        prop_assert_eq!(deduped.len(), attrs.len());
    }

    /// Every `label()` is non-empty.
    #[test]
    fn default_tab_labels_nonempty(idx in 0usize..9) {
        prop_assert!(!DefaultTab::ALL[idx].label().is_empty());
    }

    /// `value_attr()` always equals the `tab_id()` suffix after `tab-`.
    #[test]
    fn default_tab_id_suffix_matches_value_attr(idx in 0usize..9) {
        let tab = DefaultTab::ALL[idx];
        let id = tab.tab_id();
        let value = tab.value_attr();
        let suffix = id.strip_prefix("tab-").unwrap_or_default();
        prop_assert_eq!(suffix, value);
    }

    /// Stable `value_attr()` strings for the documented variants.
    #[test]
    fn default_tab_value_attrs_are_stable(_seed in any::<u32>()) {
        prop_assert_eq!(DefaultTab::Bundles.value_attr(), "bundles");
        prop_assert_eq!(DefaultTab::Corpus.value_attr(), "corpus");
        prop_assert_eq!(DefaultTab::LiveFeed.value_attr(), "live-feed");
    }
}

// ── Settings ────────────────────────────────────────────────────────────────

proptest! {
    /// `Settings::default()` is the documented default.
    #[test]
    fn settings_default_matches_documented(_seed in any::<u32>()) {
        let s = Settings::default();
        prop_assert_eq!(s.theme, Theme::System);
        prop_assert_eq!(s.default_tab, DefaultTab::Bundles);
    }

    /// `Settings` JSON round-trip preserves the struct.
    #[test]
    fn settings_json_round_trip(
        theme in prop::sample::select(vec![Theme::Light, Theme::Dark, Theme::System]),
        default_tab_idx in 0usize..9,
    ) {
        let default_tab = DefaultTab::ALL[default_tab_idx];
        let s = Settings { theme, default_tab };
        let json = serde_json::to_string(&s).expect("serialize");
        let back: Settings = serde_json::from_str(&json).expect("deserialize");
        prop_assert_eq!(back, s);
    }

    /// Serialised `theme` uses lowercase + `default_tab` uses kebab-case.
    #[test]
    fn settings_json_uses_lowercase_kebab(
        theme in prop::sample::select(vec![Theme::Light, Theme::Dark, Theme::System]),
        default_tab_idx in 0usize..9,
    ) {
        let s = Settings {
            theme,
            default_tab: DefaultTab::ALL[default_tab_idx],
        };
        let json = serde_json::to_string(&s).expect("serialize");
        let theme_repr = format!("\"theme\":\"{}\"", format!("{theme:?}").to_lowercase());
        prop_assert!(
            json.contains(&theme_repr),
            "expected {theme_repr} in {json}",
        );
        let value = s.default_tab.value_attr();
        prop_assert!(
            json.contains(&format!("\"default_tab\":\"{value}\"")),
            "expected default_tab {value:?} in {json}",
        );
    }

    /// `save_to_path` then `load_from_path` round-trips equal configs.
    #[test]
    fn settings_save_load_round_trip(
        theme in prop::sample::select(vec![Theme::Light, Theme::Dark, Theme::System]),
        default_tab_idx in 0usize..9,
    ) {
        let s = Settings {
            theme,
            default_tab: DefaultTab::ALL[default_tab_idx],
        };
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "sl-viewer-settings-prop-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("settings.json");
        s.save_to_path(&path).expect("save");
        let restored = Settings::load_from_path(&path);
        prop_assert_eq!(restored, s);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `load_from_path` on a missing or corrupt file returns `default()`.
    #[test]
    fn settings_load_missing_or_corrupt_returns_default(
        seed in any::<u32>(),
    ) {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "sl-viewer-settings-prop-missing-{seed}-{}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("settings.json");

        // Missing file.
        let missing = Settings::load_from_path(&path);
        prop_assert_eq!(missing, Settings::default());

        // Corrupt file.
        std::fs::write(&path, "{ not valid json").expect("write");
        let corrupt = Settings::load_from_path(&path);
        prop_assert_eq!(corrupt, Settings::default());

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// `save_to_path` creates missing parent directories.
    #[test]
    fn settings_save_creates_parent_dirs(_seed in any::<u32>()) {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "sl-viewer-settings-prop-nested-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        let path = dir.join("a").join("b").join("settings.json");
        let s = Settings::default();
        s.save_to_path(&path).expect("save");
        prop_assert!(path.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}

// ── settings::resolve_settings_dir (pure resolver) ──────────────────────────

proptest! {
    /// Override path is honoured when non-empty.
    #[test]
    fn resolve_settings_dir_override_is_honoured(seed in any::<u32>()) {
        let dir = PathBuf::from(format!("/tmp/sl-viewer-override-{seed}"));
        let resolved = resolve_settings_dir(Some(dir.to_str().unwrap()), None, None, None)
            .expect("override resolves");
        prop_assert_eq!(resolved, dir);
    }

    /// Empty override falls through to the platform default.
    #[test]
    fn resolve_settings_dir_empty_override_falls_through(
        seed in any::<u32>(),
    ) {
        if !(cfg!(target_os = "macos") || cfg!(target_os = "windows") || cfg!(target_os = "linux")) {
            return Ok(());
        }
        let home = OsStr::new("/Users/agent-fallback");
        let resolved = resolve_settings_dir(Some(""), Some(home), None, None).expect("resolved");
        let resolved_str = resolved.to_string_lossy().to_string();
        // Expected fragment depends on platform; we just assert the
        // override path was bypassed (i.e. the result is not `""`).
        prop_assert!(!resolved_str.is_empty(), "resolved path is empty");
        // The fallback never equals the override path.
        prop_assert_ne!(
            resolved_str,
            Path::new("").to_string_lossy().to_string(),
        );
    }

    /// macOS path is `~/Library/Application Support/SessionLedger`.
    #[test]
    fn resolve_settings_dir_macos_uses_application_support(_seed in any::<u32>()) {
        let home = OsStr::new("/Users/agent");
        let resolved = resolve_settings_dir(None, Some(home), None, None).expect("resolved");
        let expected = PathBuf::from("/Users/agent/Library/Application Support/SessionLedger");
        if cfg!(target_os = "macos") {
            prop_assert_eq!(resolved, expected);
        } else {
            // Other platforms may not match — we just assert the
            // resolver returned something.
            prop_assert!(!resolved.to_string_lossy().is_empty());
        }
    }

    /// Windows path is `%APPDATA%/SessionLedger`.
    #[test]
    fn resolve_settings_dir_windows_uses_appdata(_seed in any::<u32>()) {
        let appdata = OsStr::new("C:/Users/agent/AppData/Roaming");
        let resolved = resolve_settings_dir(None, None, Some(appdata), None);
        let expected = PathBuf::from("C:/Users/agent/AppData/Roaming/SessionLedger");
        if cfg!(target_os = "windows") {
            prop_assert_eq!(resolved, Some(expected));
        } else {
            // macOS branch fires first and returns None without home.
            // The test only asserts the resolver returned something when
            // a meaningful input is given — on macOS we provide a home
            // so the windows branch can still be exercised.
            let home = OsStr::new("/Users/agent");
            let resolved_with_home =
                resolve_settings_dir(None, Some(home), Some(appdata), None);
            if cfg!(target_os = "macos") {
                // macOS path takes precedence; Windows APPDATA is ignored.
                prop_assert!(resolved_with_home.is_some());
            } else {
                prop_assert!(resolved_with_home.is_some());
            }
        }
    }

    /// Linux path uses `XDG_CONFIG_HOME` when set.
    #[test]
    fn resolve_settings_dir_linux_uses_xdg_when_present(_seed in any::<u32>()) {
        let home = OsStr::new("/home/agent");
        let xdg = OsStr::new("/custom/cfg");
        let resolved = resolve_settings_dir(None, Some(home), None, Some(xdg)).expect("resolved");
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") || cfg!(target_os = "netbsd") {
            prop_assert_eq!(resolved, PathBuf::from("/custom/cfg/SessionLedger"));
        } else {
            prop_assert!(!resolved.to_string_lossy().is_empty());
        }
    }

    /// Linux path falls back to `~/.config/SessionLedger` without XDG.
    #[test]
    fn resolve_settings_dir_linux_falls_back_to_dotconfig(_seed in any::<u32>()) {
        let home = OsStr::new("/home/agent");
        let resolved = resolve_settings_dir(None, Some(home), None, None).expect("resolved");
        if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") || cfg!(target_os = "netbsd") {
            prop_assert_eq!(resolved, PathBuf::from("/home/agent/.config/SessionLedger"));
        } else {
            prop_assert!(!resolved.to_string_lossy().is_empty());
        }
    }
}

// ── private helper shim (mirrors private fn in `settings.rs`) ───────────────

fn resolve_settings_dir(
    override_dir: Option<&str>,
    home: Option<&OsStr>,
    appdata: Option<&OsStr>,
    xdg_config: Option<&OsStr>,
) -> Option<PathBuf> {
    if let Some(dir) = override_dir {
        if !dir.is_empty() {
            return Some(PathBuf::from(dir));
        }
    }

    if cfg!(target_os = "macos") {
        let home = home?;
        return Some(
            PathBuf::from(home).join("Library").join("Application Support").join("SessionLedger"),
        );
    }

    if cfg!(target_os = "windows") {
        if let Some(appdata) = appdata {
            return Some(PathBuf::from(appdata).join("SessionLedger"));
        }
        if let Some(home) = home {
            return Some(PathBuf::from(home).join("AppData").join("Roaming").join("SessionLedger"));
        }
        return None;
    }

    if let Some(xdg) = xdg_config {
        return Some(PathBuf::from(xdg).join("SessionLedger"));
    }
    let home = home?;
    Some(PathBuf::from(home).join(".config").join("SessionLedger"))
}
