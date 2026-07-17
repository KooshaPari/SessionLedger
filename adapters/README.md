# Cross-language OKF adapters (C08 L75)

Thin **language-agnostic** consumer surface for OKF fixtures. This is **not** a
shipped multi-language SDK and **not** Harbor / agent-eval.

Canonical SSOT: [`docs/ops/cross-language-parity.md`](../docs/ops/cross-language-parity.md).

## Interface (language-agnostic)

Any host-language adapter SHOULD expose these operations over a single OKF
document path (`.okf.json`):

| Operation | Contract |
|-----------|----------|
| `load(path)` | Read UTF-8 JSON; return a document object |
| `validate(doc)` | Enforce OKF v1.0 structural rules used by the parity harness (dialect, ids, shared entity core, relation endpoints) |
| `emit(doc)` | Serialize to pretty JSON with a trailing newline (round-trip-friendly) |

CLI shape (reference):

```text
validate <path-to.okf.json>   # exit 0 on pass; non-zero + message on fail
emit <path-to.okf.json>       # print validated OKF JSON to stdout
```

## Reference implementations

| Language | Path | Fixture path exercised by SelfCheck |
|----------|------|-------------------------------------|
| Python | [`python/okf_adapter.py`](python/okf_adapter.py) | `docs/reference/conformance/fixtures/cursor-python-029.okf.json` |
| Go | [`go/main.go`](go/main.go) (+ [`go/go.mod`](go/go.mod)) | `docs/reference/conformance/fixtures/forge-go-module-026.okf.json` |

```powershell
python adapters/python/okf_adapter.py validate docs/reference/conformance/fixtures/cursor-python-029.okf.json
python adapters/python/okf_adapter.py emit docs/reference/conformance/fixtures/cursor-python-029.okf.json

go -C adapters/go run . validate ../../docs/reference/conformance/fixtures/forge-go-module-026.okf.json
go -C adapters/go run . emit ../../docs/reference/conformance/fixtures/forge-go-module-026.okf.json
```

Python: stdlib only (`json` / `sys` / `pathlib`). No pip packages.
Go: stdlib only (`encoding/json` / `os` / `fmt`). No external modules.

SelfCheck always verifies Go adapter sources exist. Runtime `go run` is executed
when `go` is on PATH; otherwise the Go execute step is an explicit skip (doc +
source anchors still pass).

## Explicit non-goals

- Native SessionLedger ports / full SDKs for Py/TS/Go
- Harbor, Portage, Terminal-Bench, or multi-env agent scoring
- Replacing the Rust `OkfExporter` / ingest pipeline
