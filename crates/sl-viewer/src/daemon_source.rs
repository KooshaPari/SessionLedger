//! Daemon-backed source for web viewer sessions.

use session_ledger::{
    domain::session::{Corpus, Message, Role, Session},
    validate_okf_document, OkfDocument,
};

/// Parse the `GET /api/bundles` response into the viewer's shared session model.
pub fn parse_daemon_bundles(body: &str) -> Result<Vec<Session>, String> {
    let documents: Vec<OkfDocument> = serde_json::from_str(body)
        .map_err(|error| format!("failed to parse daemon bundles: {error}"))?;

    documents.into_iter().map(session_from_okf).collect()
}

fn session_from_okf(document: OkfDocument) -> Result<Session, String> {
    if !validate_okf_document(&document).is_empty() {
        return Err(format!("failed to parse daemon bundles: invalid OKF {}", document.source_id));
    }

    let corpus = corpus_from_okf(&document.provenance.corpus)?;
    let cwd = entity_property(&document, "resource", "cwd");
    let title = entity_property(&document, "state", "title");
    let messages = document
        .entities
        .iter()
        .filter(|entity| entity.r#type == "intent" && !entity.label.trim().is_empty())
        .map(|entity| Message::new(Role::User, entity.label.clone()))
        .collect();

    Ok(Session { id: document.source_id, corpus, cwd, title, messages })
}

fn entity_property(document: &OkfDocument, entity_type: &str, property: &str) -> Option<String> {
    document
        .entities
        .iter()
        .find(|entity| entity.r#type == entity_type)
        .and_then(|entity| entity.properties.get(property))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}

fn corpus_from_okf(corpus: &str) -> Result<Corpus, String> {
    match corpus {
        "forge" => Ok(Corpus::Forge),
        "codex" => Ok(Corpus::Codex),
        "claude-code" => Ok(Corpus::ClaudeCode),
        "cursor" => Ok(Corpus::Cursor),
        "factory-droid" => Ok(Corpus::FactoryDroid),
        "chatgpt-web" => Ok(Corpus::ChatGptWeb),
        "claude-web" => Ok(Corpus::ClaudeWeb),
        "gemini-web" => Ok(Corpus::GeminiWeb),
        other => Err(format!("failed to parse daemon bundles: unsupported corpus {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_ledger::domain::session::Corpus;

    #[test]
    fn parses_daemon_documents_into_sessions() {
        let sessions = parse_daemon_bundles(
            r#"[{"okf":"1.0","source_id":"fuzz-a","entities":[{"id":"intent-0","type":"intent","label":"ship it","properties":null},{"id":"resource-1","type":"resource","label":"working-directory","properties":{"cwd":"/tmp/demo"}}],"provenance":{"corpus":"forge","source_id":"fuzz-a"},"tags":[]}]"#,
        )
        .expect("valid daemon document");

        assert_eq!(sessions[0].id, "fuzz-a");
        assert_eq!(sessions[0].corpus, Corpus::Forge);
        assert_eq!(sessions[0].cwd.as_deref(), Some("/tmp/demo"));
        assert_eq!(sessions[0].messages[0].content, "ship it");
    }

    #[test]
    fn rejects_invalid_okf_response_without_mock_fallback() {
        let error = parse_daemon_bundles(r#"[{"okf":"2.0"}]"#)
            .expect_err("incomplete response is rejected");

        assert!(error.contains("failed to parse daemon bundles"));
    }
}
