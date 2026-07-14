//! Soft loom permutation smoke for C00 L7 concurrency safety.
//!
//! Enabled only with `RUSTFLAGS='--cfg loom'` (see `.github/workflows/loom-smoke.yml`
//! and `docs/ops/concurrency-safety.md`). The loom crate is a
//! `[target.'cfg(loom)'.dev-dependencies]` entry so default `cargo test` never
//! builds it.
//!
//! This is a tiny cancel + capacity conservation model — not a full port of
//! `tests/race_model.rs` (no `sync_channel` under loom). Soft CI only;
//! `continue-on-error: true`.

#[cfg(not(loom))]
#[test]
fn loom_cfg_not_enabled_documents_soft_lane() {
    // Discoverable under default `cargo test` without pulling loom.
    eprintln!("skip: loom_model requires RUSTFLAGS=--cfg loom (soft CI: loom-smoke.yml)");
}

#[cfg(loom)]
mod loom_perm {
    use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use loom::sync::Arc;
    use loom::thread;

    /// Two producers race a shared capacity counter under cooperative cancel.
    /// Loom explores interleavings; the assert is timing-independent.
    #[test]
    fn cancel_and_capacity_conserve() {
        loom::model(|| {
            const CAPACITY: usize = 2;
            let cancel = Arc::new(AtomicBool::new(false));
            let reserved = Arc::new(AtomicUsize::new(0));

            let mut joins = Vec::new();
            for _ in 0..2 {
                let cancel = Arc::clone(&cancel);
                let reserved = Arc::clone(&reserved);
                joins.push(thread::spawn(move || {
                    for _ in 0..4 {
                        if cancel.load(Ordering::Acquire) {
                            return;
                        }
                        let mut prev = reserved.load(Ordering::Acquire);
                        loop {
                            if prev >= CAPACITY {
                                return;
                            }
                            match reserved.compare_exchange(
                                prev,
                                prev + 1,
                                Ordering::AcqRel,
                                Ordering::Acquire,
                            ) {
                                Ok(_) => break,
                                Err(current) => prev = current,
                            }
                        }
                    }
                }));
            }

            cancel.store(true, Ordering::Release);
            for join in joins {
                join.join().unwrap();
            }
            assert!(
                reserved.load(Ordering::Acquire) <= CAPACITY,
                "reserved slots must not exceed capacity under loom interleavings"
            );
        });
    }
}
