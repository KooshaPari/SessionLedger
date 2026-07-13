# Compression Evaluation Gate

This gate records shallow evidence that SessionLedger's zstd adapter reduces a representative OKF session while preserving byte-for-byte round trips.

## Method

Run the feature-gated integration test:

```sh
cargo test -p session-ledger --features compress --test compression_eval --locked
```

The test loads `tests/fixtures/okf/auth-fix-session-001.okf.json`, compresses it with `ZstdCompressor` at level 3, decompresses it, and asserts that the decoded text exactly matches the fixture. It then computes:

```text
ratio_bps = compressed_bytes * 10_000 / source_bytes
```

Basis points keep the assertion integer-only and stable in CI.

## Threshold

The current gate requires `ratio_bps <= 6_500`, meaning the compressed payload must be at most 65% of the source fixture size. This is intentionally loose enough for fixture evolution, but tight enough to catch accidental passthrough compression, feature miswiring, or a compressor-level regression.

If the fixture changes materially, update the threshold only with a fresh local run and include the observed compressed/source byte counts in the PR.

## Token-Burn Proxy

SessionLedger burns tokens when continuation context is injected back into an agent. Byte savings are not a tokenizer-accurate measurement, but for JSON-like OKF payloads they are a useful rough proxy:

```text
bytes_saved = source_bytes - compressed_bytes
rough_tokens_saved ~= bytes_saved / 4
```

The gate therefore protects a simple operational claim: smaller serialized continuation payloads usually imply less context transferred, stored, and eventually rehydrated into prompt material. The CI assertion is about byte ratio, and the token estimate remains a coarse planning signal rather than a billing metric.
