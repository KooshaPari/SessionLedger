use dioxus::prelude::*;
use session_ledger::domain::session::Session;

use crate::bundle_list::{summarize, BundleSummary};
use crate::corpus_loader::{load_sessions, DataSource};
use crate::detail_pane::{extract_detail, BundleDetail};
use crate::history_tab::HistoryTimeline;
use crate::live_feed::LiveFeed;
use crate::memory_tab::MemoryWiki;
use crate::mock_data::sample_bundles;
use crate::session_list::SessionList;

/// Tab identifiers for the viewer.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Bundles,
    History,
    Memory,
    LiveFeed,
}

/// Shared session data provided at the root of the component tree.
///
/// Consumers call `use_context::<SessionContext>()` to access the loaded sessions.
#[derive(Clone, Debug, PartialEq)]
pub struct SessionContext(pub Vec<Session>);

/// Resolve the active [`DataSource`].
///
/// Resolution order:
/// 1. `FORGE_DB` environment variable (path to a Forge SQLite file)
/// 2. Default: in-memory mock data
fn resolve_data_source() -> DataSource {
    #[cfg(feature = "sqlite")]
    if let Ok(path) = std::env::var("FORGE_DB") {
        let p = std::path::PathBuf::from(path);
        return DataSource::ForgeDb(p);
    }
    DataSource::Mock
}

/// Root application component.
///
/// Three-tab layout:
/// - **Bundles** — browse compiled continuation bundles (the original view)
/// - **History** — session history timeline (renders real Forge sessions when
///   `FORGE_DB` env var points at a Forge SQLite database)
/// - **Memory** — wiki/docs view of distilled memories
///
/// Real corpus data is loaded once at startup and injected via Dioxus context
/// so every child component can access it without prop-drilling.
#[component]
pub fn App() -> Element {
    // Load sessions once at the root; propagate via context.
    let source = resolve_data_source();
    let sessions = match load_sessions(&source) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[sl-viewer] failed to load corpus ({e}); falling back to mock data");
            crate::mock_data::sample_sessions()
        }
    };
    use_context_provider(|| SessionContext(sessions));

    let mut active_tab: Signal<Tab> = use_signal(|| Tab::Bundles);

    let bundles_class = if active_tab() == Tab::Bundles { "tab active" } else { "tab" };
    let history_class = if active_tab() == Tab::History { "tab active" } else { "tab" };
    let memory_class = if active_tab() == Tab::Memory { "tab active" } else { "tab" };
    let feed_class = if active_tab() == Tab::LiveFeed { "tab active" } else { "tab" };

    let tab_body = match active_tab() {
        Tab::Bundles => rsx! { BundlesTab {} },
        Tab::History => rsx! { HistoryTimeline {} },
        Tab::Memory => rsx! { MemoryWiki {} },
        Tab::LiveFeed => rsx! { LiveFeed {} },
    };

    rsx! {
        style {
            r#"
                body {{ margin: 0; font-family: system-ui, -apple-system, sans-serif; background: #0f1117; color: #e1e4ea; }}
                .app {{ display: flex; height: 100vh; }}
                .sidebar {{ width: 340px; min-width: 340px; border-right: 1px solid #2a2d35; overflow-y: auto; background: #161822; }}
                .sidebar h2 {{ padding: 16px 20px; margin: 0; font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: #8b8fa3; border-bottom: 1px solid #2a2d35; }}
                .bundle-entry {{ padding: 12px 20px; cursor: pointer; border-bottom: 1px solid #1e2029; transition: background 0.15s; }}
                .bundle-entry:hover {{ background: #1c1f2b; }}
                .bundle-entry.selected {{ background: #252836; border-left: 3px solid #6c8cff; }}
                .bundle-entry .source {{ font-size: 13px; font-weight: 600; color: #c8cdd6; }}
                .bundle-entry .goal {{ font-size: 12px; color: #8b8fa3; margin-top: 4px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
                .bundle-entry .meta {{ font-size: 11px; color: #5c5f6e; margin-top: 6px; display: flex; gap: 8px; }}
                .bundle-entry .badge {{ display: inline-block; padding: 1px 8px; border-radius: 4px; font-size: 10px; font-weight: 600; text-transform: uppercase; }}
                .badge-acceptance {{ background: #1a3a2a; color: #4ade80; }}
                .badge-contract {{ background: #2a1a3a; color: #c084fc; }}
                .detail {{ flex: 1; overflow-y: auto; padding: 32px 40px; }}
                .detail h1 {{ font-size: 18px; font-weight: 600; margin: 0 0 24px 0; color: #e1e4ea; }}
                .detail-section {{ margin-bottom: 24px; }}
                .detail-section h3 {{ font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: #6c8cff; margin: 0 0 8px 0; }}
                .detail-section p {{ font-size: 14px; line-height: 1.6; margin: 0; color: #c8cdd6; }}
                .detail-section ul {{ margin: 4px 0 0 0; padding-left: 20px; }}
                .detail-section li {{ font-size: 13px; line-height: 1.7; color: #a1a6b5; }}
                .empty-state {{ display: flex; align-items: center; justify-content: center; height: 100%; color: #5c5f6e; font-size: 14px; }}
                .tab-bar {{ display: flex; border-bottom: 1px solid #2a2d35; background: #13151c; }}
                .tab {{ flex: 1; padding: 10px 12px; text-align: center; cursor: pointer; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: #5c5f6e; border-bottom: 2px solid transparent; transition: all 0.15s; }}
                .tab:hover {{ color: #8b8fa3; background: #1a1c26; }}
                .tab.active {{ color: #6c8cff; border-bottom-color: #6c8cff; background: #161822; }}
                .session-list {{ display: flex; flex-direction: column; height: 100%; }}
                .search-input {{ width: 100%; padding: 10px 16px; background: #1c1f2b; border: 1px solid #2a2d35; border-radius: 6px; color: #e1e4ea; font-size: 13px; box-sizing: border-box; margin-bottom: 4px; }}
                .session-count {{ padding: 6px 20px; font-size: 11px; color: #5c5f6e; }}
                .session-item {{ padding: 12px 20px; cursor: pointer; border-bottom: 1px solid #1e2029; transition: background 0.15s; }}
                .session-item:hover {{ background: #1c1f2b; }}
                .session-item.selected {{ background: #252836; border-left: 3px solid #6c8cff; }}
                .session-source {{ font-size: 13px; font-weight: 600; color: #c8cdd6; }}
                .session-goal {{ font-size: 12px; color: #8b8fa3; margin-top: 4px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
                .session-meta {{ font-size: 11px; color: #5c5f6e; margin-top: 6px; display: flex; gap: 8px; align-items: center; }}
                .meta-bundles {{ color: #6c8cff; }}
                .badge {{ display: inline-block; padding: 1px 6px; border-radius: 4px; font-size: 10px; font-weight: 600; }}
                .badge-ok {{ background: #1a3a2a; color: #4ade80; }}
                .badge-contract {{ background: #2a1a3a; color: #c084fc; }}
                .live-feed {{ display: flex; flex-direction: column; height: 100%; }}
                .live-feed-header {{ display: flex; align-items: center; gap: 10px; padding: 10px 16px; border-bottom: 1px solid #2a2d35; background: #13151c; }}
                .live-feed-title {{ font-size: 13px; font-weight: 600; color: #c8cdd6; flex: 1; }}
                .feed-status {{ font-size: 11px; font-weight: 600; }}
                .feed-status.live {{ color: #4ade80; }}
                .feed-status.disconnected {{ color: #f87171; }}
                .feed-status.connecting {{ color: #facc15; }}
                .retry-btn {{ padding: 3px 10px; font-size: 11px; font-weight: 600; background: #252836; border: 1px solid #2a2d35; border-radius: 4px; color: #8b8fa3; cursor: pointer; }}
                .retry-btn:hover {{ background: #2f3244; color: #c8cdd6; }}
                .live-feed-list {{ flex: 1; overflow-y: auto; padding: 8px 0; }}
                .feed-empty {{ padding: 16px 20px; font-size: 13px; color: #5c5f6e; }}
                .feed-entry {{ display: flex; gap: 10px; align-items: baseline; padding: 6px 16px; border-bottom: 1px solid #1e2029; font-family: monospace; }}
                .feed-entry:hover {{ background: #1c1f2b; }}
                .feed-ts {{ font-size: 11px; color: #5c5f6e; white-space: nowrap; }}
                .feed-path {{ font-size: 12px; color: #a1b4ff; word-break: break-all; }}
            "#,
        }
        div { class: "app",
            div { class: "sidebar",
                div { class: "tab-bar",
                    div {
                        class: "{bundles_class}",
                        onclick: move |_| active_tab.set(Tab::Bundles),
                        "Bundles"
                    }
                    div {
                        class: "{history_class}",
                        onclick: move |_| active_tab.set(Tab::History),
                        "History"
                    }
                    div {
                        class: "{memory_class}",
                        onclick: move |_| active_tab.set(Tab::Memory),
                        "Memory"
                    }
                    div {
                        class: "{feed_class}",
                        onclick: move |_| active_tab.set(Tab::LiveFeed),
                        "Live Feed"
                    }
                }
                {tab_body}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Bundles tab (original compiled-bundles view)
// ---------------------------------------------------------------------------

/// The compiled-bundles tab — the original sidebar + detail panel.
#[component]
fn BundlesTab() -> Element {
    let bundles = use_signal(sample_bundles);
    let mut selected_idx: Signal<Option<usize>> = use_signal(|| None);

    let summaries: Vec<BundleSummary> = bundles.iter().map(|b| summarize(&b)).collect();
    let detail = selected_idx().and_then(|idx| bundles.get(idx)).map(|b| extract_detail(&b));

    rsx! {
        h2 { "Compiled Bundles" }
        SessionList {
            items: summaries,
            selected_idx: selected_idx(),
            on_select: move |idx| selected_idx.set(Some(idx)),
        }
        match detail {
            Some(d) => rsx! { DetailView { detail: d.clone() } },
            None => rsx! {
                div { class: "empty-state", "Select a bundle from the list to view details" }
            },
        }
    }
}

/// The right-hand detail panel showing full bundle contents.
#[component]
fn DetailView(detail: BundleDetail) -> Element {
    rsx! {
        div { class: "detail",
            h1 { "Bundle: {detail.source_id}" }

            // --- Intent section ---
            div { class: "detail-section",
                h3 { "Intent" }
                if let Some(ref goal) = detail.intent_goal {
                    p { "{goal}" }
                } else {
                    p { "(no goal)" }
                }
            }

            // --- Acceptance signals ---
            if !detail.acceptance_signals.is_empty() {
                div { class: "detail-section",
                    h3 { "Acceptance Signals" }
                    ul {
                        for sig in &detail.acceptance_signals {
                            li { "{sig}" }
                        }
                    }
                }
            }

            // --- Constraints ---
            if !detail.constraints.is_empty() {
                div { class: "detail-section",
                    h3 { "Constraints" }
                    ul {
                        for c in &detail.constraints {
                            li { "{c}" }
                        }
                    }
                }
            }

            // --- Context ---
            div { class: "detail-section",
                h3 { "Context" }
                if let Some(ref cwd) = detail.context_cwd {
                    p { "cwd: {cwd}" }
                }
                if let Some(ref title) = detail.context_title {
                    p { "title: {title}" }
                }
                if detail.context_cwd.is_none() && detail.context_title.is_none() {
                    p { "(no context data)" }
                }
            }

            // --- Contract criteria ---
            if !detail.contract_criteria.is_empty() {
                div { class: "detail-section",
                    h3 { "Contract Criteria" }
                    ul {
                        for crit in &detail.contract_criteria {
                            li { "{crit}" }
                        }
                    }
                }
            }

            // --- Token estimate ---
            div { class: "detail-section",
                h3 { "Token Budget" }
                p { "{detail.total_token_estimate} tokens across all slices" }
            }
        }
    }
}
