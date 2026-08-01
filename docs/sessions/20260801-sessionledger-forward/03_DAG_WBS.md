# Forward DAG and Work Breakdown

## Dependency graph

```text
S0 preserve candidate + qgate green
 |\
 | +--> S1 reclaim disk / verify tools
 |       |
 |       +--> S2 build macOS bundle + install smoke
 |               |
 +--> S3 daemon discovery + parser + atomic OKF + SSE
         |       |
         +-------+--> S4 installed dogfood: inbox/detail/replay
                         |
                         +--> S5 performance, memory, restart, consent audit
                                 |
                                 +--> S6 sign, publish, rollback drill
                                         |
                                         +--> S7 release decision
```

## WBS and gate state

| ID | Work package | Exit evidence | State |
| --- | --- | --- | --- |
| S0 | Preserve branch, commits, hosted qgate | SHA `436e1618`; qgate 30679369252 | PASS |
| S1 | Remove only generated artifacts; confirm free space | `df`, clean worktree, no source loss | BLOCKED by ENOSPC |
| S2 | Build/package/install real `.app` | bundle path, launch, uninstall/rollback log | PENDING |
| S3 | Discover local roots and stream records | parser tests, OKF files, SSE events | PASS in focused proof |
| S4 | Exercise installed UI with real data | scroll/detail/chat/a11y capture | PENDING |
| S5 | Bound CPU/RSS/latency and restart behavior | repeatable benchmark report | PENDING |
| S6 | Generate checksums, signatures, release notes | verification and rollback drill | PENDING |
| S7 | Sponsor go/no-go | all blocking gates green | HOLD |

## Critical path

`S1 -> S2 -> S4 -> S5 -> S6 -> S7` is critical. S3 can proceed in parallel,
but it cannot substitute for installed-app evidence. Any failed gate loops to
the owning package with a focused test and an updated session note.

## Immediate next actions

1. Reclaim candidate-generated disk space and record before/after capacity.
2. Run the real macOS bundle/install smoke; preserve logs and checksums.
3. Point the installed daemon at auto-discovered local roots and verify one
   end-to-end session through OKF, SSE, inbox, and replay.
