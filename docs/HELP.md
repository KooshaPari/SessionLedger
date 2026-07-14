# SessionLedger Viewer Help

Short reference for the in-viewer keyboard help overlay, command palette, and related docs.

## Open help in the viewer

| Action | Result |
|--------|--------|
| Press `?` | Toggle the keyboard help overlay (ignored while focus is in a text field). |
| Click **Help (?)** | Open or close the same overlay from the sidebar. |
| Press `Escape` | Close the overlay and return focus to the Help control. |

The overlay is an accessible dialog (`role="dialog"`, `aria-modal="true"`) listing the
current viewer shortcuts. It is implemented in `crates/sl-viewer/src/help_overlay.rs`.

## Command palette

| Action | Result |
|--------|--------|
| Press `Cmd+K` / `Ctrl+K` | Toggle the command palette (works in text fields too). |
| Press `Escape` | Close the palette. |

The palette ships six commands — focus search, open keyboard help, next/previous
view tab, clear search, and toggle theme — in
`crates/sl-viewer/src/command_palette.rs`.

## Related references

- [`viewer-hotkeys.md`](viewer-hotkeys.md) — canonical shortcut table and test evidence.
- [`guides/quick-start/QUICKSTART.md`](guides/quick-start/QUICKSTART.md) — first-run stack setup.

## Automated checks

`tests/visual/harness/a11y.spec.js` asserts that `?` opens help, `Ctrl+K` opens the
command palette, and `Escape` closes either overlay against the production Dioxus web build.
