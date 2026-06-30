//! Heuristic intent extraction — the P1 adapter for [`IntentExtractor`].
//!
//! This adapter uses lightweight string matching to extract the user's goal,
//! acceptance signals, and constraints from a session's message stream. It is
//! the P1 replacement for forgecode's `NoopIntentExtractor`.
//!
//! Phase 3 will supersede this with an LLM-backed extractor behind the same
//! [`IntentExtractor`] trait — see `docs/DESIGN.md` §7.

use crate::domain::intent::Intent;
use crate::domain::session::{Role, Session};
use crate::ports::{IntentExtractor, PortError};

/// Heuristic-based intent extractor.
///
/// Uses lightweight text heuristics on user messages to infer:
/// - **Goal**: the first substantial user message (or latest if more specific).
/// - **Acceptance signals**: phrases like "looks good", "works", "done", "fixed".
/// - **Constraints**: phrases like "don't change", "must not", "keep".
///
/// This is intentionally simple. The LLM-backed extractor (Phase 3) will
/// replace it behind the same trait.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeuristicIntentExtractor;

// Heuristic patterns for acceptance signals (lowercased for matching).
const ACCEPTANCE_PATTERNS: &[&str] = &[
    "looks good",
    "works",
    "that's correct",
    "correct",
    "done",
    "fixed",
    "passes",
    "approved",
    "looks right",
    "looks great",
    "all good",
    "that works",
    "nice",
    "perfect",
    "exactly",
    "confirmed",
];

// Heuristic patterns for constraints (lowercased for matching).
const CONSTRAINT_PATTERNS: &[&str] = &[
    "don't change",
    "do not change",
    "must not",
    "should not",
    "keep",
    "maintain",
    "preserve",
    "never",
    "don't touch",
    "do not touch",
    "don't modify",
    "do not modify",
    "only",
    "but don't",
    "but do not",
    "without changing",
    "without modifying",
    "leave alone",
    "leave as is",
];

impl HeuristicIntentExtractor {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Heuristically extract intent from a session's messages.
    ///
    /// This is factored as a public associated function so it can be used
    /// directly by the compiler without going through the trait when no
    /// adapter injection is needed (P1 default).
    #[must_use]
    pub fn extract_intent(session: &Session) -> Intent {
        let user_messages: Vec<&str> = session
            .messages
            .iter()
            .filter(|m| m.role == Role::User)
            .map(|m| m.content.as_str())
            .collect();

        let user_turn_count = user_messages.len();

        // Goal: first substantial user message (>20 chars) or the very first one.
        let goal = user_messages
            .iter()
            .find(|msg| msg.len() > 20)
            .or_else(|| user_messages.first())
            .map(|msg| (*msg).to_string());

        // Acceptance signals: collect matched patterns found in user messages.
        let mut acceptance_signals: Vec<String> = Vec::new();
        // Constraints: collect matched patterns found in user messages.
        let mut constraints: Vec<String> = Vec::new();

        for msg in &user_messages {
            let lower = msg.to_lowercase();
            for pat in ACCEPTANCE_PATTERNS {
                if lower.contains(pat) {
                    let signal = pat.to_string();
                    if !acceptance_signals.contains(&signal) {
                        acceptance_signals.push(signal);
                    }
                }
            }
            for pat in CONSTRAINT_PATTERNS {
                if lower.contains(pat) {
                    let constraint = pat.to_string();
                    if !constraints.contains(&constraint) {
                        constraints.push(constraint);
                    }
                }
            }
        }

        Intent { goal, acceptance_signals, constraints, user_turn_count }
    }
}

impl IntentExtractor for HeuristicIntentExtractor {
    fn extract(&self, session: &Session) -> Result<Intent, PortError> {
        Ok(Self::extract_intent(session))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::session::{Corpus, Message, Role};

    fn fixture_session(user_msgs: &[&str]) -> Session {
        let mut s = Session::new("test-sess", Corpus::Forge);
        for msg in user_msgs {
            s.messages.push(Message::new(Role::User, *msg));
        }
        s
    }

    fn fixture_session_with_assistant(user_msgs: &[&str], assistant_msgs: &[&str]) -> Session {
        let mut s = Session::new("test-sess-2", Corpus::Forge);
        for msg in user_msgs {
            s.messages.push(Message::new(Role::User, *msg));
        }
        for msg in assistant_msgs {
            s.messages.push(Message::new(Role::Assistant, *msg));
        }
        s
    }

    #[test]
    fn extracts_goal_from_first_substantial_user_message() {
        let session = fixture_session(&["hi", "can you fix the login bug in the auth module"]);
        let intent = HeuristicIntentExtractor::extract_intent(&session);
        let expected = Intent {
            goal: Some("can you fix the login bug in the auth module".to_string()),
            acceptance_signals: vec![],
            constraints: vec![],
            user_turn_count: 2,
        };
        assert_eq!(intent, expected);
    }

    #[test]
    fn extracts_acceptance_signals_and_constraints() {
        let session = fixture_session(&[
            "fix the pagination bug but don't change the database schema",
            "looks good, the tests pass now",
        ]);
        let intent = HeuristicIntentExtractor::extract_intent(&session);
        assert_eq!(
            intent.goal.as_deref(),
            Some("fix the pagination bug but don't change the database schema")
        );
        assert!(intent.acceptance_signals.contains(&"looks good".to_string()));
        assert_eq!(intent.acceptance_signals.len(), 1);
        assert!(intent.constraints.contains(&"don't change".to_string()));
        assert_eq!(intent.user_turn_count, 2);
    }

    #[test]
    fn returns_empty_intent_for_no_user_messages() {
        let session = fixture_session(&[]);
        let intent = HeuristicIntentExtractor::extract_intent(&session);
        assert!(intent.is_empty());
        assert_eq!(intent.user_turn_count, 0);
    }

    #[test]
    fn ignores_assistant_messages_when_extracting_intent() {
        let session = fixture_session_with_assistant(
            &["add a rate limiter to the api"],
            &["sure, let me implement that", "here is the implementation"],
        );
        let intent = HeuristicIntentExtractor::extract_intent(&session);
        assert_eq!(intent.goal.as_deref(), Some("add a rate limiter to the api"));
        assert_eq!(intent.user_turn_count, 1);
    }

    #[test]
    fn deduplicates_repeated_acceptance_signal() {
        let session = fixture_session(&["looks good so far", "looks good, ship it"]);
        let intent = HeuristicIntentExtractor::extract_intent(&session);
        assert_eq!(intent.acceptance_signals.len(), 1);
        assert!(intent.acceptance_signals.contains(&"looks good".to_string()));
    }

    #[test]
    fn intent_extractor_trait_works() {
        let extractor = HeuristicIntentExtractor::new();
        let session = fixture_session(&["refactor the database layer"]);
        let intent = extractor.extract(&session).expect("extraction should succeed");
        assert_eq!(intent.goal.as_deref(), Some("refactor the database layer"));
    }
}
