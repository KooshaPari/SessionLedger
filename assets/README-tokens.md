# SessionLedger tokens.css adoption (vision-pillar L107 → per-repo)

This file documents how the SessionLedger repo adopts the shared
**Phenotype visual-identity palette library** from
`AgilePlus/.claude/worktrees/vision-pillar/assets/tokens.css`
(vendored here as `assets/tokens.css`).

## Source of truth

- Canonical: `AgilePlus/.claude/worktrees/vision-pillar/assets/tokens.css` (commit `5b95794`).
- Local copy: `assets/tokens.css` (this file's sibling).
- Drift guard: `AgilePlus/.claude/worktrees/vision-pillar/scripts/check-palette-overlap.sh` (10/10 PASS on current shipped state).

## Family selection for SessionLedger

Use the **Lab-Coat** family wrapper class (`.family-lab-coat`):

```css
@import url("./tokens.css");  /* or via Dioxus asset pipeline */

.app-root {
  composes: family-lab-coat;
  /* OR inline equivalents */
  --bg-primary: var(--lc-lab-white);
  --bg-panel: var(--lc-slate);
  --accent-primary: var(--lc-cobalt);
  --accent-secondary: var(--lc-teal);
  --accent-warning: var(--lc-orange);  /* #f97316 — was #f59e0b, swapped 2026-07-06 */
}
```

## Token quick-reference (Lab-Coat family, canonical as of commit 6afaac0)

| Token | Hex | Role |
|---|---|---|
| `--lc-lab-white` | `#f6f8fa` | Background / panel base |
| `--lc-slate` | `#1f2937` | Panel/text secondary |
| `--lc-cobalt` | `#2563eb` | Primary accent — slide-stain blue |
| `--lc-orange` | `#f97316` | Live-session indicator — lit Bunsen burner |
| `--lc-teal` | `#14b8a6` | Secondary — growth-medium teal |

## Status

- **File-copy phase (this commit)**: vendored tokens.css into `assets/`.
- **Wiring phase (next owned-repos pass)**: requires `pub mod theme;` in `crates/sl-viewer/src/lib.rs` + `pub use theme::*;` in lib.rs to make the Rust-side colors available. Then either:
  - Replace existing `ThemeColors::dark()` / `::light()` hex literals with `theme::lab_coat::*` constants, OR
  - Add a new `LabCoat` theme variant alongside existing Dark/Light.

## Cross-repo palette registry

The 5 documented families in `assets/tokens.css`:

1. **Backbone-2** — sharecli + substrate (infra)
2. **Lab-Coat** — SessionLedger (R&D)
3. **Terminal-Forge** — forgecode (terminal)
4. **Tracera** — pre-existing
5. **MelosViz** — pre-existing

All families are pairwise non-overlapping per the overlap verifier.

## Why this is file-only at this point

- Dioxus asset pipeline + Rust `theme.rs` module wiring are owned-repos scope (Rust edit).
- Vendoring tokens.css now means: when the wiring PR lands, the palette registry is already in the right place.
- No runtime behavior changes from this commit; it's purely a foundation commit.

## Re-syncing from canonical

When `AgilePlus/.../vision-pillar/assets/tokens.css` updates:

```bash
# Re-vendor:
cp /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/.claude/worktrees/vision-pillar/assets/tokens.css \
   /Users/kooshapari/CodeProjects/Phenotype/repos/SessionLedger/assets/tokens.css

# Re-verify overlap:
bash /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/.claude/worktrees/vision-pillar/scripts/check-palette-overlap.sh
```

References:
- Vision-pillar canonical: commit `5b95794` (tokens.css) + commit `5b01e1d` (amber-sync) + commit `04597ce` (overlap verifier)
- SessionLedger: commit `915a408` (initial iconset) + commit `8e68296` (amber swap) + this commit
- Lab-Coat palette decision: 2026-07-06, vision-pillar + melosviz-3d overlap check