use dioxus::prelude::*;
use session_ledger::domain::session::Session;

use crate::async_states::{ContentSkeleton, ErrorState, SkeletonLayout};
use crate::bundle_diff::{BundleDiff, OkfBundle};
use crate::bundle_list::{summarize, BundleSummary};
use crate::corpus_loader::{load_sessions, DataSource};
use crate::detail_pane::{extract_detail, BundleDetail};
use crate::fixture::query_fixture_active;
use crate::history_tab::HistoryTimeline;
use crate::live_feed::LiveFeed;
use crate::memory_tab::MemoryWiki;
use crate::mock_data::sample_bundles;
use crate::replay_view::ReplayView;
use crate::search_view::SearchView;
use crate::theme::ThemeColors;
use crate::timeline::TimelineView;
use crate::unfinished_tab::UnfinishedWork;

/// Tab identifiers for the viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Bundles,
    History,
    Unfinished,
    Memory,
    LiveFeed,
    Search,
    Timeline,
    Replay,
}

impl Tab {
    const ALL: [Tab; 8] = [
        Tab::Bundles,
        Tab::History,
        Tab::Unfinished,
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
            Tab::Unfinished => "Unfinished",
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
            Tab::Unfinished => "tab-unfinished",
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
            Tab::Unfinished => "panel-unfinished",
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
    #[cfg(feature = "web")]
    use_effect(|| {
        let _ = document::eval(
            r#"
            document.documentElement.lang = 'en';
            const stored = window.localStorage.getItem('sl-viewer-theme');
            const prefersLight = window.matchMedia?.('(prefers-color-scheme: light)').matches;
            const theme = stored === 'light' || stored === 'dark'
                ? stored
                : (prefersLight ? 'light' : 'dark');
            document.documentElement.dataset.theme = theme;
            "#,
        );
    });

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

    #[cfg(feature = "web")]
    use_effect(|| {
        let _ = document::eval(
            r#"
            window.setTimeout(() => {
              const splash = document.querySelector('.launch-splash');
              if (splash) splash.remove();
            }, 1800);
            "#,
        );
    });

    let tab_body = match active_tab() {
        Tab::Bundles => rsx! { BundlesTab {} },
        Tab::History => rsx! { HistoryTimeline {} },
        Tab::Unfinished => rsx! { UnfinishedWork {} },
        Tab::Memory => rsx! { MemoryWiki {} },
        Tab::LiveFeed => rsx! { LiveFeed {} },
        Tab::Search => rsx! { SearchView {} },
        Tab::Timeline => rsx! { TimelineView { bundles: sample_bundles() } },
        Tab::Replay => rsx! { ReplayView {} },
    };

    let mut activate = move |tab: Tab| {
        active_tab.set(tab);
        let _ = document::eval(&format!("document.getElementById('{}')?.focus();", tab.id()));
    };

    rsx! {
        style {
            r#"
                :root {{
                    color-scheme: light;
                    --font-display: ui-serif, Georgia, "Times New Roman", serif;
                    --font-body: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
                    --font-mono: ui-monospace, "SF Mono", Menlo, Consolas, monospace;
                    --font-ui: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
                    --sl-font-caption: var(--font-ui);
                    --sl-font-size-caption: 0.75rem;
                    --sl-line-height-caption: 1.35;
                    --sl-measure-max: 65ch;
                    --sl-bg: #f6f8fa;
                    --sl-surface: #ffffff;
                    --sl-surface-muted: #eef2f7;
                    --sl-border: #d8dee8;
                    --sl-text: #1f2937;
                    --sl-text-muted: #5c5f6e;
                    --sl-accent: #2563eb;
                    --sl-accent-secondary: #14b8a6;
                    --sl-accent-warning: #f97316;
                    --sl-danger: #b91c1c;
                    --sl-danger-surface: #fef2f2;
                    --sl-space-xs: 4px;
                    --sl-space-sm: 8px;
                    --sl-space-md: 12px;
                    --sl-space-lg: 16px;
                    --sl-space-xl: 20px;
                    --sl-space-2xl: 32px;
                    --sl-radius-sm: 4px;
                    --sl-radius-md: 6px;
                    --sl-radius-lg: 8px;
                    --sl-radius-pill: 10px;
                    --sl-motion-fast: 150ms;
                    --sl-motion-medium: 220ms;
                    --sl-motion-slow: 1.8s;
                    --sl-ease-out: ease-out;
                    --sl-ease-in-out: ease-in-out;
                    --sl-skeleton-base: #e8ecf2;
                    --sl-skeleton-highlight: rgba(37, 99, 235, 0.14);
                }}
                :root[data-theme="dark"] {{
                    color-scheme: dark;
                    --sl-bg: #111827;
                    --sl-surface: #1f2937;
                    --sl-surface-muted: #243244;
                    --sl-border: #374151;
                    --sl-text: #f3f4f6;
                    --sl-text-muted: #b6bfcc;
                    /* On-dark cobalt: AA ≥4.5:1 on slate + accent color-mix chrome. */
                    --sl-accent: #93c5fd;
                    --sl-accent-secondary: #2dd4bf;
                    --sl-accent-warning: #f97316;
                    --sl-danger: #f87171;
                    --sl-danger-surface: #2a1a1a;
                    --sl-space-xs: 4px;
                    --sl-space-sm: 8px;
                    --sl-space-md: 12px;
                    --sl-space-lg: 16px;
                    --sl-space-xl: 20px;
                    --sl-space-2xl: 32px;
                    --sl-radius-sm: 4px;
                    --sl-radius-md: 6px;
                    --sl-radius-lg: 8px;
                    --sl-radius-pill: 10px;
                    --sl-motion-fast: 150ms;
                    --sl-motion-medium: 220ms;
                    --sl-motion-slow: 1.8s;
                    --sl-ease-out: ease-out;
                    --sl-ease-in-out: ease-in-out;
                    --sl-skeleton-base: #2b3544;
                    --sl-skeleton-highlight: rgba(147, 197, 253, 0.14);
                }}
                html, body {{ margin: 0; max-width: 100%; overflow-x: clip; }}
                body {{ font-family: var(--font-body); background: var(--sl-bg); color: var(--sl-text); }}
                .app {{ display: flex; flex-direction: column; height: 100vh; width: 100%; max-width: 100vw; overflow-x: clip; }}
                .app > .sidebar {{
                    width: 100%;
                    min-width: 0;
                    max-width: 100%;
                    border-right: none;
                    overflow-y: auto;
                    overflow-x: clip;
                    background: var(--sl-surface);
                    display: flex;
                    flex-direction: column;
                    flex: 1;
                    min-height: 0;
                }}
                .viewer-main .sidebar {{
                    width: 100%;
                    min-width: 0;
                    max-width: 100%;
                    border-right: none;
                }}
                .sidebar h2 {{ padding: 16px 20px; margin: 0; font-family: var(--font-ui); font-size: 14px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: var(--sl-text-muted); border-bottom: 1px solid var(--sl-border); }}
                .bundle-entry {{ padding: var(--sl-space-md) var(--sl-space-xl); cursor: pointer; border-bottom: 1px solid var(--sl-border); transition: background var(--sl-motion-fast) var(--sl-ease-out); }}
                .bundle-entry:hover {{ background: var(--sl-surface-muted); }}
                .bundle-entry.selected {{ background: var(--sl-surface-muted); border-left: 3px solid var(--sl-accent); }}
                .bundle-entry .source {{ font-size: 13px; font-weight: 600; color: var(--sl-text); }}
                .bundle-entry .goal {{ font-size: 12px; color: var(--sl-text-muted); margin-top: 4px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
                .bundle-entry .meta {{ font-size: 11px; color: var(--sl-text-muted); margin-top: 6px; display: flex; gap: 8px; }}
                .bundle-entry .badge {{ display: inline-block; padding: 1px 8px; border-radius: 4px; font-size: 10px; font-weight: 600; text-transform: uppercase; }}
                .badge-acceptance {{ background: color-mix(in srgb, var(--sl-accent-secondary) 18%, transparent); color: var(--sl-accent-secondary); }}
                .badge-contract {{ background: color-mix(in srgb, var(--sl-accent) 16%, transparent); color: var(--sl-accent); }}
                .detail {{ flex: 1; overflow-y: auto; padding: 32px 40px; }}
                .detail h1 {{ font-family: var(--font-display); font-size: 18px; font-weight: 600; margin: 0 0 24px 0; color: var(--sl-text); }}
                .detail-section {{ margin-bottom: 24px; }}
                .detail-section h3 {{ font-family: var(--font-ui); font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; color: var(--sl-accent); margin: 0 0 8px 0; }}
                .detail-section p {{ font-size: 14px; line-height: 1.6; margin: 0; color: var(--sl-text); max-width: var(--sl-measure-max); }}
                .detail-section ul {{ margin: 4px 0 0 0; padding-left: 20px; max-width: var(--sl-measure-max); }}
                .detail-section li {{ font-size: 13px; line-height: 1.7; color: var(--sl-text-muted); }}
                .caption {{ font-family: var(--sl-font-caption); font-size: var(--sl-font-size-caption); line-height: var(--sl-line-height-caption); color: var(--sl-text-muted); }}
                .launch-splash {{
                    position: fixed;
                    inset: 0;
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background: var(--sl-bg);
                    animation: splash-dismiss var(--sl-motion-medium) var(--sl-ease-out) forwards;
                    animation-delay: 1.2s;
                }}
                .launch-splash-inner {{ text-align: center; }}
                .launch-splash-mark {{
                    display: block;
                    font-family: var(--font-display);
                    font-size: 1.75rem;
                    font-weight: 600;
                    color: var(--sl-accent);
                    letter-spacing: -0.02em;
                }}
                .launch-splash-caption {{
                    display: block;
                    margin-top: var(--sl-space-sm);
                    font-family: var(--sl-font-caption);
                    font-size: var(--sl-font-size-caption);
                    line-height: var(--sl-line-height-caption);
                    color: var(--sl-text-muted);
                    text-transform: uppercase;
                    letter-spacing: 0.12em;
                }}
                @keyframes splash-dismiss {{
                    to {{ opacity: 0; visibility: hidden; pointer-events: none; }}
                }}
                .empty-state {{ display: flex; align-items: center; justify-content: center; height: 100%; color: var(--sl-text-muted); font-size: 14px; }}
                .sl-content-skeleton {{ display: flex; flex: 1; min-height: 0; overflow: hidden; }}
                .sl-content-skeleton-bundles {{ flex-direction: row; }}
                .sl-content-skeleton-list {{ flex-direction: column; }}
                .sl-content-skeleton-stream {{ flex-direction: column; flex: 1; padding: var(--sl-space-md) var(--sl-space-lg); box-sizing: border-box; }}
                .sl-skeleton-stream-lines {{ display: flex; flex-direction: column; gap: var(--sl-space-sm); width: 100%; font-family: "SF Mono", "Menlo", "Consolas", monospace; }}
                .sl-skeleton-stream-line-wrap {{ min-height: 16px; }}
                .sl-skeleton-stream-line {{ height: 12px; }}
                .sl-skeleton-list {{ width: 340px; min-width: 340px; max-width: 340px; border-right: 1px solid var(--sl-border); padding: var(--sl-space-sm) 0; box-sizing: border-box; }}
                .sl-skeleton-row {{ padding: var(--sl-space-md) var(--sl-space-xl); border-bottom: 1px solid var(--sl-border); min-height: 72px; box-sizing: border-box; }}
                .sl-skeleton-block {{ border-radius: var(--sl-radius-sm); background: linear-gradient(90deg, var(--sl-skeleton-base) 0%, var(--sl-skeleton-highlight) 50%, var(--sl-skeleton-base) 100%); background-size: 200% 100%; animation: sl-skeleton-shimmer var(--sl-motion-slow) var(--sl-ease-in-out) infinite; }}
                .sl-skeleton-block-title {{ height: 13px; width: 62%; margin-bottom: var(--sl-space-sm); }}
                .sl-skeleton-block-subtitle {{ height: 12px; width: 84%; margin-bottom: var(--sl-space-sm); }}
                .sl-skeleton-block-meta {{ height: 10px; width: 38%; }}
                .sl-skeleton-detail {{ flex: 1; padding: var(--sl-space-2xl) 40px; box-sizing: border-box; }}
                .sl-skeleton-block-heading {{ height: 18px; width: 48%; margin-bottom: var(--sl-space-xl); }}
                .sl-skeleton-block-line {{ height: 14px; width: 100%; max-width: var(--sl-measure-max); margin-bottom: var(--sl-space-md); }}
                .sl-skeleton-block-line-short {{ width: 72%; }}
                @keyframes sl-skeleton-shimmer {{
                    0% {{ background-position: 100% 0; }}
                    100% {{ background-position: -100% 0; }}
                }}
                @media (max-width: 600px) {{
                    .sl-skeleton-list {{ width: 100%; min-width: 0; max-width: 100%; border-right: none; }}
                    .sl-content-skeleton-bundles {{ flex-direction: column; }}
                    .sl-skeleton-detail {{ padding: var(--sl-space-lg); }}
                }}
                .tab-bar {{ display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); border-bottom: 1px solid var(--sl-border); background: var(--sl-surface-muted); }}
                .tab {{ flex: 1; padding: var(--sl-space-md) var(--sl-space-md); text-align: center; cursor: pointer; font-family: var(--font-ui); font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: var(--sl-text-muted); border: none; border-bottom: 2px solid transparent; background: transparent; transition: all var(--sl-motion-fast) var(--sl-ease-out); }}
                .tab:hover {{ color: var(--sl-text); background: color-mix(in srgb, var(--sl-accent) 8%, transparent); }}
                .tab.active {{ color: var(--sl-accent); border-bottom-color: var(--sl-accent); background: var(--sl-surface); }}
                .tab:focus {{ outline: none; }}
                .tab:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: -2px; color: {colors.focus}; }}
                .theme-toggle {{ width: calc(100% - 32px); margin: 10px 16px; padding: 7px 12px; border: 1px solid var(--sl-border); border-radius: 6px; background: var(--sl-surface-muted); color: var(--sl-text); cursor: pointer; font-family: var(--font-ui); font-size: 12px; font-weight: 600; }}
                .theme-toggle:hover {{ border-color: var(--sl-accent); color: var(--sl-accent); }}
                .theme-toggle:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: 2px; }}
                .search-input:focus-visible, .search-btn:focus-visible, .retry-btn:focus-visible, .btn:focus-visible, .replay-input:focus-visible, .speed-input:focus-visible, .compare-btn:focus-visible, .sl-error-retry:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: 2px; }}
                .session-item:focus-visible, .feed-entry:focus-visible {{ outline: 2px solid {colors.focus}; outline-offset: -2px; }}
                .session-list {{ display: flex; flex-direction: column; height: 100%; }}
                .search-input {{ width: 100%; padding: 10px 16px; background: var(--sl-surface-muted); border: 1px solid var(--sl-border); border-radius: 6px; color: var(--sl-text); font-size: 13px; box-sizing: border-box; margin-bottom: 4px; }}
                .session-count {{ padding: 6px 20px; font-size: 11px; color: var(--sl-text-muted); }}
                .session-item {{ padding: var(--sl-space-md) var(--sl-space-xl); cursor: pointer; border-bottom: 1px solid var(--sl-border); transition: background var(--sl-motion-fast) var(--sl-ease-out); }}
                .session-item:hover {{ background: var(--sl-surface-muted); }}
                .session-item.selected {{ background: var(--sl-surface-muted); border-left: 3px solid var(--sl-accent); }}
                .session-source {{ font-size: 13px; font-weight: 600; color: var(--sl-text); }}
                .session-goal {{ font-size: 12px; color: var(--sl-text-muted); margin-top: 4px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
                .session-meta {{ font-size: 11px; color: var(--sl-text-muted); margin-top: 6px; display: flex; gap: 8px; align-items: center; }}
                .meta-bundles {{ color: var(--sl-accent); }}
                .badge {{ display: inline-block; padding: 1px 6px; border-radius: 4px; font-size: 10px; font-weight: 600; }}
                .badge-ok {{ background: color-mix(in srgb, var(--sl-accent-secondary) 18%, transparent); color: var(--sl-accent-secondary); }}
                .badge-contract {{ background: color-mix(in srgb, var(--sl-accent) 16%, transparent); color: var(--sl-accent); }}
                .search-view {{ display: flex; flex-direction: column; height: 100%; overflow-y: auto; }}
                .search-form {{ padding: 0 0 8px 0; border-bottom: 1px solid var(--sl-border); }}
                .search-fields {{ display: flex; flex-direction: column; gap: 4px; padding: 10px 16px; }}
                .search-label {{ font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: var(--sl-text-muted); }}
                .search-actions {{ display: flex; gap: 8px; padding: 8px 16px 10px; }}
                .search-btn {{ padding: 6px 16px; font-size: 12px; font-weight: 600; border-radius: 5px; cursor: pointer; border: 1px solid var(--sl-border); background: var(--sl-surface-muted); color: var(--sl-text-muted); }}
                .search-btn:hover {{ background: color-mix(in srgb, var(--sl-accent) 8%, transparent); color: var(--sl-text); }}
                .search-btn-primary {{ background: color-mix(in srgb, var(--sl-accent) 16%, transparent); color: var(--sl-accent); border-color: var(--sl-accent); }}
                .search-btn-primary:hover {{ background: color-mix(in srgb, var(--sl-accent) 24%, transparent); color: var(--sl-accent); }}
                .search-results {{ flex: 1; overflow-y: auto; }}
                .search-error {{ padding: 10px 16px; font-size: 13px; color: var(--sl-danger); background: var(--sl-danger-surface); border-bottom: 1px solid var(--sl-border); }}
                .live-feed {{ display: flex; flex-direction: column; height: 100%; }}
                .live-feed-header {{ display: flex; align-items: center; gap: var(--sl-space-md); padding: var(--sl-space-md) var(--sl-space-lg); border-bottom: 1px solid var(--sl-border); background: var(--sl-bg); }}
                .live-feed-title {{ font-size: 13px; font-weight: 600; color: var(--sl-text); flex: 1; }}
                .feed-status {{ font-size: 11px; font-weight: 600; }}
                .feed-status.live {{ color: var(--sl-accent-secondary); }}
                .feed-status.disconnected {{ color: var(--sl-danger); }}
                .feed-status.connecting {{ color: var(--sl-accent-warning); }}
                .retry-btn {{ padding: var(--sl-space-xs) var(--sl-space-md); font-size: 11px; font-weight: 600; background: var(--sl-surface-muted); border: 1px solid var(--sl-border); border-radius: var(--sl-radius-sm); color: var(--sl-text-muted); cursor: pointer; transition: background var(--sl-motion-fast) var(--sl-ease-out), color var(--sl-motion-fast) var(--sl-ease-out); }}
                .retry-btn:hover {{ background: color-mix(in srgb, var(--sl-accent) 8%, var(--sl-surface-muted)); color: var(--sl-text); }}
                .live-feed-list {{ flex: 1; overflow-y: auto; padding: var(--sl-space-sm) 0; }}
                .feed-empty {{ padding: var(--sl-space-lg) var(--sl-space-xl); font-size: 13px; color: var(--sl-text-muted); }}
                .feed-entry {{ display: flex; gap: var(--sl-space-md); align-items: baseline; padding: var(--sl-space-sm) var(--sl-space-lg); border-bottom: 1px solid var(--sl-border); font-family: var(--font-mono); transition: background var(--sl-motion-fast) var(--sl-ease-out); }}
                .feed-entry:hover {{ background: var(--sl-surface-muted); }}
                .feed-ts {{ font-size: 11px; color: var(--sl-text-muted); white-space: nowrap; }}
                .feed-path {{ font-size: 12px; color: var(--sl-accent); word-break: break-all; }}
                .compare-btn {{ padding: 2px var(--sl-space-sm); font-size: 10px; font-weight: 600; background: var(--sl-surface-muted); border: 1px solid var(--sl-border); border-radius: var(--sl-radius-sm); color: var(--sl-text-muted); cursor: pointer; margin-left: var(--sl-space-sm); transition: background var(--sl-motion-fast) var(--sl-ease-out), color var(--sl-motion-fast) var(--sl-ease-out); }}
                .compare-btn:hover {{ background: color-mix(in srgb, var(--sl-accent) 8%, var(--sl-surface-muted)); color: var(--sl-text); }}
                .compare-btn.active {{ background: color-mix(in srgb, var(--sl-accent) 16%, transparent); color: var(--sl-accent); border-color: var(--sl-accent); }}
                .diff-panel {{ border-top: 2px solid var(--sl-accent); background: var(--sl-bg); padding: 0; flex-shrink: 0; max-height: 340px; overflow-y: auto; }}
                .diff-header {{ display: flex; align-items: center; padding: var(--sl-space-md) var(--sl-space-lg); border-bottom: 1px solid var(--sl-border); background: var(--sl-bg); }}
                .diff-title {{ flex: 1; font-size: 13px; font-weight: 600; color: var(--sl-text); }}
                .diff-badge {{ display: inline-block; margin-left: var(--sl-space-sm); padding: 1px var(--sl-space-sm); border-radius: var(--sl-radius-pill); font-size: 11px; font-weight: 600; background: var(--sl-danger-surface); color: var(--sl-danger); }}
                .diff-badge-same {{ background: color-mix(in srgb, var(--sl-accent-secondary) 18%, transparent); color: var(--sl-accent-secondary); }}
                .diff-close {{ cursor: pointer; font-size: 14px; color: var(--sl-text-muted); padding: 2px var(--sl-space-sm); border-radius: var(--sl-radius-sm); transition: background var(--sl-motion-fast) var(--sl-ease-out), color var(--sl-motion-fast) var(--sl-ease-out); }}
                .diff-close:hover {{ background: var(--sl-surface-muted); color: var(--sl-text); }}
                .diff-col-headers {{ display: grid; grid-template-columns: 160px 1fr 1fr; padding: var(--sl-space-sm) var(--sl-space-lg); font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.4px; color: var(--sl-text-muted); border-bottom: 1px solid var(--sl-border); background: var(--sl-bg); }}
                .diff-rows {{ display: flex; flex-direction: column; }}
                .diff-row {{ display: grid; grid-template-columns: 160px 1fr 1fr; padding: var(--sl-space-sm) var(--sl-space-lg); font-size: 12px; border-bottom: 1px solid var(--sl-border); font-family: var(--font-mono); align-items: start; }}
                .diff-row-changed {{ background: color-mix(in srgb, var(--sl-danger) 6%, var(--sl-surface)); }}
                .diff-row-changed .diff-col-a {{ color: var(--sl-danger); }}
                .diff-row-changed .diff-col-b {{ color: var(--sl-accent-secondary); }}
                .diff-field-label {{ color: var(--sl-text-muted); font-weight: 600; font-family: var(--font-ui); font-size: 11px; padding-top: 1px; }}
                .diff-col-a {{ color: var(--sl-text); overflow-wrap: break-word; }}
                .diff-col-b {{ color: var(--sl-text); overflow-wrap: break-word; }}
                .main-content {{ flex: 1; display: flex; flex-direction: column; overflow: hidden; }}
                .main-upper {{ flex: 1; overflow-y: auto; }}
                .bundles-view {{ display: contents; }}
                .viewer-main {{ flex: 1; min-width: 0; width: 100%; overflow: hidden; }}
                .corpus-error-banner {{ padding: 0 8px; }}
                .corpus-error-banner .caption {{ display: block; margin-top: var(--sl-space-xs); }}
                @media (max-width: 600px) {{
                    .tab {{
                        min-height: 44px;
                        min-width: 44px;
                        padding: 12px 8px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        box-sizing: border-box;
                    }}
                    .theme-toggle {{
                        min-height: 44px;
                        padding: 12px 16px;
                        box-sizing: border-box;
                    }}
                    .search-btn, .retry-btn {{
                        min-height: 44px;
                        min-width: 44px;
                        padding: 10px 16px;
                        box-sizing: border-box;
                    }}
                    .detail, .timeline-detail, .wiki-page {{
                        padding: 16px;
                        max-width: 100%;
                        box-sizing: border-box;
                    }}
                    .diff-col-headers, .diff-row {{
                        grid-template-columns: minmax(72px, 96px) minmax(0, 1fr) minmax(0, 1fr);
                    }}
                    .replay-controls .btn {{
                        min-height: 44px;
                        min-width: 44px;
                        padding: 10px 16px;
                        box-sizing: border-box;
                    }}
                }}
                @media (min-width: 601px) {{
                    .app > .sidebar {{
                        width: 340px;
                        min-width: 340px;
                        max-width: 340px;
                        border-right: 1px solid var(--sl-border);
                    }}
                }}
                @media (prefers-reduced-motion: reduce) {{
                    *, *::before, *::after {{
                        animation-duration: 0.01ms !important;
                        animation-iteration-count: 1 !important;
                        transition-duration: 0.01ms !important;
                        scroll-behavior: auto !important;
                    }}
                    .launch-splash {{
                        animation: none;
                        opacity: 0;
                        visibility: hidden;
                        pointer-events: none;
                    }}
                }}
            "#,
        }
        div { class: "app",
            div { class: "launch-splash", role: "presentation",
                div { class: "launch-splash-inner",
                    span { class: "launch-splash-mark", "SessionLedger" }
                    span { class: "launch-splash-caption", "Viewer" }
                }
            }
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
                main {
                    class: "viewer-main",
                    div {
                        id: "{active_tab().panel_id()}",
                        role: "tabpanel",
                        "aria-labelledby": "{active_tab().id()}",
                        {tab_body}
                    }
                }
                // After tabpanel so Tab from the active tab reaches panel controls first.
                button {
                    class: "theme-toggle",
                    r#type: "button",
                    "aria-label": "Toggle light and dark theme",
                    onclick: move |_| {
                        let _ = document::eval(
                            r#"
                            const root = document.documentElement;
                            const current = root.dataset.theme === 'light' ? 'light' : 'dark';
                            const next = current === 'light' ? 'dark' : 'light';
                            root.dataset.theme = next;
                            window.localStorage.setItem('sl-viewer-theme', next);
                            "#,
                        );
                    },
                    "Toggle Theme"
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

    if loading() || query_fixture_active("skeleton") {
        return rsx! {
            h2 { "Compiled Bundles" }
            ContentSkeleton { layout: SkeletonLayout::Bundles }
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
        div {
            class: "bundles-view",
            onkeydown: move |evt: Event<KeyboardData>| {
                if evt.key() == Key::Escape && compare_idx().is_some() {
                    evt.prevent_default();
                    compare_idx.set(None);
                }
            },
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
                    span { style: "color:var(--sl-accent); margin-left:var(--sl-space-sm);", "compare slot active" }
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
