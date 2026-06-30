//! Distillation ("dream") — compiles a [`Session`] into a [`ContinuationBundle`]
//! and emits distilled facts into the [`MemoryStore`].
//!
//! Phase 1 ships the deterministic compiler skeleton: it assembles the bundle
//! envelope (Acceptance + Intent + Context + Provenance + Worklog) from a
//! session. LLM-backed intent extraction and memory write-through arrive in
//! Phase 3 behind the [`crate::ports`] traits.

use crate::domain::bundle::{Bundle, BundleKind, ContinuationBundle};
use crate::domain::session::{Role, Session};
use serde_json::json;

/// Compile a normalized session into a continuation bundle.
///
/// The resulting bundle is injectable ([`ContinuationBundle::is_injectable`])
/// because it always carries an `Acceptance` slice.
#[must_use]
pub fn compile(session: &Session) -> ContinuationBundle {
    let mut bundle = ContinuationBundle::new(session.id.clone());

    let user_turns = session.user_turns();
    let last_user =
        session.messages.iter().rev().find(|m| m.role == Role::User).map(|m| m.content.clone());

    bundle.push(Bundle::new(
        BundleKind::Acceptance,
        json!({
            "ready": !session.messages.is_empty(),
            "scope_sized": true,
            "user_turns": user_turns,
        }),
    ));
    bundle.push(Bundle::new(BundleKind::Intent, json!({ "latest_user_request": last_user })));
    bundle.push(Bundle::new(
        BundleKind::Context,
        json!({ "cwd": session.cwd, "title": session.title }),
    ));
    bundle.push(Bundle::new(
        BundleKind::Provenance,
        json!({ "corpus": session.corpus, "source_id": session.id }),
    ));
    bundle.push(Bundle::new(
        BundleKind::Worklog,
        json!({ "message_count": session.messages.len(), "unfinished": [] }),
    ));

    bundle
}
