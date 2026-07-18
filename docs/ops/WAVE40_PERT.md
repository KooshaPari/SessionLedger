# Wave-40 PERT — SessionLedger (394/402 → target 396+/402)

**Tip:** `ab29885` · **Width:** 5 parallel lanes · **Reaudit:** after lane merges

## Nodes

| ID | Task | Predecessors | Owner lane |
|----|------|--------------|------------|
| W40-S | Scope PR (`WAVE40_SCOPE.md`) | — | scope |
| W40-E | eval-repro manifest hard sync | W40-S | w40-eval-repro |
| W40-G | signing-readiness blocking gate | W40-S | w40-signing-hard |
| W40-U | update-check CLI/script | W40-S | w40-update-check |
| W40-R | rootless runner matrix scaffold | W40-S | w40-rootless-matrix |
| W40-D | daemon tokio broadcast loom | W40-S | w40-daemon-tokio |
| W40-M | Sequential merges to main | W40-E..D | orchestrator |
| W40-A | Reaudit PR (#322 pattern) | W40-M | reaudit |

## Critical path (estimated)

`W40-S → W40-E → W40-G → W40-U → W40-M → W40-A`

Eval-repro first (CI flake + manifest drift); signing/update-check are strongest score candidates (+2 raw).

## Merge order

1. eval-repro (#321 class fix)  
2. signing-hard  
3. update-check  
4. rootless-matrix  
5. daemon-tokio  
