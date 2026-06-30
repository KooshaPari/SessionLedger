//! OKF (Open Knowledge Format) port — entity/relation/provenance model.
//!
//! # OKF mapping
//!
//! Open Knowledge Format models knowledge as a directed graph of typed
//! [`OkfEntity`] nodes connected by [`OkfRelation`] edges, each carrying a
//! [`OkfProvenance`] record that traces back to the source session.
//!
//! ## Bundle → OKF mapping
//!
//! | Bundle kind        | OKF entity type       | OKF entity label                        |
//! |--------------------|-----------------------|-----------------------------------------|
//! | `Intent` (goal)    | `intent`              | The user's goal string                  |
//! | `Intent` (accept.) | `acceptance`          | Each acceptance signal                  |
//! | `Intent` (constr.) | `constraint`          | Each constraint string                  |
//! | `Context`          | `resource` / `state`  | cwd, title properties                   |
//! | `Contract`         | `criteria`            | Named success criterion                 |
//! | `Acceptance`       | `gate`                | "resume-gate" label                     |
//! | `Provenance`       | provenance edge       | Carried as relation provenance          |
//!
//! ## Example OKF document
//!
//! ```json
//! {
//!   "okf": "1.0",
//!   "source_id": "sess-abc",
//!   "entities": [
//!     { "id": "intent-0", "type": "intent",
//!       "label": "fix the pagination bug",
//!       "properties": { "user_turn_count": 3 } },
//!     { "id": "acceptance-0", "type": "acceptance",
//!       "label": "looks good" },
//!     { "id": "constraint-0", "type": "constraint",
//!       "label": "don't change the database schema" },
//!     { "id": "resource-0", "type": "resource",
//!       "label": "working-directory",
//!       "properties": { "cwd": "/home/user/proj" } },
//!     { "id": "gate-0", "type": "gate",
//!       "label": "resume-gate",
//!       "properties": { "ready": true, "scope_sized": true } }
//!   ],
//!   "relations": [
//!     { "source": "intent-0", "target": "acceptance-0",
//!       "type": "verified_by",
//!       "provenance": { "corpus": "forge", "source_id": "sess-abc" } },
//!     { "source": "intent-0", "target": "constraint-0",
//!       "type": "bounded_by",
//!       "provenance": { "corpus": "forge", "source_id": "sess-abc" } }
//!   ],
//!   "provenance": {
//!     "corpus": "forge",
//!     "source_id": "sess-abc"
//!   }
//! }
//! ```
//!
//! The OKF version string `"1.0"` identifies this dialect. Consumers that
//! encounter a newer major version SHOULD reject the document or fall back
//! gracefully.

use crate::domain::bundle::ContinuationBundle;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// OKF data model
// ---------------------------------------------------------------------------

/// A single knowledge entity (typed node) in the OKF graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OkfEntity {
    /// Unique id within the document (e.g. `"intent-0"`, `"resource-1"`).
    pub id: String,
    /// Entity type — mirrors the source [`BundleKind`](crate::domain::bundle::BundleKind) role.
    pub r#type: String,
    /// Human-readable label for the entity.
    pub label: String,
    /// Optional structured properties.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub properties: serde_json::Value,
}

/// A typed relation between two entities in the OKF graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OkfRelation {
    /// Source entity id.
    pub source: String,
    /// Target entity id.
    pub target: String,
    /// Relationship type (e.g. `"verified_by"`, `"bounded_by"`, `"grounds"`).
    pub r#type: String,
    /// Provenance for this relation.
    pub provenance: OkfProvenance,
}

/// Provenance metadata tracing an entity or relation back to its origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OkfProvenance {
    /// Source corpus (forge, codex, claude-code, cursor).
    pub corpus: String,
    /// Source session id.
    pub source_id: String,
}

/// A complete OKF document.
///
/// This is the top-level container: a knowledge graph with entities, relations,
/// and document-level provenance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OkfDocument {
    /// Format version identifier (`"1.0"`).
    pub okf: String,
    /// Source session id.
    pub source_id: String,
    /// Knowledge entities (nodes).
    pub entities: Vec<OkfEntity>,
    /// Typed relations between entities (edges).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<OkfRelation>,
    /// Document-level provenance.
    pub provenance: OkfProvenance,
}

impl OkfDocument {
    /// Create a bare OKF document with provenance derived from `bundle`.
    #[must_use]
    pub fn new(bundle: &ContinuationBundle, corpus: &str) -> Self {
        Self {
            okf: "1.0".into(),
            source_id: bundle.source_id.clone(),
            entities: Vec::new(),
            relations: Vec::new(),
            provenance: OkfProvenance {
                corpus: corpus.into(),
                source_id: bundle.source_id.clone(),
            },
        }
    }

    /// Serialize to pretty-printed JSON.
    ///
    /// # Errors
    /// Returns [`serde_json::Error`] if the writer fails.
    pub fn to_json_pretty<W: std::io::Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(writer, self)
    }
}

// ---------------------------------------------------------------------------
// Port trait
// ---------------------------------------------------------------------------

/// Port: OKF exporter.
///
/// Converts a compiled [`ContinuationBundle`] into the Open Knowledge Format
/// (entities + relations + provenance). Implementations MAY target JSON, YAML,
/// or any other concrete serialization.
///
/// # Errors
///
/// Returns [`super::PortError::Backend`] if serialization fails.
pub trait OkfExporter {
    /// The concrete output type (e.g. `String`, `serde_json::Value`).
    type Output;

    /// Export a continuation bundle into the OKF representation.
    ///
    /// # Errors
    ///
    /// Returns [`super::PortError::Backend`] if the export cannot be produced.
    fn export(&self, bundle: &ContinuationBundle) -> Result<Self::Output, super::PortError>;
}
