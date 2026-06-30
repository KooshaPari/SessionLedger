//! Contract bundle model — the concrete acceptance criteria the work must satisfy.
//!
//! Extracted from a session's message stream: success criteria, test commands,
//! constraints, and do-not-touch rules that a continuation must honor.

use serde::{Deserialize, Serialize};

/// Structured acceptance contract extracted from a session's messages.
///
/// This is the output of the [`ContractExtractor`](crate::ports::ContractExtractor)
/// port: the concrete done-criteria, verifications, invariants, and boundaries
/// that define what "done" means and what the continuation must not break.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Explicit success criteria / acceptance conditions (e.g. "login flow works").
    pub success_criteria: Vec<String>,
    /// Test commands or verification steps (e.g. "cargo test", "npm run check").
    pub tests_or_verifications: Vec<String>,
    /// Invariants / constraints the continuation must honor.
    pub constraints: Vec<String>,
    /// Files, modules, or sections that must not be modified.
    pub do_not_touch: Vec<String>,
}

impl Contract {
    /// Empty contract — nothing extracted yet.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            success_criteria: Vec::new(),
            tests_or_verifications: Vec::new(),
            constraints: Vec::new(),
            do_not_touch: Vec::new(),
        }
    }

    /// Whether the extractor found any meaningful contract data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.success_criteria.is_empty()
            && self.tests_or_verifications.is_empty()
            && self.constraints.is_empty()
            && self.do_not_touch.is_empty()
    }
}
