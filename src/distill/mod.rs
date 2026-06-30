//! Distillation ("dream") — compiles a [`Session`] into a [`ContinuationBundle`]
//! and emits distilled facts into the [`MemoryStore`].
//!
//! Phase 1 ships the deterministic compiler skeleton: it assembles the bundle
//! envelope (Acceptance + Intent + Context + Provenance + Worklog) from a
//! session. LLM-backed intent extraction and memory write-through arrive in
//! Phase 3 behind the [`crate::ports`] traits.
//!
//! The P1 [`extractor`] module provides the heuristic intent extractor adapter.
//! The P2 [`context_extractor`] and [`contract_extractor`] modules add heuristic
//! context and contract extraction adapters.
//! The P3 [`acceptance_extractor`] and [`compiler`] modules add the acceptance
//! extractor and the bundle compiler.

pub mod acceptance_extractor;
pub mod compiler;
pub mod context_extractor;
pub mod contract_extractor;
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

    // Use the heuristic extractors to produce structured data.
    let intent = extractor::HeuristicIntentExtractor::extract_intent(session);
    let context = context_extractor::HeuristicContextExtractor::extract_context(session);
    let contract = contract_extractor::HeuristicContractExtractor::extract_contract(session);

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
        json!({
            "cwd": context.cwd,
            "title": context.title,
            "files_mentioned": context.files_mentioned,
            "key_decisions": context.key_decisions,
            "key_symbols": context.key_symbols,
            "environment_notes": context.environment_notes,
        }),
    ));
    bundle.push(Bundle::new(
        BundleKind::Contract,
        json!({
            "success_criteria": contract.success_criteria,
            "tests_or_verifications": contract.tests_or_verifications,
            "constraints": contract.constraints,
            "do_not_touch": contract.do_not_touch,
        }),
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
