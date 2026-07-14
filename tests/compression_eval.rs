#![cfg(feature = "compress")]

use session_ledger::ports::Compressor;
use session_ledger::{CharCountTokenEstimator, TokenEstimator, ZstdCompressor};

const ZSTD_LEVEL: i32 = 3;
const MAX_RATIO_BPS: usize = 6_500;

/// Compression / token-oriented OKF fixtures exercised by the CI gate.
/// Paths are relative to `CARGO_MANIFEST_DIR` and kept CI-small.
const COMPRESS_TOKEN_FIXTURES: &[&str] = &[
    "tests/fixtures/okf/auth-fix-session-001.okf.json",
    "docs/reference/conformance/fixtures/task-family-token-budget-032.okf.json",
    "docs/reference/conformance/fixtures/task-family-compress-resume-033.okf.json",
    "docs/reference/conformance/fixtures/compress-token-proxy-034.okf.json",
    "docs/reference/conformance/fixtures/token-slice-budget-035.okf.json",
    "docs/reference/conformance/fixtures/archive-gzip-resume-036.okf.json",
];

#[test]
fn compress_token_fixtures_zstd_roundtrip_stays_below_ratio_gate() {
    let compressor = ZstdCompressor::new(ZSTD_LEVEL);
    let estimator = CharCountTokenEstimator;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for rel in COMPRESS_TOKEN_FIXTURES {
        let path = root.join(rel);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));

        let compressed = compressor
            .compress(&source)
            .unwrap_or_else(|err| panic!("compress {}: {err}", path.display()));
        let decoded = compressor
            .decompress(&compressed)
            .unwrap_or_else(|err| panic!("decompress {}: {err}", path.display()));

        assert_eq!(
            decoded, source,
            "zstd must preserve fixture bytes for {}",
            path.display()
        );

        let ratio_bps = compressed.len() * 10_000 / source.len();
        assert!(
            ratio_bps <= MAX_RATIO_BPS,
            "zstd level {ZSTD_LEVEL} compressed {} to {ratio_bps} bps, above {MAX_RATIO_BPS} bps \
             ({} compressed bytes / {} source bytes)",
            path.display(),
            compressed.len(),
            source.len(),
        );

        let bytes_saved = source.len().saturating_sub(compressed.len());
        let rough_tokens_saved = bytes_saved / 4;
        let source_tokens = estimator.estimate_text(&source) as usize;
        assert!(
            source_tokens > 0,
            "CharCountTokenEstimator must score {} as non-empty",
            path.display()
        );
        assert!(
            bytes_saved > 0 && rough_tokens_saved > 0,
            "expected positive token-burn proxy on {}: bytes_saved={bytes_saved} \
             rough_tokens_saved={rough_tokens_saved}",
            path.display()
        );
    }
}
