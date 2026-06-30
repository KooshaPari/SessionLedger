//! Port traits — the boundary the domain depends on. Adapters (implemented in a
//! later phase) wire these to existing Phenotype systems; see `docs/DESIGN.md`:
//!
//! - [`CorpusSource`]  ← forgecode `ConversationRepository`, codex/claude JSONL.
//! - [`MemoryStore`]   ← `OmniRoute` `src/lib/memory` (FTS5 + Qdrant, hybrid recall).
//! - [`Compressor`]    ← forgecode zstd codec / omni-context-rtk.
//! - [`TraceSink`]     ← pheno-tracing `TracePort`.
//! - [`OkfExporter`]   ← OKF (Open Knowledge Format) entity/relation export.

use crate::domain::context::Context;
use crate::domain::contract::Contract;
use crate::domain::intent::Intent;
use crate::domain::session::Session;

/// Error surface for port operations. Adapters map their native errors into this.
#[derive(Debug, thiserror::Error)]
pub enum PortError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("backend error: {0}")]
    Backend(String),
}

/// A read source of raw sessions (one per corpus).
pub trait CorpusSource {
    /// List session ids available in this source.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the underlying corpus cannot be read.
    fn list(&self) -> Result<Vec<String>, PortError>;
    /// Load and normalize a single session.
    ///
    /// # Errors
    /// Returns [`PortError::NotFound`] if `id` is unknown, or
    /// [`PortError::Backend`] on a read/parse failure.
    fn load(&self, id: &str) -> Result<Session, PortError>;
}

/// Long-term distilled memory. Backed by `OmniRoute` memory (FTS5 + vector).
pub trait MemoryStore {
    /// Persist a distilled fact; returns the memory id (forgecode's `memory_id`).
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the write fails.
    fn store(&self, session_id: &str, key: &str, content: &str) -> Result<String, PortError>;
    /// Hybrid recall (FTS + vector) for continuation context assembly.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the query fails.
    fn recall(&self, query: &str, top_k: usize) -> Result<Vec<String>, PortError>;
}

/// Reversible compression for stored context (zstd in forgecode).
pub trait Compressor {
    /// Compress `data`.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if compression fails.
    fn compress(&self, data: &str) -> Result<Vec<u8>, PortError>;
    /// Decompress previously [`Compressor::compress`]ed bytes.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the bytes are not valid compressed data.
    fn decompress(&self, data: &[u8]) -> Result<String, PortError>;
}

/// Observability sink for ledger operations (pheno-tracing).
pub trait TraceSink {
    fn span(&self, name: &str);
}

/// Structured intent extraction from a session's message stream.
///
/// Parses session messages and extracts the user's goal, acceptance signals,
/// and constraints into a structured [`Intent`] value.
///
/// # Errors
/// Returns [`PortError::Backend`] if extraction fails (adapter-level failure).
pub trait IntentExtractor {
    /// Extract structured intent from a normalized session.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the underlying extraction fails.
    fn extract(&self, session: &Session) -> Result<Intent, PortError>;
}

/// Structured context extraction from a session's message stream.
///
/// Parses session messages and extracts the working context — files touched,
/// decisions reached, symbols referenced, and environment state — into a
/// structured [`Context`] value.
///
/// # Errors
/// Returns [`PortError::Backend`] if extraction fails (adapter-level failure).
pub trait ContextExtractor {
    /// Extract structured working-context from a normalized session.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the underlying extraction fails.
    fn extract(&self, session: &Session) -> Result<Context, PortError>;
}

/// Structured contract extraction from a session's message stream.
///
/// Parses session messages and extracts acceptance criteria — done-conditions,
/// test commands, invariants, and do-not-touch rules — into a structured
/// [`Contract`] value.
///
/// # Errors
/// Returns [`PortError::Backend`] if extraction fails (adapter-level failure).
pub trait ContractExtractor {
    /// Extract structured acceptance contract from a normalized session.
    ///
    /// # Errors
    /// Returns [`PortError::Backend`] if the underlying extraction fails.
    fn extract(&self, session: &Session) -> Result<Contract, PortError>;
}

/// OKF (Open Knowledge Format) data model and export port.
pub mod okf;
