# SessionLedger — Visual Spec (Lab-Coat)

Status: acceptance reference for C10 / L99–L107.  
Implementation of viewer CSS/components is owned by other lanes (`crates/sl-viewer/src`); this document is the **contract** those surfaces must meet.

## Sources of truth

| Asset | Role |
|-------|------|
| [`assets/tokens.css`](../assets/tokens.css) | Canonical Lab-Coat hex tokens + `.family-lab-coat` aliases |
| [`assets/README-tokens.md`](../assets/README-tokens.md) | Adoption / re-sync notes |
| [`assets/brand/README.md`](../assets/brand/README.md) | Mark rationale + family differentiation |
| [`docs/assets/identity/README.md`](assets/identity/README.md) | Identity demo motion (SVG/MP4) |
| [`tests/visual/README.md`](../tests/visual/README.md) | Manual golden checklist |

Forbidden drift: MelosViz warn `#f59e0b`, Backbone-2 graphite/panel/violet/green, Tracera midnight/teal/indigo, and purple accent `#7c3aed` (legacy dark theme) must not appear as Lab-Coat brand colors.

---

## 1. Lab-Coat palette

Family: **Lab-Coat** (R&D). Light lab-bench base; cool cobalt primary; orange live pulse; teal secondary.

### Core tokens

| Token | Hex | Semantic role |
|-------|-----|---------------|
| `--lc-lab-white` | `#f6f8fa` | App / page background (lab coat) |
| `--lc-slate` | `#1f2937` | Primary text, dark panels, bench grid |
| `--lc-cobalt` | `#2563eb` | Primary accent — focus, links, selected, bundle bars |
| `--lc-orange` | `#f97316` | Live / in-progress — Bunsen burner |
| `--lc-teal` | `#14b8a6` | Secondary accent — success-adjacent, growth medium |

### Semantic aliases (required mapping)

| Alias | Maps to | Use |
|-------|---------|-----|
| `--bg-primary` | `--lc-lab-white` | Shell background |
| `--bg-panel` | `--lc-slate` | Inset panels (or slate-tinted surfaces) |
| `--accent-primary` | `--lc-cobalt` | CTAs, selection, focus ring |
| `--accent-secondary` | `--lc-teal` | Secondary actions, positive secondary |
| `--accent-warning` / live | `--lc-orange` | Live session, warning-adjacent live state |
| `--text-muted` | slate @ ~60% or `#5c5f6e` | Empty-state / helper copy (must remain ≥ 4.5:1 on lab-white) |
| `--status-error` | see §4 | Failure text/surfaces — **not** cobalt |

### Light vs dark

- **Brand default:** light Lab-Coat (`lab-white` canvas).
- **Dark panel mode** (viewer chrome): slate panels on lab-white, or inverted slate canvas with cobalt/teal/orange accents unchanged. Accent hexes stay Lab-Coat; do not invent a second brand palette.
- Theme API (`Theme::{Dark,Light}`) must resolve to these tokens once wired — no purple primary.

### Contrast floor

- Body text on `--lc-lab-white`: use `--lc-slate` (or darker).
- Cobalt / orange / teal on lab-white: large UI / icons OK; small body text prefers slate with accent for chrome only.
- Error text must meet WCAG AA against its background (§4).

---

## 2. Empty states

Empty ≠ error. Empty is calm, instructional, and non-alarming.

### Variants

| Variant | When | Copy pattern | Visual |
|---------|------|--------------|--------|
| **Unselected** | Pane open, no row chosen | “Select a … to …” | Centered muted text; no icon required |
| **Zero data** | List/query returned nothing | “No bundles to display” / equivalent | Optional low-contrast flask/eyepiece mark at ≤ 64px, muted |
| **First-run** | No corpus / never ingested | Short why + one primary CTA (“Open corpus…” / “Start daemon”) | Cobalt CTA; no orange (not “live”) |
| **Filtered empty** | Filters hide all rows | “No matches” + clear-filters control | Teal or cobalt text link; never error red |

### Rules

- Class / region: `empty-state` (or equivalent) with `role="status"` when content updates asynchronously.
- No card chrome, no badge clusters, no multi-stat strips in empty panes.
- Do not reuse `.search-error` / `.status-error` styling for empty.

### Surfaces (viewer)

| Surface | Expected empty |
|---------|----------------|
| Bundle detail | Unselected: select from list |
| History / timeline detail | Unselected session |
| Timeline / list | Zero data or filtered empty |
| Search results | Zero matches (filtered empty) |

---

## 3. Loading & skeleton states

Loading is temporary; it must not shift layout when content arrives (minimize CLS).

### Levels

| Level | Use | Spec |
|-------|-----|------|
| **Inline copy** | Short fetches (&lt; ~300ms typical) | “Loading…” / “Searching…” adjacent to control; muted text |
| **Busy control** | Button during request | Disable + label swap; keep width stable |
| **Content skeleton** | List / detail panes | Shimmer blocks matching final content shape (rows ≈ list row height; detail ≈ title + 3 lines) |

### Skeleton tokens (target)

| Token | Value | Notes |
|-------|-------|-------|
| `--lc-skeleton-base` | slate @ 8–12% on lab-white | Or `#e5e7eb`-class neutral on brand |
| `--lc-skeleton-highlight` | lab-white → translucent cobalt wash | Shimmer peak |
| Duration | `--lc-motion-slow` (see §5) | Looping gradient or opacity pulse |
| Shape | 4–6px radius | Match list/detail radii; no pills |

### Rules

- Prefer content-shaped skeletons over spinners for primary panes.
- Spinners, if used, are cobalt stroke on transparent; 1.2s linear rotate; hidden under `prefers-reduced-motion` (static cobalt arc or copy only).
- Never paint skeletons in error red or live orange.

---

## 4. Error & failure states

Errors are recoverable by default; show message + path forward.

### Anatomy

1. **Icon or label** — “Error” / alert affordance (a11y lane owns ARIA details).
2. **Message** — one sentence, human-readable; no raw stack in default UI.
3. **Action** — Retry and/or dismiss; cobalt for Retry, muted for dismiss.

### Color

| Role | Spec |
|------|------|
| Error foreground | Warm red that contrasts on panel, e.g. `#b91c1c` on lab-white or `#f87171` on dark slate panels |
| Error surface (optional) | Tinted wash (`#fef2f2` light / `#2a1a1a` dark) — not full-bleed alarm |
| Border | Subtle related red, 1px |

Do **not** use `--lc-orange` as the sole error signal (orange = live). Do **not** use cobalt for failure text.

### Surfaces

| Surface | Behavior |
|---------|----------|
| Search fetch failure | Inline `.search-error` region; clear on next successful search |
| Replay / SSE failure | `.status-error` + Retry |
| Corpus open failure | Blocking message + fallback to mock only in dev; production shows Retry |

---

## 5. Motion & reduced motion

### Motion tokens (target; may live in `tokens.css` later)

| Token | Duration | Easing | Use |
|-------|----------|--------|-----|
| `--lc-motion-fast` | 120–150ms | `ease-out` | Hover, tab underline, focus ring |
| `--lc-motion-medium` | 200–250ms | `ease-in-out` | Panel fade, skeleton pulse half-cycle |
| `--lc-motion-slow` | 1.5–2s | `ease-in-out` | Brand breathing (bundle bars, meniscus) |
| `--lc-motion-rise` | ~3.2s | `ease-in` | Identity bubble rise (demo / mark only) |

### Brand motion (reference)

Documented in identity demo / animated mark:

- Bubbles: ~3.2s rise, stagger ~0.4s  
- Bunsen / meniscus: ~1.8–4s opacity/scale breathe  
- Bundle bars: ~2s opacity breathe  

App chrome should stay at `--lc-motion-fast` / `--lc-motion-medium`; reserve slow loops for brand/live indicators.

### `prefers-reduced-motion: reduce`

When the user (or OS) requests reduced motion:

| Element | Behavior |
|---------|----------|
| CSS transitions | Duration → `0.01ms` or `none` |
| Skeleton shimmer | Static base fill; no gradient animation |
| Spinners | Static or replace with “Loading…” text |
| Live orange pulse | Solid `--lc-orange`; no opacity loop |
| Identity SMIL / demo loops | Pause on first frame or show static SVG |
| Scroll / route | No parallax; no entrance choreography |

Implement with a global guard:

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

(Viewer may scope this under `.app-root` instead of `*`.)

---

## 6. Composition & chrome (viewer)

- One job per pane: list **or** detail **or** search results — avoid dashboard card grids.
- Focus ring: 2px `--lc-cobalt` outline, offset 2px; visible on keyboard only if focus-visible is available.
- Live session badge: `--lc-orange` dot or label; must not animate under reduced motion.
- Selected row: cobalt left border or background wash (~8% cobalt), not purple.

---

## 7. Keyboard map (viewer)

The primary view selector follows the ARIA tabs keyboard pattern:

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Move focus into or out of the active tab and through controls in the current panel |
| `Left Arrow` / `Right Arrow` | Select the previous / next primary view, wrapping at either end |
| `Home` / `End` | Select the first / last primary view |
| `Enter` / `Space` | Activate the focused tab or focused button |

---

## 8. Acceptance (L107)

A change is visually accepted when:

1. Hex usage matches §1 (spot-check against `assets/tokens.css`).
2. Empty / loading / error match §§2–4 on the surfaces listed.
3. Reduced-motion behavior in §5 verified in OS or DevTools emulation.
4. Manual golden checklist in [`tests/visual/README.md`](../tests/visual/README.md) is checked (or automated harness, when present, is green).

Out of scope for this doc: packaging installers, a11y ARIA implementation details (a11y lane), and Rust theme module wiring (owned-repos / viewer lanes).
