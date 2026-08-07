//! Raw Sessions tab — exposes the underlying `Vec<Session>` that every other
//! tab derives from, so users can see exactly what corpus discovery found
//! and reload discovery on demand.
//!
//! This is the closest the viewer comes to a "feed it data myself" affordance
//! for the local session corpora. A future revision will add a custom-path
//! picker that points [`corpus_loader::load_sessions`] at a directory the
//! user chooses (FR-RAW-2). For now the tab re-runs the same Auto discovery
//! the app already uses, with a Reload button to refresh.

use dioxus::prelude::*;
use session_ledger::domain::session::{Corpus, Session};

use crate::app::{DiscoveryState, ReloadTrigger, SessionContext};

/// Stable, human-readable label for a [`Corpus`].
fn corpus_label(corpus: Corpus) -> &'static str {
    match corpus {
        Corpus::Forge => "forge",
        Corpus::Codex => "codex",
        Corpus::ClaudeCode => "claude",
        Corpus::Cursor => "cursor",
        Corpus::FactoryDroid => "droid",
        Corpus::ChatGptWeb => "chatgpt",
        Corpus::ClaudeWeb => "claude web",
        Corpus::GeminiWeb => "gemini",
    }
}

/// Wall-clock timestamp (ms since epoch) of the most recent message in the
/// session, or `None` if the session has no messages or none are timestamped.
fn last_activity_ms(session: &Session) -> Option<i64> {
    session.messages.iter().filter_map(|m| m.ts_ms).max()
}

/// Format a millisecond timestamp as a short ISO-ish local date string,
/// falling back to "(no timestamp)" when the value is missing.
fn format_ms(ms: Option<i64>) -> String {
    match ms {
        Some(value) if value > 0 => {
            // Cheap, allocation-light format: YYYY-MM-DD HH:MM derived from
            // a naive UTC→local conversion. The viewer is a developer tool;
            // perfect locale handling isn't required here.
            let secs = value / 1000;
            let (y, mo, d, h, mi) = epoch_to_ymdhm(secs);
            format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}")
        }
        _ => "(no timestamp)".to_owned(),
    }
}

/// Pure UTC→calendar conversion (Gregorian, proleptic). Returns
/// (year, month_1_12, day_1_31, hour_0_23, minute_0_59) for the given epoch
/// seconds. Avoids pulling in a chrono dependency just for one date format.
fn epoch_to_ymdhm(secs: i64) -> (i32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let h = (rem / 3600) as u32;
    let mi = ((rem % 3600) / 60) as u32;
    // Civil-from-days algorithm by Howard Hinnant (public domain).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y } as i32;
    (y, m, d, h, mi)
}

/// The Raw Sessions component.
#[component]
pub fn CorpusTab() -> Element {
    let ctx = use_context::<SessionContext>();
    let discovery = use_context::<DiscoveryState>();
    let mut reload = use_context::<ReloadTrigger>();

    // Reactive read: rebuilt on every render that sees a sessions signal
    // change (or a manual Reload click). Sorted newest-first so the user
    // sees the most recently active sessions at the top.
    let mut sessions_sorted: Vec<Session> = ctx.0.read().clone();
    sessions_sorted.sort_by_key(|s| std::cmp::Reverse(last_activity_ms(s).unwrap_or(i64::MIN)));

    let total = sessions_sorted.len();
    let by_corpus = corpus_breakdown(&sessions_sorted);
    let plural = if total == 1 { "session" } else { "sessions" };
    let loading = discovery.loading.cloned();
    let load_error = discovery.error.cloned();

    rsx! {
        style { r#"
            .corpus-view {{
                display: flex;
                flex-direction: column;
                height: 100%;
                overflow: hidden;
            }}
            .corpus-toolbar {{
                display: flex;
                align-items: center;
                gap: var(--sl-space-md);
                padding: var(--sl-space-md) var(--sl-space-xl);
                border-bottom: 1px solid var(--sl-border);
                background: var(--sl-surface-muted);
            }}
            .corpus-title {{
                font-family: var(--font-ui);
                font-size: 14px;
                font-weight: 600;
                color: var(--sl-text);
                margin: 0;
            }}
            .corpus-count {{
                font-family: var(--font-ui);
                font-size: 12px;
                color: var(--sl-text-muted);
            }}
            .corpus-breakdown {{
                display: flex;
                gap: var(--sl-space-sm);
                flex-wrap: wrap;
                font-family: var(--font-ui);
                font-size: 11px;
                color: var(--sl-text-muted);
            }}
            .corpus-breakdown .badge {{
                padding: 2px 8px;
                border-radius: 999px;
                background: color-mix(in srgb, var(--sl-accent) 14%, transparent);
                color: var(--sl-accent);
            }}
            .corpus-reload-btn {{
                margin-left: auto;
                padding: 6px 14px;
                font-size: 12px;
                font-weight: 600;
                border-radius: var(--sl-radius-md);
                border: 1px solid var(--sl-border);
                background: var(--sl-surface);
                color: var(--sl-text);
                cursor: pointer;
            }}
            .corpus-reload-btn:hover {{
                border-color: var(--sl-accent);
                color: var(--sl-accent);
            }}
            .corpus-list {{
                flex: 1;
                overflow-y: auto;
                padding: 0;
            }}
            .corpus-row {{
                display: grid;
                grid-template-columns: 110px 1fr 90px 130px;
                gap: var(--sl-space-md);
                align-items: baseline;
                padding: 10px var(--sl-space-xl);
                border-bottom: 1px solid var(--sl-border);
                font-size: 13px;
            }}
            .corpus-row:hover {{
                background: var(--sl-surface-muted);
            }}
            .corpus-row .corpus-badge {{
                font-family: var(--font-ui);
                font-size: 10px;
                font-weight: 600;
                text-transform: uppercase;
                padding: 2px 8px;
                border-radius: var(--sl-radius-pill);
                background: color-mix(in srgb, var(--sl-accent) 18%, transparent);
                color: var(--sl-accent);
                text-align: center;
                letter-spacing: 0.5px;
            }}
            .corpus-row .corpus-id {{
                font-family: var(--font-mono);
                font-size: 12px;
                color: var(--sl-text-muted);
            }}
            .corpus-row .corpus-title-text {{
                color: var(--sl-text);
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
            }}
            .corpus-row .corpus-meta {{
                font-family: var(--font-ui);
                font-size: 11px;
                color: var(--sl-text-muted);
                text-align: right;
            }}
            .corpus-empty {{
                padding: var(--sl-space-2xl);
                text-align: center;
                color: var(--sl-text-muted);
                font-size: 13px;
            }}
        "# }
        section {
            class: "corpus-view",
            "aria-labelledby": "corpus-heading",
            header {
                class: "corpus-toolbar",
                h2 {
                    id: "corpus-heading",
                    class: "corpus-title",
                    "Raw Sessions"
                }
                span {
                    class: "corpus-count",
                    "data-testid": "corpus-count",
                    "{total} {plural} discovered"
                }
                div {
                    class: "corpus-breakdown",
                    "data-testid": "corpus-breakdown",
                    for entry in &by_corpus {
                        span {
                            class: "badge",
                            "data-corpus": "{entry.0}",
                            "{entry.0}: {entry.1}"
                        }
                    }
                }
                button {
                    class: "corpus-reload-btn",
                    r#type: "button",
                    "data-testid": "corpus-reload",
                    onclick: move |_| reload.0.with_mut(|t| *t += 1),
                    "Reload discovery"
                }
            }
            if sessions_sorted.is_empty() {
                if let Some(err) = load_error.as_ref() {
                    div {
                        class: "corpus-empty",
                        role: "status",
                        "Corpus load failed: {err}. Use Reload discovery to retry."
                    }
                } else if loading {
                    div {
                        class: "corpus-empty",
                        role: "status",
                        "Discovering local session corpus… (Codex + Claude + Cursor)"
                    }
                } else {
                    div {
                        class: "corpus-empty",
                        role: "status",
                        "No sessions loaded yet. Reload discovery or check that one of $HOME/.codex/sessions, $HOME/.claude/projects, $HOME/.cursor/projects exists."
                    }
                }
            } else {
                div {
                    class: "corpus-list",
                    "data-testid": "corpus-list",
                    for session in sessions_sorted.iter() {
                        CorpusRow { session: session.clone() }
                    }
                }
            }
        }
    }
}

/// One row of the corpus table — corpus, id, title, message count, last activity.
#[component]
fn CorpusRow(session: Session) -> Element {
    let id = session.id.clone();
    let title = session.title.clone().unwrap_or_else(|| "(untitled)".to_owned());
    let corpus = corpus_label(session.corpus);
    let count = session.messages.len();
    let last = format_ms(last_activity_ms(&session));

    rsx! {
        div {
            class: "corpus-row",
            "data-testid": "corpus-row",
            "data-corpus": "{corpus}",
            span { class: "corpus-badge", "{corpus}" }
            span {
                class: "corpus-title-text",
                "{title}"
                span {
                    class: "corpus-id",
                    style: "display:block;margin-top:2px;",
                    "{id}"
                }
            }
            span { class: "corpus-meta", "{count} msgs" }
            span { class: "corpus-meta", "{last}" }
        }
    }
}

/// Count sessions per corpus, returned as a stable ordering (insertion order
/// derived from the iteration order of the [`Corpus`] enum).
fn corpus_breakdown(sessions: &[Session]) -> Vec<(&'static str, usize)> {
    let mut counts: Vec<(&'static str, usize)> = Vec::new();
    for s in sessions {
        let label = corpus_label(s.corpus);
        if let Some(entry) = counts.iter_mut().find(|(l, _)| *l == label) {
            entry.1 += 1;
        } else {
            counts.push((label, 1));
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_ledger::domain::session::{Message, Role, Session};

    fn make_session(id: &str, corpus: Corpus, ts: Option<i64>) -> Session {
        let mut session = Session::new(id, corpus);
        if let Some(value) = ts {
            let mut message = Message::new(Role::User, format!("hello from {id}"));
            message.ts_ms = Some(value);
            session.messages.push(message);
        }
        session
    }

    #[test]
    fn corpus_breakdown_groups_by_corpus() {
        let sessions = vec![
            make_session("a", Corpus::Codex, Some(10)),
            make_session("b", Corpus::Codex, Some(20)),
            make_session("c", Corpus::ClaudeCode, Some(30)),
            make_session("d", Corpus::Cursor, None),
        ];
        let counts = corpus_breakdown(&sessions);
        let map: std::collections::HashMap<&str, usize> = counts.into_iter().collect();
        assert_eq!(map.get("codex"), Some(&2));
        assert_eq!(map.get("claude"), Some(&1));
        assert_eq!(map.get("cursor"), Some(&1));
    }

    #[test]
    fn last_activity_picks_max_timestamp() {
        let mut session = make_session("a", Corpus::Cursor, Some(100));
        let mut message = Message::new(Role::Assistant, "response".to_owned());
        message.ts_ms = Some(500);
        session.messages.push(message);
        assert_eq!(last_activity_ms(&session), Some(500));
    }

    #[test]
    fn format_ms_returns_placeholder_when_missing() {
        assert_eq!(format_ms(None), "(no timestamp)");
        assert_eq!(format_ms(Some(0)), "(no timestamp)");
    }

    #[test]
    fn epoch_to_ymdhm_handles_known_dates() {
        // 2000-01-01 00:00:00 UTC (Y2K midnight, well-known anchor)
        let secs = 946_684_800;
        let (y, m, d, h, mi) = epoch_to_ymdhm(secs);
        assert_eq!((y, m, d, h, mi), (2000, 1, 1, 0, 0));
        // 1970-01-01 00:00:00 UTC (Unix epoch)
        let (y, m, d, h, mi) = epoch_to_ymdhm(0);
        assert_eq!((y, m, d, h, mi), (1970, 1, 1, 0, 0));
    }
}
