# Visual Golden Provenance

These PNG baselines are approved browser-rendered captures for the Playwright
visual contract in `tests/visual/harness/visual.spec.js`.

| Golden | Screen | Route / state | Viewport | Motion | Notes |
|--------|--------|---------------|----------|--------|-------|
| `golden/e1-bundle-empty.png` | Bundles tab, detail pane empty state | `/` with the default mock corpus and no bundle selected | 1280x720 Chromium | `prefers-reduced-motion: reduce` | Locks VISUAL_SPEC E1: the muted "Select a bundle..." empty detail state is distinct from error styling. |

Refresh with:

```powershell
cd tests/visual/harness
npm run test:visual -- --update-snapshots
```
