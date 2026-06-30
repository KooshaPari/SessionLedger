//! Normalized session model — the common shape every ingestion adapter targets.

use serde::{Deserialize, Serialize};

/// Origin corpus a session was ingested from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Corpus {
    Forge,
    Codex,
    ClaudeCode,
    Cursor,
    FactoryDroid,
}

/// Who authored a message. Distinguishing user vs subagent is load-bearing for
/// intent extraction (the curation pipeline classifies these explicitly).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    User,
    Assistant,
    Subagent,
    Tool,
    System,
}

/// A single turn in a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    /// Unix millis; `None` when the source omits a timestamp.
    pub ts_ms: Option<i64>,
}

impl Message {
    #[must_use]
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self { role, content: content.into(), ts_ms: None }
    }
}

/// A normalized session: the unit `SessionLedger` compiles, distills, and resumes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub corpus: Corpus,
    /// Working directory / project scope, when known. Drives the dedup key.
    pub cwd: Option<String>,
    pub title: Option<String>,
    pub messages: Vec<Message>,
}

impl Session {
    #[must_use]
    pub fn new(id: impl Into<String>, corpus: Corpus) -> Self {
        Self { id: id.into(), corpus, cwd: None, title: None, messages: Vec::new() }
    }

    /// Count of turns authored by a human operator (not agent/subagent/tool).
    #[must_use]
    pub fn user_turns(&self) -> usize {
        self.messages.iter().filter(|m| m.role == Role::User).count()
    }
}
