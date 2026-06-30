//! Codex corpus adapter (Phase 2). See `docs/DESIGN.md` for on-disk shape.

use crate::domain::session::Corpus;

/// Marker for the codex ingestion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct CodexAdapter;

impl CodexAdapter {
    #[must_use]
    pub fn corpus(self) -> Corpus {
        Corpus::Codex
    }
}
