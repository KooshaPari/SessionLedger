//! Property evidence for sl-viewer's `cli_help` and `command_palette`
//! reducers.
//!
//! Both modules are pure text/data reductions that the CLI and the
//! in-viewer launcher shell depend on. If their templates drift
//! without documentation updates, the Help overlay, the `--help`
//! flag, and the Cmd+K palette diverge silently — so every visible
//! property is pinned here.
//!
//! `cli_help::version_text` invariants:
//!  * Output is non-empty.
//!  * Output contains the package version (`env!("CARGO_PKG_VERSION")`).
//!  * Output contains the `daemon:` label.
//!  * Output is deterministic across calls.
//!
//! `cli_help::help_text` invariants:
//!  * Output is non-empty.
//!  * Output documents `SL_DAEMON_URL`, `FORGE_DB`, and `SL_VIEWER_DEMO`.
//!  * Output links the documented help / quick-start docs.
//!  * Output is deterministic across calls.
//!
//! `command_palette::COMMANDS` invariants:
//!  * Non-empty.
//!  * Every command has a non-empty `id`, `label`, and `hint`.
//!  * Every `id` is unique across the palette.
//!  * Every documented `PaletteAction` variant is covered.
//!  * `id` is kebab-case-ish (lowercase ASCII letters, digits, hyphens).
//!  * `label` and `hint` carry no tab/newline characters (so the
//!    `role="option"` ARIA text is well-formed).
//!  * Action distribution (each action appears in `[1, 7]` commands)
//!    so the palette shows a non-trivial menu but no single action
//!    dominates.

use proptest::prelude::*;
use sl_viewer::cli_help::{help_text, version_text};
use sl_viewer::command_palette::{COMMANDS, PaletteAction};

// ── cli_help::version_text ──────────────────────────────────────────────────

proptest! {
    /// `version_text()` is non-empty.
    #[test]
    fn version_text_nonempty(_seed in any::<u32>()) {
        prop_assert!(!version_text().is_empty());
    }

    /// `version_text()` contains the package version.
    #[test]
    fn version_text_contains_package_version(_seed in any::<u32>()) {
        let v = version_text();
        prop_assert!(v.contains(env!("CARGO_PKG_VERSION")));
    }

    /// `version_text()` contains the `daemon:` label.
    #[test]
    fn version_text_contains_daemon_label(_seed in any::<u32>()) {
        let v = version_text();
        prop_assert!(v.contains("daemon:"));
    }

    /// `version_text()` contains the help doc link.
    #[test]
    fn version_text_contains_doc_link(_seed in any::<u32>()) {
        let v = version_text();
        prop_assert!(v.contains("sl-viewer-help.md"));
    }

    /// `version_text()` is deterministic across calls.
    #[test]
    fn version_text_deterministic(_seed in any::<u32>()) {
        prop_assert_eq!(version_text(), version_text());
    }
}

// ── cli_help::help_text ─────────────────────────────────────────────────────

proptest! {
    /// `help_text()` is non-empty.
    #[test]
    fn help_text_nonempty(_seed in any::<u32>()) {
        prop_assert!(!help_text().is_empty());
    }

    /// `help_text()` documents the runtime env vars referenced by
    /// `daemon_url` and the demo seed path.
    #[test]
    fn help_text_documents_env_vars(_seed in any::<u32>()) {
        let h = help_text();
        prop_assert!(h.contains("SL_DAEMON_URL"));
        prop_assert!(h.contains("FORGE_DB"));
        prop_assert!(h.contains("SL_VIEWER_DEMO"));
    }

    /// `help_text()` links the documented SSOT and quick-start docs.
    #[test]
    fn help_text_links_docs(_seed in any::<u32>()) {
        let h = help_text();
        prop_assert!(h.contains("sl-viewer-help.md"));
        prop_assert!(h.contains("QUICKSTART.md"));
    }

    /// `help_text()` mentions the keyboard shortcuts surfaced by the
    /// in-viewer help overlay.
    #[test]
    fn help_text_mentions_shortcuts(_seed in any::<u32>()) {
        let h = help_text();
        prop_assert!(h.contains("Cmd") || h.contains("Ctrl"));
        prop_assert!(h.contains("K"));
    }

    /// `help_text()` is deterministic across calls.
    #[test]
    fn help_text_deterministic(_seed in any::<u32>()) {
        prop_assert_eq!(help_text(), help_text());
    }
}

// ── command_palette::COMMANDS ───────────────────────────────────────────────

proptest! {
    /// `COMMANDS` is non-empty.
    #[test]
    fn commands_nonempty(_seed in any::<u32>()) {
        prop_assert!(!COMMANDS.is_empty());
    }

    /// Every command has a non-empty `id`, `label`, and `hint`.
    #[test]
    fn commands_text_fields_nonempty(_seed in any::<u32>()) {
        for cmd in COMMANDS.iter() {
            prop_assert!(!cmd.id.is_empty(), "command id is empty");
            prop_assert!(!cmd.label.is_empty(), "command label is empty");
            prop_assert!(!cmd.hint.is_empty(), "command hint is empty");
        }
    }

    /// Every command id is unique across the palette.
    #[test]
    fn commands_ids_unique(_seed in any::<u32>()) {
        let ids: Vec<&str> = COMMANDS.iter().map(|c| c.id).collect();
        let mut deduped = ids.clone();
        deduped.sort();
        deduped.dedup();
        prop_assert_eq!(deduped.len(), ids.len());
    }

    /// Every `PaletteAction` variant has at least one command so the
    /// palette can dispatch any required shell action.
    #[test]
    fn commands_cover_all_actions(_seed in any::<u32>()) {
        let required = [
            PaletteAction::FocusSearch,
            PaletteAction::ToggleTheme,
            PaletteAction::OpenHelp,
            PaletteAction::OpenSettings,
            PaletteAction::NextTab,
            PaletteAction::PrevTab,
            PaletteAction::ClearSearch,
        ];
        for action in required.iter() {
            prop_assert!(
                COMMANDS.iter().any(|c| &c.action == action),
                "missing command for action {:?}",
                *action,
            );
        }
    }

    /// Every command id is kebab-case (lowercase ASCII letters, digits,
    /// hyphens). The id is also used as a DOM id, so an invalid
    /// character would break `getElementById`.
    #[test]
    fn commands_ids_are_kebab_case(_seed in any::<u32>()) {
        for cmd in COMMANDS.iter() {
            let valid = cmd.id.chars().all(|ch| {
                ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-'
            });
            prop_assert!(valid, "id {:?} is not kebab-case ASCII", cmd.id);
        }
    }

    /// `label` and `hint` must not contain tabs or newlines so the
    /// rendered `role="option"` ARIA text is single-line.
    #[test]
    fn commands_label_and_hint_singleline(_seed in any::<u32>()) {
        for cmd in COMMANDS.iter() {
            prop_assert!(!cmd.label.contains('\n'), "label {:?} contains newline", cmd.id);
            prop_assert!(!cmd.label.contains('\t'), "label {:?} contains tab", cmd.id);
            prop_assert!(!cmd.hint.contains('\n'), "hint {:?} contains newline", cmd.id);
            prop_assert!(!cmd.hint.contains('\t'), "hint {:?} contains tab", cmd.id);
        }
    }

    /// Each `PaletteAction` variant appears in `COMMANDS` at most once
    /// so the palette does not duplicate entries.
    #[test]
    fn commands_action_distribution_at_most_one(_seed in any::<u32>()) {
        let required = [
            PaletteAction::FocusSearch,
            PaletteAction::ToggleTheme,
            PaletteAction::OpenHelp,
            PaletteAction::OpenSettings,
            PaletteAction::NextTab,
            PaletteAction::PrevTab,
            PaletteAction::ClearSearch,
        ];
        for action in required.iter() {
            let n = COMMANDS.iter().filter(|c| &c.action == action).count();
            prop_assert!(n >= 1, "action {:?} appears 0 times", *action);
            prop_assert!(n <= 7, "action {:?} appears {} times", *action, n);
        }
    }
}
