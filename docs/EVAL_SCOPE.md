# Eval Scope — SessionLedger

Explicit decision record for what “eval” means in this repo, and what is
intentionally out of product scope. Closes the soft-goal direction in
[issue #68](https://github.com/KooshaPari/SessionLedger/issues/68) (C08 eval
corpus depth).

---

## In scope (this product)

SessionLedger is a **session ingest → distill → OKF → view** pipeline. Eval
work that belongs here is limited to:

| Surface | What we maintain | Where |
|---------|------------------|-------|
| OKF conformance corpus | Hand-vetted `.okf.json` fixtures for parsers / validators / renderers | [`docs/reference/conformance/fixtures/`](reference/conformance/fixtures/) |
| Spec + examples | Structural rules and worked shapes | [`OKF-SPEC.md`](reference/OKF-SPEC.md), [`OKF-EXAMPLES.md`](reference/OKF-EXAMPLES.md) |
| Round-trip / unit tests | Compile and parse assertions against fixtures | `tests/`, crate unit tests |
| Quality gates | Coverage / lint / mutation as configured | `.qgate.toml`, CI |

These surfaces verify that SessionLedger emits and consumes valid OKF. They
are **not** multi-environment agent benchmarks.

---

## Intentional N/A — Harbor / agent-eval pipeline

| Item | Status | Rationale |
|------|--------|-----------|
| Harbor env providers | **N/A** | SessionLedger is not an agent-eval harness. |
| Portage / Terminal-Bench (or similar 2+/6-env agent-eval pipelines) | **N/A** | Product roadmap is ingest→distill→view ([`DESIGN.md`](DESIGN.md)); multi-env agent scoring is a different product class. |
| Per-eval / per-route token-burn ledgers tied to Harbor runs | **N/A** | Token fields on bundles and `/api/metrics` serve ops, not eval-cost accounting. |
| Org-wide “which agent-eval runs when” ADRs | **N/A** (deferred) | OKF fixture governance is enough for this repo; org-wide eval policy is out of tree. |

Audit cluster C08 (L76 Agent-Eval Pipeline) correctly notes the absence of
Harbor/portage/Terminal-Bench paths. That absence is a **documented product
boundary**, not an unfinished P0. Soft goal #68 accepts expanding the OKF
fixture corpus and recording this N/A rather than building an agent-eval
pipeline unless SessionLedger’s charter changes to become an eval harness.

### Revisit trigger

Reopen Harbor / multi-env agent-eval only if all of the following hold:

1. Product charter explicitly includes scoring agents across external envs.
2. A named owner and acceptance criteria exist for ≥2 env providers.
3. OKF conformance remains a separate, non-blocking corpus (this doc’s
   “in scope” table stays intact).

Until then, treat Harbor-scale agent-eval asks as **out of scope**.

---

## Related

- Soft goal: issue #68 — eval corpus depth (C08)
- Conformance process: [`docs/reference/conformance/README.md`](reference/conformance/README.md)
- Observability soft goals (separate): [`docs/ops/observability.md`](ops/observability.md)
