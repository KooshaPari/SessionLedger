//! Skeleton integration tests proving the domain compiles and behaves.

use session_ledger::distill;
use session_ledger::domain::dedup::DedupKey;
use session_ledger::domain::intent::IntentState;
use session_ledger::domain::session::Corpus;
use session_ledger::{Bundle, BundleKind, ContinuationBundle, Message, Role, Session};

fn sample_session() -> Session {
    let mut s = Session::new("sess-1", Corpus::Forge);
    s.cwd = Some("/Users/koosha/proj".into());
    s.title = Some("fix the thing".into());
    s.messages.push(Message::new(Role::User, "please fix the bug"));
    s.messages.push(Message::new(Role::Assistant, "on it"));
    s.messages.push(Message::new(Role::User, "also add a test"));
    s
}

#[test]
fn compiled_bundle_is_injectable_and_complete() {
    let s = sample_session();
    let bundle = distill::compile(&s);
    assert!(bundle.is_injectable(), "must carry an Acceptance gate");
    for kind in [
        BundleKind::Acceptance,
        BundleKind::Intent,
        BundleKind::Context,
        BundleKind::Provenance,
        BundleKind::Worklog,
    ] {
        assert!(bundle.has(kind), "missing bundle kind {kind:?}");
    }
    assert_eq!(s.user_turns(), 2);
}

#[test]
fn bundle_without_acceptance_is_not_injectable() {
    let mut b = ContinuationBundle::new("x");
    b.push(Bundle::new(BundleKind::Context, serde_json::json!({})));
    assert!(!b.is_injectable());
}

#[test]
fn intent_fsm_is_forward_only_and_prune_gated() {
    assert!(IntentState::Pending.can_transition(IntentState::Extracting));
    assert!(IntentState::Extracted.can_transition(IntentState::Verified));
    assert!(IntentState::Verified.can_transition(IntentState::Pruned));
    // illegal: cannot skip straight to pruned
    assert!(!IntentState::Pending.can_transition(IntentState::Pruned));
    // ADR-103: only verified intent prunes
    assert!(IntentState::Verified.is_prune_eligible());
    assert!(!IntentState::Extracted.is_prune_eligible());
    assert!(IntentState::Pruned.is_terminal());
}

#[test]
fn dedup_key_collapses_same_scope_and_topic() {
    let a = sample_session();
    let mut b = sample_session();
    b.id = "sess-2".into(); // different session, same scope+topic
    let ka = DedupKey::derive(&a, "fix-the-bug");
    let kb = DedupKey::derive(&b, "fix-the-bug");
    assert_eq!(ka, kb, "duplicate-scoped chats must share a dedup key");

    let kc = DedupKey::derive(&a, "unrelated-topic");
    assert_ne!(ka, kc);
}

#[test]
fn bundle_roundtrips_through_json() {
    let bundle = distill::compile(&sample_session());
    let s = serde_json::to_string(&bundle).expect("serialize");
    let back: ContinuationBundle = serde_json::from_str(&s).expect("deserialize");
    assert_eq!(bundle, back);
}
