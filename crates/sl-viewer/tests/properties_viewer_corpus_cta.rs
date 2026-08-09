//! Property evidence for sl-viewer's `corpus_cta` constants — the
//! first-run "Open corpus…" CTA's URL / DOM-id / storage-key SSOT.
//!
//! If any of these strings drift, the in-viewer CTA silently breaks
//! (the file picker never opens, the quick-start link 404s, the
//! localStorage hint stops round-tripping). Every shown constant is
//! pinned here.
//!
//! `corpus_cta::QUICKSTART_URL` invariants:
//!  * Points at the canonical repo / docs path (SSoT).
//!  * Uses HTTPS so the desktop helper `open` / `xdg-open` cannot
//!    leak a cleartext follow-up.
//!  * Ends in the documented `QUICKSTART.md` filename so the
//!    repo-relative fallback doc name matches.
//!
//! `corpus_cta::QUICKSTART_CORPUS_DOC` invariants:
//!  * Matches the `docs/guides/quick-start/QUICKSTART.md` repo path
//!    (the SSoT for the on-disk fallback log line).
//!
//! `corpus_cta::CORPUS_PICKER_INPUT_ID` invariants:
//!  * Non-empty and kebab-case ASCII (used as a DOM id).
//!  * Stable (gated by `document.getElementById`).
//!
//! `corpus_cta::FORGE_DB_HINT_STORAGE_KEY` invariants:
//!  * Non-empty and kebab-case ASCII (used as a localStorage key).

use proptest::prelude::*;
use sl_viewer::corpus_cta::{
    CORPUS_PICKER_INPUT_ID, FORGE_DB_HINT_STORAGE_KEY, QUICKSTART_CORPUS_DOC, QUICKSTART_URL,
};

proptest! {
    /// `QUICKSTART_URL` is non-empty.
    #[test]
    fn quickstart_url_nonempty(_seed in any::<u32>()) {
        prop_assert!(!QUICKSTART_URL.is_empty());
    }

    /// `QUICKSTART_URL` uses HTTPS so the desktop helper cannot leak a
    /// cleartext follow-up.
    #[test]
    fn quickstart_url_is_https(_seed in any::<u32>()) {
        prop_assert!(QUICKSTART_URL.starts_with("https://"));
    }

    /// `QUICKSTART_URL` ends in the documented `QUICKSTART.md` filename so
    /// the repo-relative fallback doc name matches.
    #[test]
    fn quickstart_url_ends_in_quickstart_md(_seed in any::<u32>()) {
        prop_assert!(QUICKSTART_URL.ends_with("QUICKSTART.md"));
    }

    /// `QUICKSTART_URL` points at the canonical repo URL.
    #[test]
    fn quickstart_url_points_at_repo(_seed in any::<u32>()) {
        prop_assert!(QUICKSTART_URL.contains("KooshaPari/SessionLedger"));
    }

    /// `QUICKSTART_CORPUS_DOC` is non-empty and matches the
    /// `docs/guides/quick-start/QUICKSTART.md` repo path.
    #[test]
    fn quickstart_corpus_doc_is_repo_path(_seed in any::<u32>()) {
        prop_assert!(!QUICKSTART_CORPUS_DOC.is_empty());
        prop_assert_eq!(QUICKSTART_CORPUS_DOC, "docs/guides/quick-start/QUICKSTART.md");
    }

    /// `QUICKSTART_URL`'s file basename matches `QUICKSTART_CORPUS_DOC`'s
    /// basename so the desktop fallback URL and the in-repo doc name
    /// stay aligned.
    #[test]
    fn quickstart_url_and_doc_basenames_match(_seed in any::<u32>()) {
        let url_basename = QUICKSTART_URL.rsplit('/').next().unwrap_or_default();
        let doc_basename = QUICKSTART_CORPUS_DOC.rsplit('/').next().unwrap_or_default();
        prop_assert_eq!(url_basename, doc_basename);
    }

    /// `CORPUS_PICKER_INPUT_ID` is non-empty and kebab-case ASCII so
    /// `document.getElementById` always resolves it.
    #[test]
    fn corpus_picker_input_id_is_kebab_case(_seed in any::<u32>()) {
        prop_assert!(!CORPUS_PICKER_INPUT_ID.is_empty());
        let valid = CORPUS_PICKER_INPUT_ID
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-');
        prop_assert!(valid, "id {:?} is not kebab-case ASCII", CORPUS_PICKER_INPUT_ID);
    }

    /// `FORGE_DB_HINT_STORAGE_KEY` is non-empty and kebab-case ASCII so
    /// the localStorage round-trip never fails on a malformed key.
    #[test]
    fn forge_db_hint_storage_key_is_kebab_case(_seed in any::<u32>()) {
        prop_assert!(!FORGE_DB_HINT_STORAGE_KEY.is_empty());
        let valid = FORGE_DB_HINT_STORAGE_KEY
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-');
        prop_assert!(valid, "key {:?} is not kebab-case ASCII", FORGE_DB_HINT_STORAGE_KEY);
    }

    /// `CORPUS_PICKER_INPUT_ID` and `FORGE_DB_HINT_STORAGE_KEY` are
    /// distinct strings so the picker never mistakes the localStorage
    /// hint for the DOM id (and vice versa).
    #[test]
    fn picker_id_and_storage_key_are_distinct(_seed in any::<u32>()) {
        prop_assert_ne!(CORPUS_PICKER_INPUT_ID, FORGE_DB_HINT_STORAGE_KEY);
    }
}
