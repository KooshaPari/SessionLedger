# Cross-language OKF fixture parity (SSOT)

Status: **C08 L75 scaffold** — documents how SessionLedger keeps OKF
conformance fixtures aligned across programming-language shapes already present
in the corpus (Python, TypeScript, Go). This is **fixture parity**, not a
multi-language runtime SDK and **not** Harbor / agent-eval.

Related: [`docs/EVAL_SCOPE.md`](../EVAL_SCOPE.md) (eval boundary),
[`docs/reference/conformance/README.md`](../reference/conformance/README.md)
(corpus layout), [`eval-manifest.json`](eval-manifest.json) (fixture anchors).

## What "parity" means here

For each language row in the matrix below, SessionLedger maintains at least one
hand-vetted `.okf.json` fixture whose `source_id` (and filename stem) embeds a
stable **language tag**. Consumers that parse OKF in any host language should
be able to load these fixtures and apply the same conformance checks described
in the corpus README (parse → validate → round-trip).

Parity does **not** require:

- Native Python / TypeScript / Go SessionLedger ports
- Per-language agent harnesses or Harbor env providers
- Identical toolchains or acceptance-command strings across fixtures

## Parity matrix

| Language | Tag (in `source_id`) | Anchor fixture | Corpus | Notes |
|----------|----------------------|----------------|--------|-------|
| Python | `python` | [`cursor-python-029.okf.json`](../reference/conformance/fixtures/cursor-python-029.okf.json) | cursor | pytest / mypy acceptance labels |
| TypeScript | `typescript` | [`codex-typescript-023.okf.json`](../reference/conformance/fixtures/codex-typescript-023.okf.json) | codex | npm / eslint acceptance labels |
| Go | `go` | [`forge-go-module-026.okf.json`](../reference/conformance/fixtures/forge-go-module-026.okf.json) | forge | `go test` / gRPC health acceptance |

These three fixtures are also pinned in [`eval-manifest.json`](eval-manifest.json)
`fixture_anchors`. Additional languages may be appended later; do not remove
rows without a major corpus decision.

### Fixture language-tag rule

For every matrix row:

1. File exists under `docs/reference/conformance/fixtures/<source_id>.okf.json`.
2. Top-level JSON `source_id` equals the filename stem.
3. `source_id` contains the language **tag** substring from the matrix
   (case-sensitive: `python`, `typescript`, or `go`).
4. Document-level `provenance.source_id` matches top-level `source_id` when
   provenance is present (OKF v1.0 structural rule).

## Explicit non-goals

| Item | Status | Rationale |
|------|--------|-----------|
| Harbor / Portage / Terminal-Bench | **N/A** | Product boundary — see [`EVAL_SCOPE.md`](../EVAL_SCOPE.md) |
| Multi-env agent scoring | **N/A** | SessionLedger is ingest → distill → OKF → view |
| Shipping language SDKs | **out of scope** | Parity is fixture/SSOT evidence only |

## Done gates

| Gate | Status | Command |
|------|--------|---------|
| Cross-language parity SelfCheck | **done** | `scripts/cross-language-parity-check.ps1 -SelfCheck` |

## Machine verification (SelfCheck)

Hermetic doc + fixture path check (no daemon, no network, no cargo):

```powershell
pwsh ./scripts/cross-language-parity-check.ps1 -SelfCheck
```

`-SelfCheck` asserts this page keeps the parity matrix, language-tag rule,
EVAL_SCOPE / Harbor N/A boundary, and that each matrix fixture exists with a
matching `source_id` language tag. `tests/cross_language_parity.rs` wraps the
same command for optional `cargo test` proof.

CI: [`.github/workflows/eval-compression.yml`](../../.github/workflows/eval-compression.yml)
runs the SelfCheck on PR/push (blocking, hermetic).
