#![cfg(feature = "compress")]

use session_ledger::ports::Compressor;
use session_ledger::ZstdCompressor;

const FIXTURE: &str = include_str!("fixtures/okf/auth-fix-session-001.okf.json");
const ZSTD_LEVEL: i32 = 3;
const MAX_RATIO_BPS: usize = 6_500;

#[test]
fn fixture_session_zstd_roundtrip_stays_below_ratio_gate() {
    let compressor = ZstdCompressor::new(ZSTD_LEVEL);

    let compressed = compressor.compress(FIXTURE).expect("compress fixture");
    let decoded = compressor.decompress(&compressed).expect("decompress fixture");

    assert_eq!(decoded, FIXTURE, "zstd must preserve fixture bytes");

    let ratio_bps = compressed.len() * 10_000 / FIXTURE.len();
    assert!(
        ratio_bps <= MAX_RATIO_BPS,
        "zstd level {ZSTD_LEVEL} compressed fixture to {ratio_bps} bps, above {MAX_RATIO_BPS} bps \
         ({} compressed bytes / {} source bytes)",
        compressed.len(),
        FIXTURE.len(),
    );
}
