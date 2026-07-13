//! Durable local audit sink for structured actor/action events.

use std::fs::OpenOptions;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

pub(crate) const AUDIT_RELATIVE_PATH: &str = "audit/events.jsonl";
pub(crate) const LOCAL_ACTOR: &str = "local";

#[derive(Clone, Debug)]
pub(crate) struct AuditSink {
    path: Arc<PathBuf>,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuditEvent<'a> {
    pub timestamp: u128,
    pub actor: &'a str,
    pub action: &'a str,
    pub outcome: &'a str,
    pub request_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

impl AuditSink {
    pub(crate) fn new(data_dir: impl AsRef<Path>) -> Self {
        Self { path: Arc::new(data_dir.as_ref().join(AUDIT_RELATIVE_PATH)) }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn append(&self, event: &AuditEvent<'_>) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new().create(true).append(true).open(self.path.as_ref())?;
        serde_json::to_writer(&mut file, event)?;
        file.write_all(b"\n")?;
        file.flush()?;
        file.sync_data()
    }
}

pub(crate) fn data_dir_from_env_or(default_dir: &Path) -> PathBuf {
    std::env::var_os("SL_DATA_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| default_dir.to_owned())
}

pub(crate) fn timestamp_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

pub(crate) fn local_request_id() -> String {
    format!("local-{}-{}", std::process::id(), timestamp_unix_ms())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_sink_appends_jsonl_without_rewriting_existing_records() {
        let dir = tempfile::TempDir::new().unwrap();
        let sink = AuditSink::new(dir.path());
        let first = AuditEvent {
            timestamp: 1,
            actor: LOCAL_ACTOR,
            action: "ingest",
            outcome: "accepted",
            request_id: "req-1",
            reason: Some("validation"),
            resource: None,
        };
        let second = AuditEvent {
            timestamp: 2,
            actor: LOCAL_ACTOR,
            action: "export",
            outcome: "succeeded",
            request_id: "req-2",
            reason: None,
            resource: Some("bundle.okf.json".to_owned()),
        };

        sink.append(&first).unwrap();
        let first_len = std::fs::metadata(sink.path()).unwrap().len();
        sink.append(&second).unwrap();
        let second_len = std::fs::metadata(sink.path()).unwrap().len();

        assert!(second_len > first_len, "audit file should grow after each append");
        let contents = std::fs::read_to_string(sink.path()).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 2);

        let parsed: Vec<serde_json::Value> =
            lines.iter().map(|line| serde_json::from_str(line).unwrap()).collect();
        assert_eq!(parsed[0]["action"], "ingest");
        assert_eq!(parsed[0]["request_id"], "req-1");
        assert_eq!(parsed[1]["action"], "export");
        assert_eq!(parsed[1]["resource"], "bundle.okf.json");
    }
}
