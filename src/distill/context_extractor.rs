//! Heuristic context extraction — the P2 adapter for [`ContextExtractor`].
//!
//! This adapter uses lightweight string matching to extract working context
//! from a session's message stream: files touched, decisions reached, symbols
//! referenced, and environment state.
//!
//! Phase 3 will supersede this with an LLM-backed extractor behind the same
//! [`ContextExtractor`] trait — see `docs/DESIGN.md` §7.

use crate::domain::context::{Context, Decision};
use crate::domain::session::Session;
use crate::ports::{ContextExtractor, PortError};

/// Heuristic-based context extractor.
///
/// Scans all messages (user + assistant) for:
/// - **Files mentioned**: paths containing `/` or `.` (e.g. `src/main.rs`).
/// - **Key decisions**: phrases like "decided", "let's use", "we chose".
/// - **Key symbols**: identifiers with `::` (e.g. `HashMap::new`) or function
///   calls (`foo()`).
/// - **Environment notes**: dependency/setup-related messages.
///
/// This is intentionally simple. The LLM-backed extractor (Phase 3) will
/// replace it behind the same trait.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeuristicContextExtractor;

// Patterns for detecting file paths (fragments that contain `/` or known
// extensions — checked on lowercase tokens).
const FILE_EXTENSIONS: &[&str] = &[
    ".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go", ".java", ".kt", ".rb", ".c", ".h", ".cpp",
    ".hpp", ".cs", ".swift", ".toml", ".json", ".yaml", ".yml", ".md", ".sql", ".css", ".scss",
    ".html", ".sh", ".tf", ".lock",
];

// Patterns for decision language (lowercased for matching).
const DECISION_PATTERNS: &[&str] = &[
    "decided",
    "decision",
    "let's use",
    "lets use",
    "we should",
    "we chose",
    "chose",
    "opted for",
    "went with",
    "picked",
    "settled on",
    "i'll go with",
    "going with",
    "best to use",
    "prefer",
];

// Patterns for environment / setup notes.
const ENVIRONMENT_PATTERNS: &[&str] = &[
    "install",
    "installed",
    "setup",
    "set up",
    "configure",
    "version",
    "npm",
    "cargo",
    "pip",
    "brew",
    "apt",
    "docker",
    "compose",
    "env",
    "export",
    "add ",
    "added ",
    "upgrade",
    "updated",
];

impl HeuristicContextExtractor {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Heuristically extract working-context from a session's messages.
    ///
    /// This is factored as a public associated function so it can be used
    /// directly by the compiler without going through the trait when no
    /// adapter injection is needed (P2 default).
    #[must_use]
    pub fn extract_context(session: &Session) -> Context {
        let mut files_mentioned: Vec<String> = Vec::new();
        let mut key_decisions: Vec<Decision> = Vec::new();
        let mut key_symbols: Vec<String> = Vec::new();
        let mut environment_notes: Vec<String> = Vec::new();

        for msg in &session.messages {
            let content = msg.content.as_str();
            let lower = content.to_lowercase();
            let tokens: Vec<&str> = content.split_whitespace().collect();

            // --- Files ---
            for token in &tokens {
                let clean = token.trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '/' && c != '.' && c != '-' && c != '_'
                });
                if is_file_path(clean) {
                    let fp = clean.to_string();
                    if !files_mentioned.contains(&fp) {
                        files_mentioned.push(fp);
                    }
                }
            }

            // --- Symbols (identifiers with :: or () pattern) ---
            // Look for token-like patterns (CamelCase, snake_case with parens or ::)
            for token in &tokens {
                let clean = token.trim_matches(|c: char| {
                    c == '('
                        || c == ')'
                        || c == ','
                        || c == ';'
                        || c == '{'
                        || c == '}'
                        || c == '['
                        || c == ']'
                });
                let is_func_call = token.contains("()");
                if clean.contains("::") || is_func_call {
                    let sym = clean.to_string();
                    if !key_symbols.contains(&sym) {
                        key_symbols.push(sym);
                    }
                }
            }

            // --- Decisions ---
            for pat in DECISION_PATTERNS {
                if lower.contains(pat) {
                    let summary = format!("Session contains '{pat}' language");
                    let decision = Decision { summary, rationale: Some(content.to_string()) };
                    if !key_decisions.iter().any(|d| d.summary == decision.summary) {
                        key_decisions.push(decision);
                    }
                }
            }

            // --- Environment notes ---
            for pat in ENVIRONMENT_PATTERNS {
                if lower.contains(pat) {
                    let note = format!("Session mentions '{pat}'");
                    if !environment_notes.contains(&note) {
                        environment_notes.push(note);
                    }
                }
            }
        }

        // Deduplicate environment notes by prefix (collapse "Session mentions 'npm'"
        // and "Session mentions 'cargo'" into a single note if both appear from
        // the same trigger).
        environment_notes.sort();
        environment_notes.dedup();

        Context {
            cwd: session.cwd.clone(),
            title: session.title.clone(),
            files_mentioned,
            key_decisions,
            key_symbols,
            environment_notes,
        }
    }
}

impl ContextExtractor for HeuristicContextExtractor {
    fn extract(&self, session: &Session) -> Result<Context, PortError> {
        Ok(Self::extract_context(session))
    }
}

/// Whether a whitespace-delimited token looks like a file path.
#[must_use]
fn is_file_path(token: &str) -> bool {
    if token.is_empty() || token.len() < 3 {
        return false;
    }
    // Must contain a `/` or start like a relative path (e.g. `src/`).
    if token.contains('/') {
        return true;
    }
    // Match common file extensions.
    let lower = token.to_lowercase();
    FILE_EXTENSIONS.iter().any(|ext| lower.ends_with(ext) && lower.len() > ext.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::session::{Corpus, Message, Role};

    fn fixture_session() -> Session {
        let mut s = Session::new("test-sess-ctx", Corpus::Forge);
        s.cwd = Some("/home/user/proj".into());
        s.title = Some("refactor auth".into());
        s.messages
            .push(Message::new(Role::User, "we need to refactor src/auth/mod.rs and add tests"));
        s.messages.push(Message::new(
            Role::Assistant,
            "I decided to use JWT tokens. Let's use jsonwebtoken crate.",
        ));
        s.messages.push(Message::new(
            Role::User,
            "good, I installed the crate and ran cargo test — it passes",
        ));
        s.messages.push(Message::new(
            Role::Assistant,
            "The verify_token() function validates expiry correctly now.",
        ));
        s
    }

    #[test]
    fn extracts_cwd_and_title_from_session() {
        let ctx = HeuristicContextExtractor::extract_context(&fixture_session());
        assert_eq!(ctx.cwd.as_deref(), Some("/home/user/proj"));
        assert_eq!(ctx.title.as_deref(), Some("refactor auth"));
    }

    #[test]
    fn extracts_files_mentioned_from_messages() {
        let ctx = HeuristicContextExtractor::extract_context(&fixture_session());
        assert!(ctx.files_mentioned.iter().any(|f| f == "src/auth/mod.rs"));
    }

    #[test]
    fn extracts_decisions_from_discourse() {
        let ctx = HeuristicContextExtractor::extract_context(&fixture_session());
        assert!(ctx.key_decisions.iter().any(|d| d.summary.contains("decided")));
    }

    #[test]
    fn extracts_symbols_from_code_mentions() {
        let ctx = HeuristicContextExtractor::extract_context(&fixture_session());
        assert!(ctx.key_symbols.iter().any(|s| s == "verify_token" || s.contains("verify_token")));
    }

    #[test]
    fn extracts_environment_notes() {
        let ctx = HeuristicContextExtractor::extract_context(&fixture_session());
        let notes: Vec<&str> = ctx.environment_notes.iter().map(String::as_str).collect();
        assert!(notes.iter().any(|n| n.contains("install") || n.contains("cargo")));
    }

    #[test]
    fn returns_empty_context_for_empty_session() {
        let s = Session::new("empty", Corpus::Forge);
        let ctx = HeuristicContextExtractor::extract_context(&s);
        assert!(ctx.is_empty());
    }

    #[test]
    fn context_extractor_trait_works() {
        let extractor = HeuristicContextExtractor::new();
        let ctx = extractor.extract(&fixture_session()).expect("extraction should succeed");
        assert_eq!(ctx.cwd.as_deref(), Some("/home/user/proj"));
    }
}
