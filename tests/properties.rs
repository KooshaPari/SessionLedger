//! Property evidence for the deterministic dedup, merge, and OKF contracts.

use std::collections::HashSet;

use proptest::prelude::*;
use session_ledger::{
    process_session, Corpus, DedupKey, MergeExecutor, Message, OkfDocument, Role, Session,
};

fn session_with_messages(
    id: &str,
    corpus: Corpus,
    cwd: &str,
    messages: Vec<(String, Option<i64>)>,
) -> Session {
    let mut session = Session::new(id, corpus);
    session.cwd = Some(cwd.to_owned());
    session.messages = messages
        .into_iter()
        .map(|(content, ts_ms)| Message { role: Role::User, content, ts_ms })
        .collect();
    session
}

fn role(index: u8) -> Role {
    match index % 5 {
        0 => Role::User,
        1 => Role::Assistant,
        2 => Role::Subagent,
        3 => Role::Tool,
        _ => Role::System,
    }
}

proptest! {
    #[test]
    fn dedup_key_is_stable_under_documented_normalization(
        cwd in "[a-zA-Z0-9_./-]{1,64}",
        topic in "[a-zA-Z0-9_-]{1,48}",
        left_id in "[a-zA-Z0-9_-]{1,24}",
        right_id in "[a-zA-Z0-9_-]{1,24}",
    ) {
        let mut left = Session::new(left_id, Corpus::Forge);
        left.cwd = Some(cwd.clone());

        let mut right = Session::new(right_id, Corpus::Cursor);
        right.cwd = Some(format!(" \t{}\n", cwd.to_ascii_uppercase()));

        let canonical = DedupKey::derive(&left, &topic);
        let normalized = DedupKey::derive(
            &right,
            &format!(" \t{}\n", topic.to_ascii_uppercase()),
        );

        prop_assert_eq!(&canonical, &normalized);
        prop_assert_eq!(canonical.as_str().len(), 64);
        prop_assert!(
            canonical
                .as_str()
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        );
    }

    #[test]
    fn merge_is_permutation_invariant(
        topic in "[a-zA-Z0-9_-]{1,48}",
        alpha_messages in prop::collection::vec(
            ("[ -~]{0,80}", prop::option::of(any::<i64>())),
            0..12,
        ),
        beta_messages in prop::collection::vec(
            ("[ -~]{0,80}", prop::option::of(any::<i64>())),
            0..12,
        ),
    ) {
        let alpha = session_with_messages(
            "alpha",
            Corpus::Forge,
            "/same/scope",
            alpha_messages,
        );
        let beta = session_with_messages(
            "beta",
            Corpus::Cursor,
            "/same/scope",
            beta_messages,
        );

        let forward = MergeExecutor::default()
            .merge(&[alpha.clone(), beta.clone()], &topic)
            .expect("same-scope sessions merge");
        let reversed = MergeExecutor::default()
            .merge(&[beta, alpha], &topic)
            .expect("input permutation also merges");

        prop_assert_eq!(forward, reversed);
    }

    #[test]
    fn merge_is_idempotent_for_duplicate_members(
        topic in "[a-zA-Z0-9_-]{1,48}",
        messages in prop::collection::vec(
            ("[ -~]{0,80}", prop::option::of(any::<i64>())),
            0..12,
        ),
    ) {
        let source = session_with_messages(
            "source",
            Corpus::Codex,
            "/same/scope",
            messages,
        );

        let once = MergeExecutor::default()
            .merge(std::slice::from_ref(&source), &topic)
            .expect("single member merges");
        let duplicated = MergeExecutor::default()
            .merge(&[source.clone(), source], &topic)
            .expect("duplicate member merges");

        prop_assert_eq!(once, duplicated);
    }

    #[test]
    fn okf_json_roundtrip_preserves_structural_invariants(
        id in "[a-zA-Z0-9_-]{1,24}",
        cwd in prop::option::of("[a-zA-Z0-9_./ -]{0,64}"),
        title in prop::option::of("[ -~]{0,80}"),
        messages in prop::collection::vec(
            (
                0u8..5,
                "[ -~]{0,120}",
                prop::option::of(any::<i64>()),
            ),
            0..20,
        ),
    ) {
        let mut session = Session::new(id, Corpus::ClaudeCode);
        session.cwd = cwd;
        session.title = title;
        session.messages = messages
            .into_iter()
            .map(|(role_index, content, ts_ms)| Message {
                role: role(role_index),
                content,
                ts_ms,
            })
            .collect();

        let document = process_session(&session);
        let encoded = serde_json::to_vec(&document).expect("OKF serializes");
        let decoded: OkfDocument = serde_json::from_slice(&encoded).expect("OKF deserializes");

        prop_assert_eq!(&document, &decoded);
        prop_assert_eq!(&document.okf, "1.0");
        prop_assert_eq!(&document.source_id, &document.provenance.source_id);

        let entity_ids = document
            .entities
            .iter()
            .map(|entity| entity.id.as_str())
            .collect::<HashSet<_>>();
        prop_assert_eq!(entity_ids.len(), document.entities.len());
        for relation in &document.relations {
            prop_assert!(entity_ids.contains(relation.source.as_str()));
            prop_assert!(entity_ids.contains(relation.target.as_str()));
        }
    }
}
