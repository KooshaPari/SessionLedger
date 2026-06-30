//! Dedup keys for merging duplicate-scoped chats.
//!
//! Use case (a): after a crash the operator wants to resume ONE chat, not many.
//! Sessions that share a scope (project cwd + normalized intent topic) collapse
//! under a single [`DedupKey`], so their continuation bundles can be merged.

use crate::domain::session::Session;
use sha2::{Digest, Sha256};

/// A stable key identifying a logical "scope" a session belongs to.
///
/// Two sessions with the same `DedupKey` are candidates for merge. The key is a
/// SHA-256 over the normalized scope (cwd) plus a topic slug, so it is stable
/// across crashes and re-ingestion.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DedupKey(String);

impl DedupKey {
    /// Derive a dedup key from a session's scope and a topic slug.
    #[must_use]
    pub fn derive(session: &Session, topic_slug: &str) -> Self {
        let scope = session.cwd.as_deref().unwrap_or("<no-cwd>").trim().to_lowercase();
        let topic = topic_slug.trim().to_lowercase();
        let mut hasher = Sha256::new();
        hasher.update(scope.as_bytes());
        hasher.update([0u8]);
        hasher.update(topic.as_bytes());
        Self(format!("{:x}", hasher.finalize()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
