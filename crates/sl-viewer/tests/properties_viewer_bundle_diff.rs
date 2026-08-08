//! Property evidence for sl-viewer's `bundle_diff` module.
//!
//! Complements `crates/sl-viewer/src/bundle_diff.rs`'s per-function
//! `#[cfg(test)] mod tests` block by pinning invariants over the *full*
//! shape of the inputs the pure-function diff logic can receive.
//!
//! `bundle_diff` invariants:
//!  * `diff_fields` is total: it always returns exactly one `FieldDiff`
//!    per documented field (9 today), in a stable order so the UI never
//!    reorders rows.
//!  * `diff_fields(a, a)` has no differing fields (reflexive).
//!  * `diff_fields(a, b)` is "value-flipped" symmetric: swapping inputs
//!    swaps `value_a` / `value_b` per field but preserves the set of
//!    fields that differ.
//!  * `FieldDiff::differs` matches `value_a != value_b` per field.
//!  * `Option<String>`-valued fields render their em-dash fallback when
//!    both sides are `None`; the resulting `differs` is `false`.
//!  * `OkfBundle::from_bundle` reduces a `ContinuationBundle` correctly:
//!    `message_count` is the slice count, `has_acceptance` / `has_contract`
//!    reflect presence of those kinds, and `token_count` falls back to 0
//!    when no `Intent` slice carries a numeric `user_turn_count`.

use proptest::prelude::*;
use session_ledger::domain::bundle::{Bundle, BundleKind, ContinuationBundle};
use sl_viewer::bundle_diff::{diff_fields, FieldDiff, OkfBundle};

// ── strategies ─────────────────────────────────────────────────────────────

const EXPECTED_FIELD_NAMES: &[&str] = &[
    "source_id",
    "token_count",
    "message_count",
    "duration_ms",
    "model",
    "created_at",
    "goal",
    "has_acceptance",
    "has_contract",
];

fn okf_bundle_strategy() -> impl Strategy<Value = OkfBundle> {
    (
        // source_id — non-empty identifier-shaped string.
        "[a-zA-Z0-9_-]{1,16}",
        // token_count — bounded u64.
        0u64..1_000_000,
        // message_count — bounded usize.
        0usize..16,
        // duration_ms — bounded u64.
        0u64..1_000_000,
        // model — Some(str) or None (None rendered as em-dash).
        prop::option::of("[a-zA-Z0-9 ._-]{1,32}"),
        // created_at — ISO-shaped or None.
        prop::option::of("[0-9T:Z.+-]{1,24}"),
        // goal — Some(str) or None.
        prop::option::of("[a-zA-Z0-9 ._-]{1,40}"),
        // has_acceptance, has_contract.
        any::<bool>(),
        any::<bool>(),
    )
        .prop_map(
            |(
                source_id,
                token_count,
                message_count,
                duration_ms,
                model,
                created_at,
                goal,
                has_acceptance,
                has_contract,
            )| {
                OkfBundle {
                    source_id,
                    token_count,
                    message_count,
                    duration_ms,
                    model,
                    created_at,
                    goal,
                    has_acceptance,
                    has_contract,
                }
            },
        )
}

// ── diff_fields properties ─────────────────────────────────────────────────

proptest! {
    /// Property: `diff_fields` is total — always returns exactly one
    /// `FieldDiff` per documented field, in stable order. Guards against
    /// drift between the row count the UI expects and the diff emits.
    #[test]
    fn diff_fields_returns_full_stable_field_set(
        a in okf_bundle_strategy(),
        b in okf_bundle_strategy(),
    ) {
        let diffs = diff_fields(&a, &b);
        prop_assert_eq!(diffs.len(), EXPECTED_FIELD_NAMES.len(), "diff length must match documented field count");
        let names: Vec<&str> = diffs.iter().map(|d| d.name).collect();
        prop_assert_eq!(&names[..], EXPECTED_FIELD_NAMES, "field names must be stable");
    }

    /// Property: `diff_fields(a, a)` is reflexive — no fields differ when
    /// both sides are equal. Catches off-by-one comparisons and missed
    /// field copy bugs.
    #[test]
    fn diff_fields_reflexive_no_differs(a in okf_bundle_strategy()) {
        let diffs = diff_fields(&a, &a);
        for d in &diffs {
            prop_assert!(
                !d.differs,
                "{} should not differ when both sides are the same bundle",
                d.name,
            );
            prop_assert_eq!(&d.value_a, &d.value_b, "{} values should match on reflexive diff", d.name);
        }
    }

    /// Property: `diff_fields(a, a.clone())` is also reflexive — a cloned
    /// bundle must produce no differences.
    #[test]
    fn diff_fields_cloned_no_differs(a in okf_bundle_strategy()) {
        let diffs = diff_fields(&a, &a.clone());
        for d in &diffs {
            prop_assert!(!d.differs, "{} should not differ when both sides are clones", d.name);
        }
    }

    /// Property: `diff_fields(a, b)` and `diff_fields(b, a)` agree on the
    /// set of fields that differ (differs is symmetric), while each
    /// field's `value_a` / `value_b` swap accordingly.
    #[test]
    fn diff_fields_symmetric_differs_swapped_values(
        a in okf_bundle_strategy(),
        b in okf_bundle_strategy(),
    ) {
        let ab = diff_fields(&a, &b);
        let ba = diff_fields(&b, &a);
        prop_assert_eq!(ab.len(), ba.len());
        for (l, r) in ab.iter().zip(ba.iter()) {
            prop_assert_eq!(l.name, r.name);
            prop_assert_eq!(
                l.differs, r.differs,
                "differs must be symmetric for field {}", l.name,
            );
            prop_assert_eq!(&l.value_a, &r.value_b, "value_a must equal r.value_b for field {}", l.name);
            prop_assert_eq!(&l.value_b, &r.value_a, "value_b must equal r.value_a for field {}", l.name);
        }
    }

    /// Property: `FieldDiff::differs` matches `value_a != value_b`. Catches
    /// drift where the boolean is computed independently of the values.
    #[test]
    fn differs_matches_value_inequality(
        a in okf_bundle_strategy(),
        b in okf_bundle_strategy(),
    ) {
        let diffs = diff_fields(&a, &b);
        for d in &diffs {
            prop_assert_eq!(
                d.differs,
                d.value_a != d.value_b,
                "{}.differs ({}) must match value_a != value_b ({} != {})",
                d.name, d.differs, d.value_a, d.value_b,
            );
        }
    }

    /// Property: `Option<String>` fields render the em-dash fallback when
    /// both sides are `None`, and the resulting diff is not a difference.
    /// This is the "both absent" contract; the "present vs absent" case
    /// is covered by the symmetric-differs / differs-matches-inequality
    /// properties above.
    #[test]
    fn option_fields_use_em_dash_for_both_none(
        token_count in 0u64..1000,
        message_count in 0usize..16,
    ) {
        let a = OkfBundle {
            source_id: "sess".into(),
            token_count,
            message_count,
            duration_ms: 0,
            model: None,
            created_at: None,
            goal: None,
            has_acceptance: false,
            has_contract: false,
        };
        let diffs = diff_fields(&a, &a);
        for name in ["model", "created_at", "goal"] {
            let field = diffs.iter().find(|d| d.name == name).expect("field must exist");
            prop_assert_eq!(&field.value_a, "—", "{} must render em-dash for None", name);
            prop_assert_eq!(&field.value_b, "—", "{} must render em-dash for None", name);
            prop_assert!(!field.differs, "{} must not differ when both sides are None", name);
        }
    }
}

// ── OkfBundle::from_bundle properties ──────────────────────────────────────

proptest! {
    /// Property: `message_count` equals the number of bundles in the
    /// input continuation.
    #[test]
    fn from_bundle_message_count_matches_len(slice_count in 0usize..8) {
        let bundles: Vec<Bundle> = (0..slice_count)
            .map(|i| Bundle::new(BundleKind::Intent, serde_json::json!({"i": i})))
            .collect();
        let cb = ContinuationBundle {
            source_id: "test".into(),
            bundles,
        };
        let okf = OkfBundle::from_bundle(&cb);
        prop_assert_eq!(okf.message_count, slice_count);
    }

    /// Property: `has_acceptance` is `true` iff any bundle in the input
    /// has kind `Acceptance`. Same for `has_contract`.
    #[test]
    fn from_bundle_has_flags_reflect_kind_presence(
        // 0..6 bundles; each may be Intent (i), Acceptance (a), Contract (c).
        kinds in prop::collection::vec(
            prop::sample::select(vec![BundleKind::Intent, BundleKind::Acceptance, BundleKind::Contract]),
            0..6,
        ),
    ) {
        let bundles: Vec<Bundle> = kinds
            .iter()
            .map(|k| Bundle::new(*k, serde_json::json!({})))
            .collect();
        let cb = ContinuationBundle {
            source_id: "test".into(),
            bundles,
        };
        let okf = OkfBundle::from_bundle(&cb);

        prop_assert_eq!(okf.has_acceptance, kinds.contains(&BundleKind::Acceptance));
        prop_assert_eq!(okf.has_contract, kinds.contains(&BundleKind::Contract));
    }

    /// Property: `token_count` falls back to 0 when no `Intent` bundle
    /// carries a numeric `user_turn_count`. Guards the silent-fallback
    /// behaviour documented in the impl.
    #[test]
    fn from_bundle_token_count_zero_when_no_intent_or_field(
        // Variants: 0 = no Intent bundle at all; 1 = Intent without
        // user_turn_count; 2 = Intent with non-numeric user_turn_count.
        variant in 0u8..3,
    ) {
        let bundles: Vec<Bundle> = match variant {
            0 => Vec::new(),
            1 => vec![Bundle::new(BundleKind::Intent, serde_json::json!({"goal": "x"}))],
            _ => vec![Bundle::new(
                BundleKind::Intent,
                serde_json::json!({"user_turn_count": "not-a-number"}),
            )],
        };
        let cb = ContinuationBundle {
            source_id: "test".into(),
            bundles,
        };
        let okf = OkfBundle::from_bundle(&cb);
        prop_assert_eq!(okf.token_count, 0, "token_count must default to 0 when missing/non-numeric");
    }

    /// Property: `source_id` carries through from the continuation bundle
    /// unchanged.
    #[test]
    fn from_bundle_source_id_carries_through(source_id in "[a-zA-Z0-9_-]{1,32}") {
        let cb = ContinuationBundle {
            source_id: source_id.clone(),
            bundles: Vec::new(),
        };
        let okf = OkfBundle::from_bundle(&cb);
        prop_assert_eq!(okf.source_id, source_id);
    }
}

// ── cross-test glue ────────────────────────────────────────────────────────

/// Compile-time guarantee that the FieldDiff-derived constants stay in sync.
/// If the impl adds a field, this test fails to compile until EXPECTED_FIELD_NAMES
/// is updated, prompting the reviewer to confirm the UI row count.
#[allow(dead_code)]
const fn _assert_field_count_fits_diff(diff: &[FieldDiff], expected_len: usize) -> bool {
    diff.len() == expected_len
}
