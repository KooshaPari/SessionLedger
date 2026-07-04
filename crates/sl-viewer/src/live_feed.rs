//! Live SSE feed panel — streams OKF bundle paths from `sl-daemon`.
//!
//! Connects to `http://localhost:9001/api/stream` and displays the last 20
//! incoming bundle paths with a connection-status indicator.

use dioxus::prelude::*;

const DAEMON_SSE_URL: &str = "http://localhost:9001/api/stream";
const MAX_ENTRIES: usize = 20;

/// Connection status for the SSE feed.
#[derive(Debug, Clone, PartialEq)]
enum FeedStatus {
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
    let mut entries: Signal<Vec<FeedEntry>> = use_signal(Vec::new);
    let mut status: Signal<FeedStatus> = use_signal(|| FeedStatus::Disconnected);
    let mut trigger_connect: Signal<u32> = use_signal(|| 0u32);

    // Spawn a background coroutine that reads the SSE stream.
    // Re-spawned each time `trigger_connect` increments (retry button).
    let _feed_task = use_coroutine(move |_rx: UnboundedReceiver<()>| {
        let _version = trigger_connect();
        async move {
            status.set(FeedStatus::Connecting);

            #[cfg(not(target_arch = "wasm32"))]
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

                let stream = stream.map(|r| {
                    r.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                });
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

            #[cfg(target_arch = "wasm32")]
            {
                // Web: use browser EventSource via gloo-net or web-sys.
                // For now, mark as disconnected with a note.
                let _ = entries;
                status.set(FeedStatus::Disconnected);
            }
        }
    });

    let status_val = status();
    let (status_label, status_class) = match &status_val {
        FeedStatus::Live => ("● Live", "feed-status live"),
        FeedStatus::Disconnected => ("○ Disconnected", "feed-status disconnected"),
        FeedStatus::Connecting => ("◌ Connecting…", "feed-status connecting"),
    };

    let feed_entries = entries();

    rsx! {
        div { class: "live-feed",
            div { class: "live-feed-header",
                span { class: "live-feed-title", "Live Feed" }
                span { class: "{status_class}", "{status_label}" }
                if status_val == FeedStatus::Disconnected {
                    button {
                        class: "retry-btn",
                        onclick: move |_| {
                            trigger_connect.with_mut(|v| *v += 1);
                        },
                        "Retry"
                    }
                }
            }
            div { class: "live-feed-list",
                if feed_entries.is_empty() {
                    div { class: "feed-empty", "Waiting for bundle events…" }
                }
                for entry in feed_entries.iter().rev() {
                    div { class: "feed-entry",
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
