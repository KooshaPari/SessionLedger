//! Cursor corpus adapter (Phase 2). See `docs/DESIGN.md` for on-disk shape.

use crate::domain::session::Corpus;

/// Marker for the cursor ingestion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct CursorAdapter;

impl CursorAdapter {
    #[must_use]
    pub fn corpus(self) -> Corpus {
        Corpus::Cursor
    }
}
