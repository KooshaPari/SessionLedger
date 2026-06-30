//! Intent lifecycle FSM.
//!
//! Mirrors forgecode's forward-only intent DAG so `SessionLedger` and forgecode
//! agree on lifecycle semantics (forgecode is both an ingestion source and the
//! proven blueprint — see `docs/DESIGN.md`). The `SessionLedger` ledger acts as
//! the missing `IntentExtractor` forgecode currently stubs with `NoopIntentExtractor`.

use serde::{Deserialize, Serialize};

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
