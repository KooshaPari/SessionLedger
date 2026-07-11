use dioxus::prelude::*;
use session_ledger::domain::session::Session;

use crate::async_states::{ErrorState, LoadingState};
use crate::bundle_diff::{BundleDiff, OkfBundle};
use crate::bundle_list::{summarize, BundleSummary};
use crate::corpus_loader::{load_sessions, DataSource};
use crate::detail_pane::{extract_detail, BundleDetail};
use crate::history_tab::HistoryTimeline;
use crate::live_feed::LiveFeed;
use crate::memory_tab::MemoryWiki;
use crate::mock_data::sample_bundles;
use crate::replay_view::ReplayView;
use crate::search_view::SearchView;
use crate::theme::ThemeColors;
use crate::timeline::TimelineView;

/// Tab identifiers for the viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Bundles,
    History,
    Memory,
    LiveFeed,
    Search,
    Timeline,
    Replay,
}

impl Tab {
    const ALL: [Tab; 7] = [
        Tab::Bundles,
        Tab::History,
        Tab::Memory,
        Tab::LiveFeed,
        Tab::Search,
        Tab::Timeline,
        Tab::Replay,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::Bundles => "Bundles",
            Tab::History => "History",
            Tab::Memory => "Memory",
            Tab::LiveFeed => "Live Feed",
            Tab::Search => "Search",
            Tab::Timeline => "Timeline",
            Tab::Replay => "Replay",
        }
    }

    fn id(self) -> &'static str {
        match self {
            Tab::Bundles => "tab-bundles",
            Tab::History => "tab-history",
            Tab::Memory => "tab-memory",
            Tab::LiveFeed => "tab-live-feed",
            Tab::Search => "tab-search",
            Tab::Timeline => "tab-timeline",
            Tab::Replay => "tab-replay",
        }
    }

    fn panel_id(self) -> &'static str {
        match self {
            Tab::Bundles => "panel-bundles",
            Tab::History => "panel-history",
            Tab::Memory => "panel-memory",
            Tab::LiveFeed => "panel-live-feed",
            Tab::Search => "panel-search",
            Tab::Timeline => "panel-timeline",
            Tab::Replay => "panel-replay",
        }
    }

    fn index(self) -> usize {
        Self::ALL.iter().position(|&t| t == self).unwrap_or(0)
    }

    fn from_index(i: usize) -> Tab {
        Self::ALL[i % Self::ALL.len()]
    }
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
    let (sessions, corpus_error) = match load_sessions(&source) {
        Ok(s) => (s, None),
        Err(e) => {
            eprintln!("[sl-viewer] failed to load corpus ({e}); falling back to mock data");
            (crate::mock_data::sample_sessions(), Some(e))
        }
    };
    use_context_provider(|| SessionContext(sessions));

    let mut active_tab: Signal<Tab> = use_signal(|| Tab::Bundles);
    let colors = ThemeColors::dark();

    let tab_body = match active_tab() {
        Tab::Bundles => rsx! { BundlesTab {} },
        Tab::History => rsx! { HistoryTimeline {} },
        Tab::Memory => rsx! { MemoryWiki {} },
        Tab::LiveFeed => rsx! { LiveFeed {} },
        Tab::Search => rsx! { SearchView {} },
        Tab::Timeline => rsx! { TimelineView { bundles: sample_bundles() } },
        Tab::Replay => rsx! { ReplayView {} },
    };

    let mut activate = move |tab: Tab| {
        active_tab.set(tab);
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
                .tab {{ flex: 1; padding: 10px 12px; text-align: center; cursor: pointer; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: #5c5f6e; border: none; border-bottom: 2px solid transparent; background: transparent; transition: all 0.15s; font-family: inherit; }}
                .tab:hover {{ color: #8b8fa3; background: #1a1c26; }}
                .tab.active {{ color: #6c8cff; border-bottom-color: #6c8cff; background: #161822; }}
                .tab:focus {{ outline: none; }}
                .tab:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: -2px; color: {colors.focus}; }}
                .search-input:focus-visible, .search-btn:focus-visible, .retry-btn:focus-visible, .btn:focus-visible, .replay-input:focus-visible, .speed-input:focus-visible, .compare-btn:focus-visible, .sl-error-retry:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: 2px; }}
                .session-item:focus-visible, .feed-entry:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: -2px; }}
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
                .search-view {{ display: flex; flex-direction: column; height: 100%; overflow-y: auto; }}
                .search-form {{ padding: 0 0 8px 0; border-bottom: 1px solid #2a2d35; }}
                .search-fields {{ display: flex; flex-direction: column; gap: 4px; padding: 10px 16px; }}
                .search-label {{ font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: #5c5f6e; }}
                .search-actions {{ display: flex; gap: 8px; padding: 8px 16px 10px; }}
                .search-btn {{ padding: 6px 16px; font-size: 12px; font-weight: 600; border-radius: 5px; cursor: pointer; border: 1px solid #2a2d35; background: #252836; color: #8b8fa3; }}
                .search-btn:hover {{ background: #2f3244; color: #c8cdd6; }}
                .search-btn-primary {{ background: #1e2a4a; color: #6c8cff; border-color: #2a3a6a; }}
                .search-btn-primary:hover {{ background: #263460; color: #a0b4ff; }}
                .search-results {{ flex: 1; overflow-y: auto; }}
                .search-error {{ padding: 10px 16px; font-size: 13px; color: #f87171; background: #2a1a1a; border-bottom: 1px solid #3a2020; }}
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
                .compare-btn {{ padding: 2px 8px; font-size: 10px; font-weight: 600; background: #1e2435; border: 1px solid #2a2d35; border-radius: 4px; color: #8b8fa3; cursor: pointer; margin-left: 6px; }}
                .compare-btn:hover {{ background: #2a2d45; color: #c8cdd6; }}
                .compare-btn.active {{ background: #2a1a3a; color: #c084fc; border-color: #4a2a6a; }}
                .diff-panel {{ border-top: 2px solid #6c8cff; background: #0d0f18; padding: 0; flex-shrink: 0; max-height: 340px; overflow-y: auto; }}
                .diff-header {{ display: flex; align-items: center; padding: 10px 16px; border-bottom: 1px solid #2a2d35; background: #13151c; }}
                .diff-title {{ flex: 1; font-size: 13px; font-weight: 600; color: #c8cdd6; }}
                .diff-badge {{ display: inline-block; margin-left: 8px; padding: 1px 8px; border-radius: 10px; font-size: 11px; font-weight: 600; background: #2a1a1a; color: #f87171; }}
                .diff-badge-same {{ background: #1a3a2a; color: #4ade80; }}
                .diff-close {{ cursor: pointer; font-size: 14px; color: #5c5f6e; padding: 2px 6px; border-radius: 4px; }}
                .diff-close:hover {{ background: #252836; color: #c8cdd6; }}
                .diff-col-headers {{ display: grid; grid-template-columns: 160px 1fr 1fr; padding: 6px 16px; font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: #5c5f6e; border-bottom: 1px solid #2a2d35; background: #13151c; }}
                .diff-rows {{ display: flex; flex-direction: column; }}
                .diff-row {{ display: grid; grid-template-columns: 160px 1fr 1fr; padding: 6px 16px; font-size: 12px; border-bottom: 1px solid #1e2029; font-family: monospace; align-items: start; }}
                .diff-row-changed {{ background: #1a1520; }}
                .diff-row-changed .diff-col-a {{ color: #f87171; }}
                .diff-row-changed .diff-col-b {{ color: #4ade80; }}
                .diff-field-label {{ color: #8b8fa3; font-weight: 600; font-family: system-ui, sans-serif; font-size: 11px; padding-top: 1px; }}
                .diff-col-a {{ color: #c8cdd6; overflow-wrap: break-word; }}
                .diff-col-b {{ color: #c8cdd6; overflow-wrap: break-word; }}
                .main-content {{ flex: 1; display: flex; flex-direction: column; overflow: hidden; }}
                .main-upper {{ flex: 1; overflow-y: auto; }}
                .corpus-error-banner {{ padding: 0 8px; }}
                @media (prefers-reduced-motion: reduce) {{
                    *, *::before, *::after {{
                        animation-duration: 0.01ms !important;
                        animation-iteration-count: 1 !important;
                        transition-duration: 0.01ms !important;
                        scroll-behavior: auto !important;
                    }}
                }}
            "#,
        }
        div { class: "app",
            div { class: "sidebar",
                nav {
                    "aria-label": "Primary viewer navigation",
                    div {
                        class: "tab-bar",
                        role: "tablist",
                        "aria-label": "SessionLedger views",
                        for tab in Tab::ALL {
                            {
                                let is_active = active_tab() == tab;
                                let cls = if is_active { "tab active" } else { "tab" };
                                let selected = if is_active { "true" } else { "false" };
                                let tab_index = if is_active { "0" } else { "-1" };
                                rsx! {
                                    button {
                                        key: "{tab.id()}",
                                        id: "{tab.id()}",
                                        class: "{cls}",
                                        role: "tab",
                                        r#type: "button",
                                        tabindex: "{tab_index}",
                                        "aria-selected": "{selected}",
                                        "aria-controls": "{tab.panel_id()}",
                                        onclick: move |_| activate(tab),
                                        onkeydown: move |evt: Event<KeyboardData>| {
                                            let len = Tab::ALL.len();
                                            let idx = tab.index();
                                            match evt.key() {
                                                Key::Enter => {
                                                    evt.prevent_default();
                                                    activate(tab);
                                                }
                                                Key::Character(ref ch) if ch == " " => {
                                                    evt.prevent_default();
                                                    activate(tab);
                                                }
                                                Key::ArrowRight => {
                                                    evt.prevent_default();
                                                    activate(Tab::from_index(idx + 1));
                                                }
                                                Key::ArrowLeft => {
                                                    evt.prevent_default();
                                                    activate(Tab::from_index(idx + len - 1));
                                                }
                                                Key::Home => {
                                                    evt.prevent_default();
                                                    activate(Tab::Bundles);
                                                }
                                                Key::End => {
                                                    evt.prevent_default();
                                                    activate(Tab::Replay);
                                                }
                                                _ => {}
                                            }
                                        },
                                        "{tab.label()}"
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(ref err) = corpus_error {
                    div { class: "corpus-error-banner",
                        ErrorState {
                            message: format!("Corpus load failed ({err}); showing mock sessions."),
                        }
                    }
                }
                div {
                    id: "{active_tab().panel_id()}",
                    role: "tabpanel",
                    "aria-labelledby": "{active_tab().id()}",
                    {tab_body}
                }
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
    let mut bundles = use_signal(Vec::new);
    let mut loading = use_signal(|| true);
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    let mut load_gen: Signal<u32> = use_signal(|| 0u32);
    let mut selected_idx: Signal<Option<usize>> = use_signal(|| None);
    let mut compare_idx: Signal<Option<usize>> = use_signal(|| None);

    // Structured load gate so LoadingState / ErrorState cover async bundle fetch.
    // Today this is synchronous sample data; the same signals work for a future
    // daemon/HTTP loader without changing the chrome.
    use_effect(move || {
        let _ = load_gen();
        loading.set(true);
        load_error.set(None);
        let loaded = sample_bundles();
        if loaded.is_empty() {
            load_error.set(Some("No bundles available to display.".into()));
        } else {
            bundles.set(loaded);
        }
        loading.set(false);
    });

    if loading() {
        return rsx! {
            h2 { "Compiled Bundles" }
            LoadingState { message: "Loading bundles…".to_string() }
        };
    }
    if let Some(err) = load_error() {
        return rsx! {
            h2 { "Compiled Bundles" }
            ErrorState {
                message: err,
                retryable: true,
                on_retry: move |_| load_gen.with_mut(|g| *g += 1),
            }
        };
    }

    let summaries: Vec<BundleSummary> = bundles.iter().map(|b| summarize(&b)).collect();
    let detail = selected_idx().and_then(|idx| bundles.get(idx)).map(|b| extract_detail(&b));

    // Determine if we should show the diff panel.
    let diff_pair: Option<(OkfBundle, OkfBundle)> =
        selected_idx().zip(compare_idx()).and_then(|(ia, ib)| {
            let a = bundles.get(ia).as_ref().map(|b| OkfBundle::from_bundle(b))?;
            let c = bundles.get(ib).as_ref().map(|b| OkfBundle::from_bundle(b))?;
            Some((a, c))
        });

    rsx! {
        h2 { "Compiled Bundles" }
        SessionListWithCompare {
            items: summaries,
            selected_idx: selected_idx(),
            compare_idx: compare_idx(),
            on_select: move |idx| selected_idx.set(Some(idx)),
            on_compare: move |idx| {
                // Toggle: clicking same row again clears compare slot.
                if compare_idx() == Some(idx) {
                    compare_idx.set(None);
                } else {
                    compare_idx.set(Some(idx));
                }
            },
        }
        div { class: "main-content",
            div { class: "main-upper",
                match detail {
                    Some(d) => rsx! { DetailView { detail: d.clone() } },
                    None => rsx! {
                        div { class: "empty-state", "Select a bundle from the list to view details" }
                    },
                }
            }
            if let Some((a, b)) = diff_pair {
                BundleDiff {
                    bundle_a: a,
                    bundle_b: b,
                    on_close: move |_| compare_idx.set(None),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SessionList variant with a per-row "Compare" button
// ---------------------------------------------------------------------------

#[derive(Props, Clone, PartialEq)]
struct SessionListWithCompareProps {
    items: Vec<BundleSummary>,
    selected_idx: Option<usize>,
    compare_idx: Option<usize>,
    on_select: EventHandler<usize>,
    on_compare: EventHandler<usize>,
}

#[component]
fn SessionListWithCompare(props: SessionListWithCompareProps) -> Element {
    let mut query = use_signal(String::new);
    let needle = query().to_lowercase();
    let filtered: Vec<(usize, BundleSummary)> = props
        .items
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            needle.is_empty()
                || s.intent_goal.to_lowercase().contains(&needle)
                || s.source_id.to_lowercase().contains(&needle)
        })
        .map(|(i, s)| (i, s.clone()))
        .collect();
    let count = filtered.len();
    let plural = if count == 1 { "" } else { "s" };

    rsx! {
        div { class: "session-list",
            input {
                class: "search-input",
                placeholder: "Filter sessions...",
                value: "{query}",
                oninput: move |e| query.set(e.value()),
            }
            div { class: "session-count",
                "{count} session{plural}"
                if props.compare_idx.is_some() {
                    span { style: "color:#c084fc; margin-left:8px;", "compare slot active" }
                }
            }
            for (orig_idx, summary) in filtered.into_iter() {
                {
                    let is_selected = props.selected_idx == Some(orig_idx);
                    let is_compare = props.compare_idx == Some(orig_idx);
                    let cls = if is_selected { "session-item selected" } else { "session-item" };
                    let compare_cls = if is_compare { "compare-btn active" } else { "compare-btn" };
                    let s = summary.clone();
                    rsx! {
                        div {
                            class: "{cls}",
                            onclick: move |_| props.on_select.call(orig_idx),
                            div { class: "session-source",
                                "{s.source_id}"
                                span {
                                    class: "{compare_cls}",
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                        props.on_compare.call(orig_idx);
                                    },
                                    "⇄"
                                }
                            }
                            div { class: "session-goal", "{s.intent_goal}" }
                            div { class: "session-meta",
                                span { class: "meta-bundles", "󰆧 {s.bundle_count}" }
                                if s.has_acceptance {
                                    span { class: "badge badge-ok", "✓ AC" }
                                }
                                if s.has_contract {
                                    span { class: "badge badge-contract", "◎ CT" }
                                }
                            }
                        }
                    }
                }
            }
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
