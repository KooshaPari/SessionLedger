//! Native-agent session resolution shared by ShareCLI and SessionLedger.
//!
//! The daemon deliberately accepts evidence rather than process handles. This
//! keeps the wire contract portable and lets the caller gather platform-specific
//! facts (PID, tty, argv, cwd) without giving the daemon authority to inspect or
//! control processes.

use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ProcessEvidence {
    pub executable: Option<String>,
    pub executable_fingerprint: Option<String>,
    pub argv: Vec<String>,
    pub environment: Vec<(String, String)>,
    pub cwd: Option<String>,
    pub pid: Option<u32>,
    pub tty: Option<String>,
    pub started_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResumeRecipe {
    pub executable: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AgentSession {
    pub session_id: String,
    pub harness: String,
    pub cwd: Option<String>,
    pub pid: Option<u32>,
    pub tty: Option<String>,
    pub resume: ResumeRecipe,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionConfidence {
    Exact,
    Corroborated,
    Heuristic,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SessionCandidate {
    pub session: AgentSession,
    pub confidence: ResolutionConfidence,
    pub matched_fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResolveRequest {
    pub evidence: ProcessEvidence,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResolveResponse {
    pub candidates: Vec<SessionCandidate>,
}

#[derive(Clone, Default)]
pub struct Resolver {
    sessions: Arc<RwLock<Vec<AgentSession>>>,
}

impl Resolver {
    pub fn register(&self, session: AgentSession) {
        let mut sessions = self.sessions.write().expect("resolver lock poisoned");
        sessions.retain(|existing| existing.session_id != session.session_id);
        sessions.push(session);
    }

    pub fn resolve(&self, evidence: &ProcessEvidence) -> ResolveResponse {
        let sessions = self.sessions.read().expect("resolver lock poisoned");
        let mut candidates = sessions.iter().filter_map(|session| score(session, evidence)).collect::<Vec<_>>();
        candidates.sort_by(|a, b| confidence_rank(&b.confidence).cmp(&confidence_rank(&a.confidence)));
        ResolveResponse { candidates }
    }
}

fn score(session: &AgentSession, evidence: &ProcessEvidence) -> Option<SessionCandidate> {
    let mut matched = Vec::new();
    if let (Some(pid), Some(expected)) = (evidence.pid, session.pid) {
        if pid == expected { matched.push("pid".into()); }
    }
    if let (Some(tty), Some(expected)) = (evidence.tty.as_ref(), session.tty.as_ref()) {
        if tty == expected { matched.push("tty".into()); }
    }
    if let (Some(cwd), Some(expected)) = (evidence.cwd.as_ref(), session.cwd.as_ref()) {
        if cwd == expected { matched.push("cwd".into()); }
    }
    if matched.is_empty() { return None; }
    let confidence = if matched.iter().any(|f| f == "pid") && matched.iter().any(|f| f == "tty") {
        ResolutionConfidence::Exact
    } else if matched.len() >= 2 {
        ResolutionConfidence::Corroborated
    } else {
        ResolutionConfidence::Heuristic
    };
    Some(SessionCandidate { session: session.clone(), confidence, matched_fields: matched })
}

fn confidence_rank(confidence: &ResolutionConfidence) -> u8 {
    match confidence { ResolutionConfidence::Exact => 3, ResolutionConfidence::Corroborated => 2, ResolutionConfidence::Heuristic => 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> AgentSession {
        AgentSession { session_id: "s1".into(), harness: "codex".into(), cwd: Some("/tmp/project".into()), pid: Some(42), tty: Some("ttys001".into()), resume: ResumeRecipe { executable: "codex".into(), args: vec!["resume".into(), "s1".into()], cwd: Some("/tmp/project".into()) } }
    }

    #[test]
    fn exact_pid_and_tty_match_wins() {
        let resolver = Resolver::default();
        resolver.register(session());
        let response = resolver.resolve(&ProcessEvidence { pid: Some(42), tty: Some("ttys001".into()), cwd: Some("/tmp/project".into()), ..Default::default() });
        assert_eq!(response.candidates[0].confidence, ResolutionConfidence::Exact);
    }

    #[test]
    fn unrelated_evidence_returns_no_candidate() {
        let resolver = Resolver::default();
        resolver.register(session());
        let response = resolver.resolve(&ProcessEvidence { pid: Some(7), cwd: Some("/other".into()), ..Default::default() });
        assert!(response.candidates.is_empty());
    }
}
