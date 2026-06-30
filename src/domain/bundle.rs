//! The continuation bundle model.
//!
//! A [`ContinuationBundle`] is the canonical, injectable artifact for lossless
//! resume of an agent session in a NEW session. It is an ordered set of typed
//! [`Bundle`]s plus an `Acceptance` gate.
//!
//! Bundle kinds (owner's four + three proposed extensions, justified in
//! `docs/DESIGN.md`):
//! - `Acceptance`  — the resume gate: scope sized + ready-to-continue assertion.
//! - `Contract`    — invariants/constraints the continuation MUST honor.
//! - `Context`     — distilled state needed to act (files, decisions, environment).
//! - `Intent`      — the goal/intent DAG (what the operator is trying to achieve).
//! - `Provenance`  — where each fact came from (corpus, session id, message
//!   span) so a resumed session can cite/trust its inputs. *(proposed)*
//! - `Worklog`     — diff/worklog: what was done vs. still outstanding; feeds
//!   the in-progress / unfinished localization. *(proposed)*
//! - `Dedup`       — the merge key + merged-session manifest for collapsing
//!   duplicate-scoped chats into one resumable chat. *(proposed)*

use serde::{Deserialize, Serialize};

/// The kind of a [`Bundle`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BundleKind {
    Acceptance,
    Contract,
    Context,
    Intent,
    Provenance,
    Worklog,
    Dedup,
}

/// A single typed slice of a continuation bundle.
///
/// `body` is opaque structured JSON so each kind can evolve its own schema
/// without a breaking change to the envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bundle {
    pub kind: BundleKind,
    /// Estimated token budget this slice consumes when injected.
    pub token_estimate: u32,
    pub body: serde_json::Value,
}

impl Bundle {
    #[must_use]
    pub fn new(kind: BundleKind, body: serde_json::Value) -> Self {
        Self { kind, token_estimate: 0, body }
    }
}

/// The canonical continuation artifact injected into a NEW session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuationBundle {
    /// Source session id (or merged dedup key) this bundle resumes.
    pub source_id: String,
    pub bundles: Vec<Bundle>,
}

impl ContinuationBundle {
    #[must_use]
    pub fn new(source_id: impl Into<String>) -> Self {
        Self { source_id: source_id.into(), bundles: Vec::new() }
    }

    /// A bundle is *injectable* only when it carries an `Acceptance` gate —
    /// the operator-sized scope assertion that resume is safe.
    #[must_use]
    pub fn is_injectable(&self) -> bool {
        self.has(BundleKind::Acceptance)
    }

    #[must_use]
    pub fn has(&self, kind: BundleKind) -> bool {
        self.bundles.iter().any(|b| b.kind == kind)
    }

    /// Total injected token budget across all slices.
    #[must_use]
    pub fn total_token_estimate(&self) -> u32 {
        self.bundles.iter().map(|b| b.token_estimate).sum()
    }

    pub fn push(&mut self, bundle: Bundle) {
        self.bundles.push(bundle);
    }
}
