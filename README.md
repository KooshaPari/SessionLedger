# SessionLedger

> Compile, distill, and **resume** agent sessions — losslessly.

SessionLedger is a session-ledger / observability + memory-distillation system for
agent sessions (Forge, Codex, Claude Code, Cursor, …). It:

- **Compiles** any agent session into a structured, observable ledger (OKF-style).
- **Distills** ("dream") a session into short-term + long-term memory stores.
- Serves a **wiki / docs / history viewer** over sessions.
- Produces a canonical, **injectable continuation bundle** for lossless resume:
  an `Acceptance` gate + `Contract` + `Context` + `Intent` (+ `Provenance`,
  `Worklog`, `Dedup`) bundle you can inject into a NEW session.

### Why

1. **Merge duplicate-scoped chats** — after a crash, resume ONE chat, not many.
2. **Recover lost work** from crashed/abandoned sessions into an
   *in-progress / unfinished* section.
3. General session **observability + history**.

## Architecture (hexagonal)

```
ingestion/   per-corpus adapters → normalized Session       (ports::CorpusSource)
domain/      bundle model · intent FSM · dedup keys · session  (pure, no I/O)
distill/     "dream" → ContinuationBundle + memory writes     (ports::MemoryStore)
viewer/      wiki / history read model
ports/       trait boundaries composed from existing Phenotype systems
```

SessionLedger **composes, never duplicates**: forgecode (lifecycle FSM + zstd +
ADR-103 pruning), OmniRoute memory (FTS5 + Qdrant), context-mode / omni-context-rtk
(compression), pheno-tracing / PhenoObservability (observability). See
[`docs/DESIGN.md`](docs/DESIGN.md).

## Build

```bash
cargo build
cargo test
cargo clippy --all-targets   # warnings denied
```

Language: **Rust** (Phenotype scripting-policy default; composition targets are Rust).

## License

Dual-licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE).
