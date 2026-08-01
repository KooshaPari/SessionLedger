use std::cell::RefCell;

use dioxus::prelude::*;
use session_ledger::domain::session::Session;
use sl_viewer::app::SessionContext;
use sl_viewer::corpus_loader::{load_sessions, DataSource};
use sl_viewer::history_tab::HistoryTimeline;
use sl_viewer::memory_tab::MemoryWiki;

thread_local! {
    static SESSION_SIGNAL: RefCell<Option<Signal<Vec<Session>>>> = const { RefCell::new(None) };
}

#[component]
fn MemoryHarness() -> Element {
    let sessions = use_signal(Vec::<Session>::new);
    use_context_provider(|| SessionContext(sessions));
    SESSION_SIGNAL.with(|slot| *slot.borrow_mut() = Some(sessions));

    rsx! { MemoryWiki {} }
}

#[test]
fn memory_wiki_rerenders_when_async_context_load_completes() {
    let mut dom = VirtualDom::new(MemoryHarness);
    let _initial = dom.rebuild_to_vec();

    let sessions = load_sessions(&DataSource::Mock).expect("mock corpus loads");
    SESSION_SIGNAL.with(|slot| {
        slot.borrow().expect("harness captured signal").set(sessions);
    });

    let update = dom.render_immediate_to_vec();
    assert!(
        format!("{update:?}").contains("Login timeout fix"),
        "memory wiki did not render sessions delivered after initial mount: {update:?}"
    );
}

#[component]
fn HistoryHarness() -> Element {
    let sessions = use_signal(Vec::<Session>::new);
    use_context_provider(|| SessionContext(sessions));
    SESSION_SIGNAL.with(|slot| *slot.borrow_mut() = Some(sessions));

    rsx! { HistoryTimeline {} }
}

#[test]
fn history_timeline_rerenders_when_async_context_load_completes() {
    let mut dom = VirtualDom::new(HistoryHarness);
    let _initial = dom.rebuild_to_vec();

    let sessions = load_sessions(&DataSource::Mock).expect("mock corpus loads");
    SESSION_SIGNAL.with(|slot| {
        slot.borrow().expect("harness captured signal").set(sessions);
    });

    let update = dom.render_immediate_to_vec();
    assert!(
        format!("{update:?}").contains("Login timeout fix"),
        "history timeline did not render sessions delivered after initial mount: {update:?}"
    );
}
