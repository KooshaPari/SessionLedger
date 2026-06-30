//! Context bundle model — the working state a resumed session needs to know.
//!
//! Extracted from a session's message stream: what files were touched, what
//! decisions were reached, what symbols/types are load-bearing, and what
//! environment state was established.

use serde::{Deserialize, Serialize};

/// A single extracted decision with optional rationale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decision {
    /// Short summary (e.g. "use Axum over Actix").
    pub summary: String,
    /// Context that motivated the choice, when discernible.
    pub rationale: Option<String>,
}

/// Structured working-context extracted from a session's messages.
///
/// This is the output of the [`ContextExtractor`](crate::ports::ContextExtractor)
/// port: the distilled files, decisions, symbols, and environment that a
/// continuation needs to act without re-reading the entire transcript.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Context {
    /// Working directory / project root, when known.
    pub cwd: Option<String>,
    /// Session title or topic, when known.
    pub title: Option<String>,
    /// File paths explicitly mentioned in session messages.
    pub files_mentioned: Vec<String>,
    /// Key technical decisions reached during the session.
    pub key_decisions: Vec<Decision>,
    /// Important symbols / types / identifiers referenced.
    pub key_symbols: Vec<String>,
    /// Environment notes (dependencies installed, config changed, etc.).
    pub environment_notes: Vec<String>,
}

impl Context {
    /// Empty context — nothing extracted yet.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            cwd: None,
            title: None,
            files_mentioned: Vec::new(),
            key_decisions: Vec::new(),
            key_symbols: Vec::new(),
            environment_notes: Vec::new(),
        }
    }

    /// Whether the extractor found any meaningful context data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cwd.is_none()
            && self.title.is_none()
            && self.files_mentioned.is_empty()
            && self.key_decisions.is_empty()
            && self.key_symbols.is_empty()
            && self.environment_notes.is_empty()
    }
}
