//! Ingestion adapters — one per corpus. Each normalizes a raw transcript into a
//! [`crate::domain::session::Session`] and implements [`crate::ports::CorpusSource`].
//!
//! Phase 1 ships the corpus enumeration + the forge adapter contract; the
//! remaining adapters (codex/claude-code/cursor) follow the same trait. See
//! `docs/DESIGN.md` for each corpus's on-disk shape.

pub mod claude_code;
pub mod codex;
pub mod cursor;
pub mod forge;
