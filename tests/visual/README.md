# Visual golden checklist (SessionLedger)

Manual acceptance for [`docs/VISUAL_SPEC.md`](../../docs/VISUAL_SPEC.md) (Lab-Coat / L99–L107).  
Automated screenshot harness is **optional**; until it lands, run this checklist before merging viewer visual changes.

## Prerequisites

- Viewer running against a known fixture corpus (or mock data).
- Display scale 100%; window ≥ 1280×720 for desktop shots.
- Tokens reference: `assets/tokens.css` Lab-Coat block.

## Palette lock-in

- [ ] Canvas / page background reads as lab-white `#f6f8fa` (or documented dark slate panel mode — accents still Lab-Coat).
- [ ] Primary accent / selection / focus uses cobalt `#2563eb`.
- [ ] Live / in-progress indicator uses orange `#f97316` (not `#f59e0b`).
- [ ] Secondary accent uses teal `#14b8a6` where applicable.
- [ ] No purple `#7c3aed`, Backbone-2, Tracera, or MelosViz brand hexes in chrome.

## Empty states

Capture or eyeball each surface (store under `tests/visual/golden/` if saving PNGs):

| ID | Surface | Expect |
|----|---------|--------|
| E1 | Bundle detail, nothing selected | Muted “Select a bundle…” empty-state; not error styling |
| E2 | History detail, nothing selected | Muted “Select a session…” empty-state |
| E3 | Timeline / list with zero rows | “No bundles…” (or equivalent); calm, not red |
| E4 | Search with no matches | Zero-match copy + clear path; not `.search-error` |

- [ ] E1–E4 pass copy + color rules in VISUAL_SPEC §2.

## Loading states

| ID | Surface | Expect |
|----|---------|--------|
| L1 | Search in flight | “Searching…” / “Loading…”; control not layout-shifting wildly |
| L2 | List/detail skeleton (when implemented) | Content-shaped placeholders; cobalt-neutral shimmer, not orange/red |

- [ ] L1 pass; L2 pass or N/A if skeletons not yet shipped.

## Error states

| ID | Surface | Expect |
|----|---------|--------|
| R1 | Search fetch failure | Error region + readable message; Retry or recoverable path |
| R2 | Replay / stream failure | `.status-error` (or equivalent) + Retry |
| R3 | Error color | Warm red on panel — **not** live orange alone |

- [ ] R1–R3 pass VISUAL_SPEC §4.

## Motion / reduced motion

- [ ] Default: hover/tab transitions ≤ ~150ms; no gratuitous loops on chrome.
- [ ] Live indicator may breathe; under **prefers-reduced-motion: reduce**, pulse stops (solid orange / static).
- [ ] Skeletons / spinners do not animate under reduced motion (static or text-only).
- [ ] Identity demo / animated mark: static first frame or paused when reduced motion is on (docs/demo surfaces).

Emulate in Chromium DevTools → Rendering → Emulate CSS media feature `prefers-reduced-motion: reduce`.

## Optional screenshot harness

When adding automation, prefer Playwright (or similar) against a fixed fixture URL:

```text
tests/visual/
  README.md          ← this file
  golden/            ← baseline PNGs (optional)
    e1-bundle-empty.png
    e2-history-empty.png
    e3-timeline-zero.png
    e4-search-empty.png
    l1-search-loading.png
    r1-search-error.png
  harness/           ← optional scripts (not required for Wave-1)
```

Suggested compare tolerance: ≤ 0.1% pixel diff on non-AA regions; ignore OS window chrome.

Stub directory `golden/` is reserved for baselines; commit PNGs only when a harness exists and CI can refresh them intentionally.

## Related checklists

- Icon / mark palette: [`assets/icons/CHECKLIST.md`](../../assets/icons/CHECKLIST.md)
- Identity demo media: [`docs/assets/identity/README.md`](../../docs/assets/identity/README.md)
