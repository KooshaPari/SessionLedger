//! Live SSE feed panel — streams OKF bundle paths from `sl-daemon`.
//!
//! Connects to `http://localhost:9001/api/stream` and displays the last 20
//! incoming bundle paths with a connection-status indicator.

use dioxus::prelude::*;

use crate::async_states::{ContentSkeleton, ErrorState, SkeletonLayout};
use crate::fixture::query_fixture_active;

#[cfg_attr(any(target_arch = "wasm32", not(feature = "desktop")), allow(dead_code))]
const DAEMON_SSE_URL: &str = "http://localhost:9001/api/stream";
#[cfg_attr(any(target_arch = "wasm32", not(feature = "desktop")), allow(dead_code))]
const MAX_ENTRIES: usize = 20;

/// Connection status for the SSE feed.
#[derive(Debug, Clone, PartialEq)]
enum FeedStatus {
    #[cfg_attr(any(target_arch = "wasm32", not(feature = "desktop")), allow(dead_code))]
    Live,
    Disconnected,
    Connecting,
}

/// A single entry in the feed ring-buffer.
#[derive(Debug, Clone, PartialEq)]
struct FeedEntry {
    path: String,
    /// Wall-clock seconds since UNIX epoch (approximate; from js Date or
    /// instant in native builds).
    timestamp: String,
}

/// Live SSE feed panel component.
///
/// Shows a scrollable list of the last 20 OKF bundle paths received from
/// `sl-daemon`, plus a status badge and a retry button when disconnected.
#[component]
pub fn LiveFeed() -> Element {
    #[allow(unused_mut)]
    let mut entries: Signal<Vec<FeedEntry>> = use_signal(Vec::new);
    let mut status: Signal<FeedStatus> = use_signal(|| FeedStatus::Disconnected);
    let mut trigger_connect: Signal<u32> = use_signal(|| 0u32);

    // Spawn a background coroutine that reads the SSE stream.
    // Re-spawned each time `trigger_connect` increments (retry button).
    let _feed_task = use_coroutine(move |_rx: UnboundedReceiver<()>| {
        let _version = trigger_connect();
        async move {
            status.set(FeedStatus::Connecting);

            #[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
            {
                use tokio::io::{AsyncBufReadExt, BufReader};

                let client = reqwest::Client::new();
                let resp = match client.get(DAEMON_SSE_URL).send().await {
                    Ok(r) => r,
                    Err(_) => {
                        status.set(FeedStatus::Disconnected);
                        return;
                    }
                };

                status.set(FeedStatus::Live);
                let stream = resp.bytes_stream();
                use futures_util::StreamExt;
                use tokio_util::io::StreamReader;

                let stream = stream.map(|r| r.map_err(std::io::Error::other));
                let reader = StreamReader::new(stream);
                let mut lines = BufReader::new(reader).lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    let line = line.trim().to_owned();
                    if let Some(path) = line.strip_prefix("data:") {
                        let path = path.trim().to_owned();
                        if path.is_empty() {
                            continue;
                        }
                        let ts = chrono::Local::now().format("%H:%M:%S").to_string();
                        entries.with_mut(|v| {
                            if v.len() >= MAX_ENTRIES {
                                v.remove(0);
                            }
                            v.push(FeedEntry { path, timestamp: ts });
                        });
                    }
                }
                status.set(FeedStatus::Disconnected);
            }

            #[cfg(any(target_arch = "wasm32", not(feature = "desktop")))]
            {
                // Web/no-native builds: use browser EventSource via gloo-net or
                // web-sys once wired. For now, mark as disconnected.
                let _ = entries;
                status.set(FeedStatus::Disconnected);
            }
        }
    });

    let fixture_stream_skeleton = query_fixture_active("stream-skeleton");
    let status_val = if fixture_stream_skeleton { FeedStatus::Connecting } else { status() };
    let (status_label, status_class, status_aria) = match &status_val {
        FeedStatus::Live => ("● Live", "feed-status live", "Live feed connected"),
        FeedStatus::Disconnected => {
            ("○ Disconnected", "feed-status disconnected", "Live feed disconnected")
        }
        FeedStatus::Connecting => {
            ("◌ Connecting…", "feed-status connecting", "Connecting to bundle feed")
        }
    };

    let feed_entries = if fixture_stream_skeleton { Vec::new() } else { entries() };

    rsx! {
        div {
            class: "live-feed",
            "data-testid": "live-feed-root",
            div { class: "live-feed-header",
                span { class: "live-feed-title", "Live Feed" }
                span {
                    class: "{status_class}",
                    role: "status",
                    "aria-live": "polite",
                    "aria-label": "{status_aria}",
                    "data-testid": "live-feed-status",
                    "{status_label}"
                }
                if status_val == FeedStatus::Disconnected {
                    button {
                        class: "retry-btn",
                        "data-testid": "live-feed-retry",
                        onclick: move |_| {
                            trigger_connect.with_mut(|v| *v += 1);
                        },
                        "Retry"
                    }
                }
            }
            div {
                class: "live-feed-list",
                "data-testid": "live-feed-list",
                if status_val == FeedStatus::Connecting {
                    ContentSkeleton { layout: SkeletonLayout::StreamFeed, list_rows: 5 }
                } else if status_val == FeedStatus::Disconnected && feed_entries.is_empty() {
                    ErrorState {
                        message: "Live feed disconnected — daemon unreachable at localhost:9001.".to_string(),
                        retryable: true,
                        on_retry: move |_| {
                            trigger_connect.with_mut(|v| *v += 1);
                        },
                    }
                } else if feed_entries.is_empty() {
                    div {
                        class: "feed-empty",
                        "data-testid": "live-feed-empty",
                        "Waiting for bundle events…"
                    }
                }
                for entry in feed_entries.iter().rev() {
                    div {
                        class: "feed-entry",
                        "data-testid": "live-feed-entry",
                        "data-bundle-path": "{entry.path}",
                        span { class: "feed-ts", "{entry.timestamp}" }
                        span { class: "feed-path",
                            // Show only the filename for readability.
                            {
                                std::path::Path::new(&entry.path)
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(&entry.path)
                            }
                        }
                    }
                }
            }
        }
    }
}
