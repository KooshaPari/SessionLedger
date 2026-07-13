//! Web visual-fixture helpers for Playwright golden harnesses.

/// Returns true when the browser URL query contains `fixture=<name>`.
#[must_use]
pub fn query_fixture_active(name: &str) -> bool {
    #[cfg(feature = "web")]
    {
        let window = match web_sys::window() {
            Some(window) => window,
            None => return false,
        };
        let location = window.location();
        let search = location.search().unwrap_or_default();
        let params = match web_sys::UrlSearchParams::new_with_str(&search) {
            Ok(params) => params,
            Err(_) => return false,
        };
        return params.get("fixture").as_deref() == Some(name);
    }

    #[cfg(not(feature = "web"))]
    {
        let _ = name;
        false
    }
}
