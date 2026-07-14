//! Minimal i18n scaffold (C01 L16).
//!
//! Phase-0 ships an English-only JSON catalog under `locales/en.json` plus a
//! lookup helper. No fluent/gettext/ICU runtime yet — additional locales and
//! plural/ICU rules are future hooks documented in `docs/ops/i18n.md`.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Default (and currently only) catalog locale.
pub const DEFAULT_LOCALE: &str = "en";

/// Embedded English catalog (compile-time include of `locales/en.json`).
const EN_CATALOG_JSON: &str = include_str!("../locales/en.json");

#[derive(Debug, Deserialize)]
struct CatalogFile {
    locale: String,
    messages: HashMap<String, String>,
}

/// Loaded message catalog for one locale.
#[derive(Debug, Clone)]
pub struct Catalog {
    locale: String,
    messages: HashMap<String, String>,
}

impl Catalog {
    /// Parse a JSON catalog document (`{ "locale", "messages": { key: text } }`).
    ///
    /// # Errors
    /// Returns a serde error when the JSON shape is invalid.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let file: CatalogFile = serde_json::from_str(json)?;
        Ok(Self { locale: file.locale, messages: file.messages })
    }

    /// Locale tag for this catalog (BCP 47-ish, e.g. `en`).
    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Look up `key`; returns `None` when missing.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(String::as_str)
    }

    /// Look up `key`, falling back to the key itself when missing.
    #[must_use]
    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.get(key).unwrap_or(key)
    }

    /// Number of message entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Whether the catalog has no messages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

static EN_CATALOG: OnceLock<Catalog> = OnceLock::new();

/// Return the embedded English catalog (parsed once).
///
/// # Panics
/// Panics only if the checked-in `locales/en.json` is invalid JSON — a
/// packaging bug caught by unit tests and `SelfCheck`.
#[must_use]
pub fn en_catalog() -> &'static Catalog {
    EN_CATALOG.get_or_init(|| {
        Catalog::from_json(EN_CATALOG_JSON)
            .expect("locales/en.json must parse as a message catalog")
    })
}

/// Translate `key` using the active Phase-0 catalog (English).
///
/// Missing keys return the key string so callers stay resilient while catalogs
/// grow. Future: accept a locale argument and fall back `requested → en → key`.
#[must_use]
pub fn t(key: &str) -> &str {
    en_catalog().get(key).unwrap_or(key)
}

/// Future hook: resolve a catalog for `locale` (today only `en` is loaded).
///
/// Returns [`None`] for unknown locales so callers can fall back explicitly.
#[must_use]
pub fn try_catalog(locale: &str) -> Option<&'static Catalog> {
    if locale.eq_ignore_ascii_case(DEFAULT_LOCALE) || locale.eq_ignore_ascii_case("en-US") {
        Some(en_catalog())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en_catalog_parses_and_has_core_keys() {
        let cat = en_catalog();
        assert_eq!(cat.locale(), "en");
        assert!(!cat.is_empty());
        assert_eq!(cat.t("viewer.app_title"), "SessionLedger");
        assert!(cat.get("cli.app_about").is_some());
        assert!(cat.get("viewer.empty_state").is_some());
    }

    #[test]
    fn missing_key_falls_back_to_key() {
        assert_eq!(t("does.not.exist"), "does.not.exist");
    }

    #[test]
    fn try_catalog_accepts_en_aliases_only() {
        assert!(try_catalog("en").is_some());
        assert!(try_catalog("en-US").is_some());
        assert!(try_catalog("fr").is_none());
    }
}
