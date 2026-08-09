//! Property evidence for sl-viewer's `web_exports::WebExportProvider`
//! reductions.
//!
//! Integration tests. The unit tests in `web_exports.rs` pin specific
//! values; these properties pin invariants over the full set of
//! `WebExportProvider` variants.
//!
//! `WebExportProvider` invariants:
//!  * `label` is non-empty, distinct per variant, and contains no
//!    whitespace other than single ASCII spaces.
//!  * `corpus` returns a `Corpus::ChatGptWeb` / `Corpus::ClaudeWeb` /
//!    `Corpus::GeminiWeb` variant exactly matching the provider's
//!    web-export identity (no future drift to a desktop corpus).
//!  * `default_subdir` is non-empty, distinct per variant, and equals
//!    the corresponding `label` (so the directory under `~/Downloads`
//!    lines up with the user-facing provider name).
//!  * `corpus` is total (every variant maps to a known corpus).
//!
//! `web_export_roots_with_env` invariants:
//!  * With `explicit = None`, the output is a subset of the three
//!    defaults (no extras leak in) — each entry's provider is one of
//!    the three documented providers.
//!  * With `explicit = None`, every default entry that exists on disk
//!    appears in the output exactly once.
//!
//! proptest is added to `sl-viewer/[dev-dependencies]` (mirroring the
//! workspace root); see PR #425 for the initial wiring.

use std::path::PathBuf;

use proptest::prelude::*;
use session_ledger::domain::session::Corpus;
use sl_viewer::web_exports::{web_export_roots_with_env, WebExportProvider};

// ── strategies ──────────────────────────────────────────────────────────────

fn provider_strategy() -> impl Strategy<Value = WebExportProvider> {
    prop::sample::select(vec![
        WebExportProvider::ChatGpt,
        WebExportProvider::Claude,
        WebExportProvider::Gemini,
    ])
}

// ── WebExportProvider::label ────────────────────────────────────────────────

proptest! {
    /// Property: every `label()` is non-empty. Guards against a future
    /// variant whose label accidentally becomes empty (UI rendering
    /// would crash on `String::new()` in the badge).
    #[test]
    fn label_is_nonempty(provider in provider_strategy()) {
        prop_assert!(!provider.label().is_empty());
    }

    /// Property: every `label()` contains no whitespace other than
    /// single ASCII spaces (no tabs / newlines / double-spaces that
    /// would look broken in a badge).
    #[test]
    fn label_is_well_formed(provider in provider_strategy()) {
        let label = provider.label();
        prop_assert!(!label.contains('\t'));
        prop_assert!(!label.contains('\n'));
        prop_assert!(!label.contains("  "));
    }

    /// Property: distinct providers produce distinct labels (no
    /// accidental aliasing in the UI badge).
    #[test]
    fn labels_are_distinct(
        a in provider_strategy(),
        b in provider_strategy(),
    ) {
        if a != b {
            prop_assert_ne!(a.label(), b.label());
        }
    }
}

// ── WebExportProvider::corpus ───────────────────────────────────────────────

proptest! {
    /// Property: `corpus()` is total — every provider variant maps to
    /// a known `Corpus` variant (no panics, no surprise fallback).
    #[test]
    fn corpus_is_total(provider in provider_strategy()) {
        let corpus = provider.corpus();
        prop_assert!(matches!(
            corpus,
            Corpus::ChatGptWeb | Corpus::ClaudeWeb | Corpus::GeminiWeb
        ));
    }

    /// Property: distinct providers map to distinct corpora (catches
    /// drift where two providers are silently merged into one corpus).
    #[test]
    fn corpus_is_injective(
        a in provider_strategy(),
        b in provider_strategy(),
    ) {
        if a != b {
            prop_assert_ne!(a.corpus(), b.corpus());
        }
    }
}

// ── WebExportProvider::default_subdir ───────────────────────────────────────

proptest! {
    /// Property: `default_subdir()` is non-empty.
    #[test]
    fn default_subdir_is_nonempty(provider in provider_strategy()) {
        prop_assert!(!provider.default_subdir().is_empty());
    }

    /// Property: distinct providers have distinct default subdirs.
    #[test]
    fn default_subdirs_are_distinct(
        a in provider_strategy(),
        b in provider_strategy(),
    ) {
        if a != b {
            prop_assert_ne!(a.default_subdir(), b.default_subdir());
        }
    }

    /// Property: `default_subdir()` equals `label()`. The directory
    /// under `~/Downloads` must match the user-facing provider name.
    #[test]
    fn default_subdir_matches_label(provider in provider_strategy()) {
        prop_assert_eq!(provider.default_subdir(), provider.label());
    }
}

// ── web_export_roots_with_env ───────────────────────────────────────────────

proptest! {
    /// Property: with `explicit = None`, the output is a subset of the
    /// three documented web-export providers (no extras leak in).
    /// We construct a non-existent home directory so none of the
    /// defaults exist on disk — the output is therefore empty.
    #[test]
    fn roots_with_no_explicit_returns_empty_for_missing_home(
        _i in 0u8..8,
    ) {
        // Use a path that certainly doesn't exist (a single-segment
        // filename under "/") so all defaults are absent.
        let home = PathBuf::from("/__nonexistent_sessionledger_root__");
        let explicit = None;
        let roots = web_export_roots_with_env(&home, explicit);
        prop_assert!(roots.is_empty(), "got unexpected roots: {roots:?}");
    }

    /// Property: with `explicit = None`, every default entry whose
    /// path exists on disk appears in the output exactly once. The
    /// test creates a tempdir, materializes one of the three defaults
    /// (Claude), and asserts only that provider's root comes back.
    #[test]
    fn roots_with_no_explicit_filters_to_existing(
        i in 0u8..3,
    ) {
        // Each iteration picks one provider to materialize; the other
        // two defaults stay absent.
        let provider = [WebExportProvider::ChatGpt, WebExportProvider::Claude, WebExportProvider::Gemini]
            [i as usize];
        let tmp = std::env::temp_dir().join(format!(
            "sessionledger-test-roots-{}-{}",
            std::process::id(),
            i
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).expect("mkdir");
        let existing_path = tmp.join("Downloads").join(provider.default_subdir());
        std::fs::create_dir_all(&existing_path).expect("mkdir provider");

        let roots = web_export_roots_with_env(&tmp, None);
        prop_assert_eq!(roots.len(), 1);
        prop_assert_eq!(roots[0].0, provider);
        prop_assert_eq!(roots[0].1.clone(), existing_path);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// Property: with `explicit = None`, the providers in the output
    /// are always drawn from the documented three-provider set (no
    /// unknown provider variants leak in).
    #[test]
    fn roots_with_no_explicit_only_known_providers(
        _i in 0u8..4,
    ) {
        let home = std::env::temp_dir().join(format!(
            "sessionledger-test-providerset-{}",
            std::process::id(),
        ));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("mkdir home");
        for p in [
            WebExportProvider::ChatGpt,
            WebExportProvider::Claude,
            WebExportProvider::Gemini,
        ] {
            std::fs::create_dir_all(home.join("Downloads").join(p.default_subdir()))
                .expect("mkdir downloads");
        }

        let roots = web_export_roots_with_env(&home, None);
        prop_assert_eq!(roots.len(), 3);
        let mut providers: Vec<_> = roots.iter().map(|(p, _)| *p).collect();
        providers.sort_by_key(|p| p.label());
        let expected: Vec<_> = [
            WebExportProvider::ChatGpt,
            WebExportProvider::Claude,
            WebExportProvider::Gemini,
        ]
        .into_iter()
        .collect();
        prop_assert_eq!(providers, expected);

        let _ = std::fs::remove_dir_all(&home);
    }
}
