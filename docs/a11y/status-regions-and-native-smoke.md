# Status regions, cognitive copy, and native WebView smoke

Machine-claimable accessibility evidence for SessionLedger viewer **status visibility**
(L81.7), **inclusive help copy** (L81.10), and **keyboard efficiency** (L81.14).

## Automated contract (CI)

| Check | Location | What it proves |
|-------|----------|----------------|
| Status region ARIA | `tests/visual/harness/a11y.spec.js` | `role=status` / `role=alert`, `aria-live`, `aria-busy` on fixture-driven loading, skeleton, and error surfaces |
| Long-operation patience hint | `/?fixture=loading-long` + Playwright | Plain-language ETA-style copy for operations that may exceed ~10s |
| Stream skeleton status | `/?fixture=stream-skeleton` + Playwright | Live Feed connecting state exposes content-shaped skeleton + labelled status badge |
| Reduced motion | `a11y.spec.js` | Spinner animation flattened under `prefers-reduced-motion: reduce` (global guard in `app.rs`) |
| Landmarks | `a11y.spec.js` | `navigation` + `main` landmarks on every production tab |
| Help golden snapshot | `tests/fixtures/a11y/help_shortcuts.golden.tsv` + `help_overlay.rs` unit test | Keyboard-help table stays in sync with shipped overlay copy |
| Inclusive language seed | `scripts/inclusive-language-check.ps1` in `.github/workflows/a11y.yml` | Docs + viewer user-facing strings avoid deny-list terms |

Build the web viewer before local runs:

```powershell
cd crates/sl-viewer
dx build --platform web --release --no-default-features --features web
cd ../../tests/visual/harness
npm ci
npx playwright test a11y.spec.js
```

## Visual fixtures

| Query | Tab | Surface exercised |
|-------|-----|-------------------|
| `/?fixture=skeleton` | Bundles | `ContentSkeleton` (`aria-busy`, `aria-label`) |
| `/?fixture=loading-long` | Bundles | `LoadingState` + patience hint for long loads |
| `/?fixture=search-error` | Search | `ErrorState` (`role=alert`, Retry) |
| `/?fixture=stream-skeleton` | Live Feed | Stream skeleton + `aria-label` status badge |

## Native WebView smoke checklist (manual, ~5 min)

Run the desktop viewer (`cargo run -p sl-viewer` with the desktop feature) and tick each item.
OS window chrome is outside the browser harness; this checklist captures what auditors should verify on the shipped shell.

1. **Landmarks** — Tab to the sidebar: VoiceOver/NVDA announces “Primary viewer navigation”. Tab into the main panel: “main” region is reachable.
2. **Status while loading** — With daemon stopped, open **Live Feed**: hear or see “Connecting…” status; skeleton blocks appear without layout jump.
3. **Error recovery** — Trigger a Search failure (daemon down): alert copy is plain language (“Something went wrong” + detail), Retry is keyboard-focusable.
4. **Reduced motion** — Enable OS “reduce motion”, relaunch viewer: tab transitions and loading spinner do not animate distractingly.
5. **Keyboard efficiency** — Press `?` (outside a text field): help overlay opens with shortcut table; `Escape` closes and focus returns to **Help (?)**.
6. **Search escape hatch** — On Search tab, type in a filter, press `Escape`: fields clear, focus stays in the filter.

Record pass/fail and build ID in your audit worksheet. Do not claim Authenticode or platform signing here — see packaging ADR 0003 for the portable trust model.

## Related references

- [`docs/viewer-hotkeys.md`](../viewer-hotkeys.md) — keyboard contract
- [`docs/HELP.md`](../HELP.md) — operator help
- [`docs/VISUAL_SPEC.md`](../VISUAL_SPEC.md) §3–5 — loading, skeleton, and error anatomy
