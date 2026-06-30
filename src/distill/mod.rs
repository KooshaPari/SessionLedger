//! Distillation ("dream") — compiles a [`Session`] into a [`ContinuationBundle`]
//! and emits distilled facts into the [`MemoryStore`].
//!
//! Phase 1 ships the deterministic compiler skeleton: it assembles the bundle
//! envelope (Acceptance + Intent + Context + Provenance + Worklog) from a
//! session. LLM-backed intent extraction and memory write-through arrive in
//! Phase 3 behind the [`crate::ports`] traits.
//!
//! The P1 [`extractor`] module provides the heuristic intent extractor adapter.

pub mod extractor;

use crate::domain::bundle::{Bundle, BundleKind, ContinuationBundle};
use crate::domain::session::Session;
use serde_json::json;

/// Compile a normalized session into a continuation bundle.
///
/// The resulting bundle is injectable ([`ContinuationBundle::is_injectable`])
/// because it always carries an `Acceptance` slice.
#[must_use]
pub fn compile(session: &Session) -> ContinuationBundle {
    let mut bundle = ContinuationBundle::new(session.id.clone());

    let user_turns = session.user_turns();

    // Use the heuristic extractor to produce structured intent data.
    let intent = extractor::HeuristicIntentExtractor::extract_intent(session);

    bundle.push(Bundle::new(
        BundleKind::Acceptance,
        json!({
            "ready": !session.messages.is_empty(),
            "scope_sized": true,
            "user_turns": user_turns,
        }),
    ));
    bundle.push(Bundle::new(
        BundleKind::Intent,
        json!({
            "goal": intent.goal,
            "acceptance_signals": intent.acceptance_signals,
            "constraints": intent.constraints,
            "user_turn_count": intent.user_turn_count,
        }),
    ));
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
