//! Web visual-fixture helpers for Playwright golden harnesses.

/// Returns the `fixture` query parameter when present.
#[must_use]
pub fn query_fixture_name() -> Option<String> {
    #[cfg(feature = "web")]
    {
        let window = match web_sys::window() {
            Some(window) => window,
            None => return None,
        };
        let location = window.location();
        let search = location.search().unwrap_or_default();
        let params = match web_sys::UrlSearchParams::new_with_str(&search) {
            Ok(params) => params,
            Err(_) => return None,
        };
        return params.get("fixture").filter(|value| !value.trim().is_empty());
    }

    #[cfg(not(feature = "web"))]
    {
        None
    }
}

/// Returns true when the browser URL query contains `fixture=<name>`.
#[must_use]
pub fn query_fixture_active(name: &str) -> bool {
    query_fixture_name().as_deref() == Some(name)
}

/// Returns true when any visual golden fixture query is active.
#[must_use]
pub fn visual_fixture_active() -> bool {
    query_fixture_name().is_some()
}

/// Returns true when the launch-splash hold fixture is active (dark or light).
///
/// These routes keep `.launch-splash` pinned for Playwright goldens so reduced
/// motion and the normal dismiss timer do not remove it mid-capture.
#[must_use]
pub fn splash_hold_fixture_active() -> bool {
    matches!(query_fixture_name().as_deref(), Some("launch-splash") | Some("launch-splash-light"))
}
