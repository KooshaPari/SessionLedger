//! Property evidence for sl-viewer's `corpus_paths` module.
//!
//! Integration tests. The unit tests in `corpus_paths.rs` pin specific
//! values; these properties pin invariants over the full shape of
//! inputs the helpers can receive.
//!
//! `CorpusPathConfig` invariants:
//!  * `empty()` produces a config with zero custom paths.
//!  * `is_empty()` is true iff `custom_paths.is_empty()`.
//!  * `Default::default()` equals `empty()`.
//!  * JSON round-trip preserves `custom_paths` exactly (order-sensitive).
//!
//! `save_config_to` / `load_config_from` invariants:
//!  * Round-trip: `save_config_to(c, p); load_config_from(p) == c`.
//!  * Missing file yields `Ok(empty())` (no error surfaced).
//!  * Junk JSON surfaces an `Err` (never silently drops the file).
//!  * `save_config_to` creates missing parent directories.
//!
//! proptest is added to `sl-viewer/[dev-dependencies]` (mirroring the
//! workspace root); see PR #425 for the initial wiring.

use std::fs;
use std::path::PathBuf;

use proptest::prelude::*;
use sl_viewer::corpus_paths::{
    load_config_from, save_config_to, CorpusPathConfig,
};

// ── strategies ──────────────────────────────────────────────────────────────

/// Strategy for a list of relative / absolute path-like strings.
fn path_strategy() -> impl Strategy<Value = PathBuf> {
    prop::string::string_regex("[/a-zA-Z0-9._-]{1,40}")
        .expect("valid regex")
        .prop_map(PathBuf::from)
}

/// Strategy for a `CorpusPathConfig` with 0..6 paths.
fn config_strategy() -> impl Strategy<Value = CorpusPathConfig> {
    prop::collection::vec(path_strategy(), 0..6).prop_map(|paths| CorpusPathConfig {
        custom_paths: paths,
    })
}

/// Strategy for junk JSON content that is *not* valid `CorpusPathConfig`.
fn junk_json_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        // Plain garbage.
        "not json at all".to_owned(),
        // Empty string.
        String::new(),
        // Truncated object.
        r#"{"custom_paths":["#.to_owned(),
        // Wrong shape — `custom_paths` as a number.
        r#"{"custom_paths": 42}"#.to_owned(),
        // Wrong shape — `custom_paths` as an object.
        r#"{"custom_paths": {"k": "v"}}"#.to_owned(),
        // Trailing junk.
        r#"{"custom_paths": []} trailing junk"#.to_owned(),
    ])
}

// ── CorpusPathConfig pure reductions ────────────────────────────────────────

proptest! {
    /// Property: `empty()` returns a config with zero `custom_paths`.
    #[test]
    fn empty_has_no_custom_paths(_i in 0u8..4) {
        let config = CorpusPathConfig::empty();
        prop_assert!(config.custom_paths.is_empty());
        prop_assert!(config.is_empty());
    }

    /// Property: `Default::default()` equals `empty()`.
    #[test]
    fn default_equals_empty(_i in 0u8..4) {
        let a: CorpusPathConfig = CorpusPathConfig::default();
        let b: CorpusPathConfig = CorpusPathConfig::empty();
        prop_assert_eq!(a, b);
    }

    /// Property: `is_empty()` is true iff `custom_paths` is empty.
    #[test]
    fn is_empty_iff_no_paths(config in config_strategy()) {
        let expected = config.custom_paths.is_empty();
        prop_assert_eq!(config.is_empty(), expected);
    }

    /// Property: JSON round-trip preserves `custom_paths` exactly
    /// (order-sensitive — the on-disk contract is `Vec<PathBuf>`).
    #[test]
    fn json_round_trip_preserves_paths(config in config_strategy()) {
        let json = serde_json::to_string(&config).expect("serialize");
        let restored: CorpusPathConfig = serde_json::from_str(&json).expect("parse");
        prop_assert_eq!(restored, config);
    }

    /// Property: JSON round-trip is idempotent — round-tripping a
    /// restored config yields the same JSON bytes.
    #[test]
    fn json_round_trip_idempotent(config in config_strategy()) {
        let json1 = serde_json::to_string(&config).expect("serialize 1");
        let restored: CorpusPathConfig = serde_json::from_str(&json1).expect("parse 1");
        let json2 = serde_json::to_string(&restored).expect("serialize 2");
        prop_assert_eq!(json1, json2);
    }

    /// Property: `len(custom_paths)` is preserved through JSON
    /// round-trip (catches drift where the round-trip drops / dedups
    /// path entries).
    #[test]
    fn json_round_trip_preserves_len(config in config_strategy()) {
        let json = serde_json::to_string(&config).expect("serialize");
        let restored: CorpusPathConfig = serde_json::from_str(&json).expect("parse");
        prop_assert_eq!(restored.custom_paths.len(), config.custom_paths.len());
    }
}

// ── save_config_to / load_config_from ───────────────────────────────────────

proptest! {
    /// Property: `save_config_to` followed by `load_config_from` yields
    /// an equal config (round-trip). This is the contract the viewer's
    /// "user picks a folder" → "viewer reads it back" flow depends on.
    #[test]
    fn save_load_round_trip(config in config_strategy(), i in 0u8..3) {
        let dir = std::env::temp_dir().join(format!(
            "sessionledger-corpus-paths-roundtrip-{}-{}",
            std::process::id(),
            i,
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("corpus_paths.json");

        save_config_to(&config, &path).expect("save");
        let restored = load_config_from(&path).expect("load");

        prop_assert_eq!(restored, config);

        let _ = fs::remove_dir_all(&dir);
    }

    /// Property: `load_config_from(<missing>)` returns `Ok(empty())`
    /// — the viewer's first launch on a new machine must not fail
    /// just because the user hasn't picked anything yet.
    #[test]
    fn missing_file_yields_empty_config(i in 0u8..4) {
        let dir = std::env::temp_dir().join(format!(
            "sessionledger-corpus-paths-missing-{}-{}",
            std::process::id(),
            i,
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("does-not-exist.json");

        let result = load_config_from(&path);
        prop_assert!(result.is_ok(), "missing file must yield Ok, got {:?}", result.err());
        let config = result.unwrap();
        prop_assert!(config.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }

    /// Property: `load_config_from(<junk>)` surfaces an `Err` — the
    /// viewer must never silently drop the user's picks on a
    /// malformed file.
    #[test]
    fn junk_json_surfaces_error(junk in junk_json_strategy(), i in 0u8..3) {
        let dir = std::env::temp_dir().join(format!(
            "sessionledger-corpus-paths-junk-{}-{}",
            std::process::id(),
            i,
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("corpus_paths.json");
        fs::write(&path, junk.as_bytes()).expect("write junk");

        let result = load_config_from(&path);
        prop_assert!(result.is_err(), "junk JSON must surface as Err, got {result:?}");

        let _ = fs::remove_dir_all(&dir);
    }

    /// Property: `save_config_to` creates missing parent directories
    /// (the viewer may save into a fresh `~/.../SessionLedger/` that
    /// doesn't exist yet).
    #[test]
    fn save_creates_parent_directories(config in config_strategy(), i in 0u8..3) {
        let dir = std::env::temp_dir().join(format!(
            "sessionledger-corpus-paths-nested-{}-{}",
            std::process::id(),
            i,
        ));
        let _ = fs::remove_dir_all(&dir);
        let nested = dir.join("a").join("b").join("c").join("corpus_paths.json");
        prop_assert!(!nested.parent().expect("parent").exists());

        save_config_to(&config, &nested).expect("save nested");

        prop_assert!(nested.exists());

        let _ = fs::remove_dir_all(&dir);
    }
}
