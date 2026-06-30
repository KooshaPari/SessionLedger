//! `SessionLedger` — compile, distill, and resume agent sessions.
//!
//! Hexagonal architecture:
//! - [`domain`]: the bundle model (Acceptance / Contract / Context / Intent /
//!   Provenance / Worklog), session entities, dedup keys, and the continuation
//!   bundle. Pure logic, no I/O.
//! - [`ports`]: trait boundaries the domain depends on (memory store, trace sink,
//!   compression, corpus source). Implemented by adapters; composed from existing
//!   Phenotype systems (see `docs/DESIGN.md`).
//! - [`ingestion`]: per-corpus adapters (forge, codex, claude-code, cursor) that
//!   normalize raw transcripts into [`domain::session::Session`].
//! - [`distill`]: the "dream" pass — distills a session into memory stores and
//!   produces the [`domain::bundle::ContinuationBundle`].
//! - [`viewer`]: read model for the wiki / docs / history surface.
//!
//! Nothing here duplicates forgecode (lifecycle FSM + zstd + ADR-103 pruning),
//! `OmniRoute` memory (FTS5 + Qdrant), or pheno-tracing — those are composed via
//! [`ports`]. See `docs/DESIGN.md` for the composition map.

pub mod distill;
pub mod domain;
pub mod ingestion;
pub mod ports;
pub mod viewer;

pub use domain::bundle::{Bundle, BundleKind, ContinuationBundle};
pub use domain::context::{Context, Decision};
pub use domain::contract::Contract;
pub use domain::intent::{Intent, IntentState};
pub use domain::session::{Message, Role, Session};
