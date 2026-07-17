# Wave-38 lane: w38-i18n-fluent — C01 L16 Fluent catalog stub

**Branch:** `feat/sl-w38-i18n-fluent`
**Worktree:** `C:\Users\koosh\SessionLedger-wtrees\w38-i18n-fluent`
**Cluster / pillar:** C01 L16 (i18n scaffold — Fluent Phase-1)

## Gap

Phase-0 ships JSON catalogs only. Add a minimal optional Fluent `.ftl` catalog
stub + `fluent-catalog` Cargo feature without migrating the viewer or requiring
Fluent in default builds.

## Acceptance criteria

1. `locales/en.ftl` + `locales/es.ftl` mirror JSON keys (hyphenated message ids).
2. Optional `fluent-catalog` feature (`fluent-bundle`, `unic-langid`) in root `Cargo.toml`.
3. `src/i18n_fluent.rs` — `t_fluent(key, locale)` with JSON fallback when feature off.
4. `src/lib.rs` — `mod i18n_fluent`.
5. `scripts/fluent-i18n-check.ps1 -SelfCheck` — FTL existence, JSON key parity, doc anchors.
6. `tests/fluent_i18n.rs` SelfCheck wrapper.
7. `docs/ops/i18n.md` Phase-1 section. **Do not edit** audit scorecard files.

## Verify

```powershell
pwsh ./scripts/fluent-i18n-check.ps1 -SelfCheck
cargo test --lib i18n_fluent
cargo test --features fluent-catalog --lib i18n_fluent
```

## Score expectation

Evidence toward L16 Fluent hook; full viewer/CLI migration remains deferred.
