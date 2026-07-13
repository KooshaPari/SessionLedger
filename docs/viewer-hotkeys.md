# Viewer Hotkeys and Keyboard Navigation

Status: user-facing keyboard contract for the current `sl-viewer` surface.

## Current Shortcuts

| Shortcut | Scope | Behavior |
|----------|-------|----------|
| `Tab` / `Shift+Tab` | Whole viewer | Move through the browser or WebView focus order. The active view tab is the only tab stop in the tablist before focus enters the active panel controls. |
| `ArrowRight` | Focused view tab | Select and focus the next view tab, wrapping from Replay to Bundles. |
| `ArrowLeft` | Focused view tab | Select and focus the previous view tab, wrapping from Bundles to Replay. |
| `Home` | Focused view tab | Select and focus Bundles. |
| `End` | Focused view tab | Select and focus Replay. |
| `Enter` / `Space` | Focused view tab | Activate the focused view tab. |
| `Escape` | Search view | Clear search filters, results, and errors without moving focus. |
| `Escape` | Replay view | Clear replay output and return the replay panel to idle. |
| `Escape` | Bundle comparison panel | Close the comparison panel. |

The current view order is Bundles, History, Unfinished, Memory, Live Feed,
Search, Timeline, and Replay.

## Planned Efficiency Aids

`Cmd+K` on macOS and `Ctrl+K` on Windows/Linux are reserved for a future command
or quick-focus overlay. Until that overlay ships, use the Search tab and normal
tab navigation to reach filters and panel controls.

The first overlay should include copy for:

- Jump to view by name.
- Focus search filters.
- Focus replay bundle ID.
- Open this keyboard shortcut map.

## Test Evidence

`tests/visual/harness/a11y.spec.js` runs against the built Dioxus web viewer and
covers the ARIA tab keyboard pattern, active-tab tab order, and Search
`Escape`-to-clear behavior. The same component source backs the desktop viewer,
with native WebView and OS chrome remaining outside that browser harness.
