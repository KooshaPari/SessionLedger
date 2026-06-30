//! Forge corpus adapter.
//!
//! Reads forgecode's `conversations` table (`~/forge/.forge.db`, ~12.9k rows)
//! via its `ConversationRepository`. Handles zstd `context_zstd` decompression
//! and user/subagent message classification (the curation pipeline at
//! `phenotype-org-audits/curation/forge/curate.py` is the shared intent core).
//!
//! Phase 1: type-level contract only; the rusqlite-backed reader lands with the
//! `sqlite` feature in Phase 2.

use crate::domain::session::Corpus;

/// Marker for the forge ingestion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct ForgeAdapter;

impl ForgeAdapter {
    #[must_use]
    pub fn corpus(self) -> Corpus {
        Corpus::Forge
    }
}
