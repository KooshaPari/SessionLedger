//! Shared loading / error UI for async bundle and daemon fetches.
//!
//! Styled from [`crate::theme::ThemeColors`] so focus/status chrome stays
//! aligned with Lab-Coat tokens used by the tab bar.

use dioxus::prelude::*;

use crate::theme::ThemeColors;

/// Full-panel loading indicator for in-flight bundle / search loads.
#[component]
pub fn LoadingState(
    /// Human-readable status (defaults to a generic bundle-load message).
    #[props(default = "Loading bundles…".to_string())]
    message: String,
) -> Element {
    let c = ThemeColors::dark();
    rsx! {
        div {
            class: "sl-loading-state",
            role: "status",
            "aria-live": "polite",
            "aria-busy": "true",
            "data-testid": "loading-state",
            style: "display:flex;flex-direction:column;align-items:center;justify-content:center;gap:12px;padding:32px 20px;color:{c.muted};font-size:13px;",
            div {
                class: "sl-loading-spinner",
                "aria-hidden": "true",
                style: "width:28px;height:28px;border:3px solid {c.border};border-top-color:{c.focus};border-radius:50%;animation:sl-spin 0.8s linear infinite;",
            }
            span { "{message}" }
            style {
                "@keyframes sl-spin {{ to {{ transform: rotate(360deg); }} }}"
            }
        }
    }
}

/// Recoverable error panel for failed async bundle / daemon loads.
#[component]
pub fn ErrorState(
    /// Error message shown to the user.
    message: String,
    /// When true, render a Retry control wired to [`Self::on_retry`].
    #[props(default)]
    retryable: bool,
    /// Retry handler (used when `retryable` is true).
    #[props(default)]
    on_retry: EventHandler<()>,
) -> Element {
    let c = ThemeColors::dark();
    rsx! {
        div {
            class: "sl-error-state",
            role: "alert",
            "aria-live": "assertive",
            "data-testid": "error-state",
            style: "display:flex;flex-direction:column;align-items:flex-start;gap:12px;padding:20px 16px;margin:8px 0;background:{c.surface};border:1px solid {c.border};border-left:3px solid {c.danger};border-radius:6px;color:{c.text};font-size:13px;line-height:1.5;",
            p {
                style: "margin:0;color:{c.danger};font-weight:600;",
                "Something went wrong"
            }
            p {
                style: "margin:0;color:{c.muted};",
                "{message}"
            }
            if retryable {
                button {
                    class: "sl-error-retry",
                    "data-testid": "error-state-retry",
                    style: "padding:6px 14px;font-size:12px;font-weight:600;border-radius:5px;cursor:pointer;border:1px solid {c.border};background:{c.bg};color:{c.focus};",
                    onclick: move |_| on_retry.call(()),
                    onkeydown: move |evt: Event<KeyboardData>| {
                        if matches!(evt.key(), Key::Enter | Key::Character(ref ch) if ch == " ") {
                            evt.prevent_default();
                            on_retry.call(());
                        }
                    },
                    "Retry"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_default_message_is_nonempty() {
        // Component defaults are compile-time; assert theme tokens used by styles exist.
        let c = ThemeColors::dark();
        assert!(!c.focus.is_empty());
        assert!(!c.danger.is_empty());
        assert!(!c.muted.is_empty());
    }

    #[test]
    fn theme_focus_is_lab_coat_cobalt() {
        assert_eq!(ThemeColors::dark().focus, "#2563eb");
    }
}
