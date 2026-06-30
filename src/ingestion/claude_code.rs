//! `ClaudeCode` corpus adapter (Phase 2). See `docs/DESIGN.md` for on-disk shape.

use crate::domain::session::Corpus;

/// Marker for the `claude_code` ingestion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct ClaudeCodeAdapter;

impl ClaudeCodeAdapter {
    #[must_use]
    pub fn corpus(self) -> Corpus {
        Corpus::ClaudeCode
    }
}
