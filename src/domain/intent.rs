//! Intent lifecycle FSM and structured intent model.
//!
//! Mirrors forgecode's forward-only intent DAG so `SessionLedger` and forgecode
//! agree on lifecycle semantics (forgecode is both an ingestion source and the
//! proven blueprint — see `docs/DESIGN.md`). The `SessionLedger` ledger acts as
//! the missing `IntentExtractor` forgecode currently stubs with `NoopIntentExtractor`.
//!
//! In addition to the FSM, this module defines the structured [`Intent`] type —
//! the output of the [`IntentExtractor`](crate::ports::IntentExtractor) port —
//! which captures the user's goal, acceptance signals, and constraints.

use serde::{Deserialize, Serialize};

/// Structured intent extracted from a session's messages.
///
/// This is the output of the [`IntentExtractor`](crate::ports::IntentExtractor)
/// port: what the user is trying to achieve, how they will know it's done, and
/// what boundaries the continuation must respect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// The user's primary goal or objective (e.g. "fix the login bug").
    pub goal: Option<String>,
    /// Signals the user uses to confirm success (e.g. "tests pass", "looks good").
    pub acceptance_signals: Vec<String>,
    /// Constraints / boundaries the continuation must honor (e.g. "don't touch auth").
    pub constraints: Vec<String>,
    /// Number of user turns contributing to this intent.
    pub user_turn_count: usize,
}

impl Intent {
    /// Empty intent — nothing extracted yet.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            goal: None,
            acceptance_signals: Vec::new(),
            constraints: Vec::new(),
            user_turn_count: 0,
        }
    }

    /// Whether the extractor found any meaningful intent data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.goal.is_none() && self.acceptance_signals.is_empty() && self.constraints.is_empty()
    }
}

/// Forward-only intent extraction lifecycle.
///
/// `Pending → Extracting → Extracted → Verified → Pruned`. A revert to
/// `Pending` is allowed (re-extraction); `Pruned` is terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntentState {
    Pending,
    Extracting,
    Extracted,
    Verified,
    Pruned,
}

impl IntentState {
    /// Whether a transition `self → next` is permitted by the FSM.
    #[must_use]
    pub fn can_transition(self, next: IntentState) -> bool {
        use IntentState::{Extracted, Extracting, Pending, Pruned, Verified};
        matches!(
            (self, next),
            (Extracting | Extracted | Verified, Pending)
                | (Pending, Extracting)
                | (Extracting, Extracted)
                | (Extracted, Verified)
                | (Verified, Pruned)
        )
    }

    /// `Pruned` is terminal — no further transitions.
    #[must_use]
    pub fn is_terminal(self) -> bool {
        matches!(self, IntentState::Pruned)
    }

    /// Pruning is intent-gated (ADR-103): only verified intent may be pruned.
    #[must_use]
    pub fn is_prune_eligible(self) -> bool {
        matches!(self, IntentState::Verified)
    }
}
