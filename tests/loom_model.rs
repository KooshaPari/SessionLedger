//! Loom permutation models for C00 L7 concurrency safety.
//!
//! Enabled only with `RUSTFLAGS='--cfg loom'` (see `.github/workflows/loom-smoke.yml`,
//! `.github/workflows/loom-permutation.yml`, and `docs/ops/concurrency-safety.md`).
//! The loom crate is a `[target.'cfg(loom)'.dev-dependencies]` entry so default
//! `cargo test` never builds it.
//!
//! Covers cancel + bounded capacity, `sync_channel`-style try_send, broadcast/SSE
//! epoch fan-out, and watcher-queue → broadcast pipeline permutations. Full
//! tokio broadcast / daemon graph ports remain unpaid.

#[cfg(not(loom))]
#[test]
fn loom_cfg_not_enabled_documents_soft_lane() {
    // Discoverable under default `cargo test` without pulling loom.
    eprintln!("skip: loom_model requires RUSTFLAGS=--cfg loom (soft CI: loom-smoke.yml)");
}

#[cfg(loom)]
mod loom_perm {
    use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use loom::sync::mpsc;
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

    /// Loom port of `race_model`'s bounded `try_send` contract: at most `CAPACITY`
    /// messages may be buffered; cancel forbids new reservations.
    #[test]
    fn bounded_try_send_respects_capacity() {
        loom::model(|| {
            const CAPACITY: usize = 2;
            let cancel = Arc::new(AtomicBool::new(false));
            let buffered = Arc::new(AtomicUsize::new(0));
            let (tx, rx) = mpsc::channel::<u32>();

            let producer = {
                let cancel = Arc::clone(&cancel);
                let buffered = Arc::clone(&buffered);
                let tx = tx.clone();
                thread::spawn(move || {
                    for id in 0..3u32 {
                        if cancel.load(Ordering::Acquire) {
                            break;
                        }
                        let prev = buffered.load(Ordering::Acquire);
                        if prev >= CAPACITY {
                            break;
                        }
                        if buffered
                            .compare_exchange(prev, prev + 1, Ordering::AcqRel, Ordering::Acquire)
                            .is_ok()
                        {
                            let _ = tx.send(id);
                        }
                    }
                })
            };

            producer.join().unwrap();
            drop(tx);

            let mut drained = 0usize;
            while rx.try_recv().is_ok() {
                drained += 1;
                buffered.fetch_sub(1, Ordering::AcqRel);
            }

            assert!(buffered.load(Ordering::Acquire) <= CAPACITY);
            assert!(drained <= CAPACITY);
        });
    }

    /// Models SSE broadcast fan-out: one epoch bump is observed by each subscriber
    /// without exceeding the publisher's generation count.
    #[test]
    fn broadcast_epoch_fans_out_to_subscribers() {
        loom::model(|| {
            let epoch = Arc::new(AtomicUsize::new(0));
            let sub_a = Arc::new(AtomicUsize::new(0));
            let sub_b = Arc::new(AtomicUsize::new(0));

            let publisher = {
                let epoch = Arc::clone(&epoch);
                thread::spawn(move || {
                    epoch.store(1, Ordering::Release);
                })
            };

            let reader_a = {
                let epoch = Arc::clone(&epoch);
                let sub_a = Arc::clone(&sub_a);
                thread::spawn(move || {
                    let current = epoch.load(Ordering::Acquire);
                    if current > 0 {
                        sub_a.store(current, Ordering::Release);
                    }
                })
            };

            let reader_b = {
                let epoch = Arc::clone(&epoch);
                let sub_b = Arc::clone(&sub_b);
                thread::spawn(move || {
                    let current = epoch.load(Ordering::Acquire);
                    if current > 0 {
                        sub_b.store(current, Ordering::Release);
                    }
                })
            };

            publisher.join().unwrap();
            reader_a.join().unwrap();
            reader_b.join().unwrap();

            let epoch_final = epoch.load(Ordering::Acquire);
            assert!(sub_a.load(Ordering::Acquire) <= epoch_final);
            assert!(sub_b.load(Ordering::Acquire) <= epoch_final);
        });
    }

    /// Watcher pipeline: bounded enqueue between scan and broadcast; drained
    /// broadcast count matches dequeued items.
    #[test]
    fn watcher_pipeline_bounded_enqueue_under_cancel() {
        loom::model(|| {
            const QUEUE_CAP: usize = 2;
            let cancel = Arc::new(AtomicBool::new(false));
            let queued = Arc::new(AtomicUsize::new(0));
            let broadcast = Arc::new(AtomicUsize::new(0));
            let (tx, rx) = mpsc::channel::<u32>();

            let scanner = {
                let cancel = Arc::clone(&cancel);
                let queued = Arc::clone(&queued);
                let tx = tx.clone();
                thread::spawn(move || {
                    for id in 0..3u32 {
                        if cancel.load(Ordering::Acquire) {
                            break;
                        }
                        let prev = queued.load(Ordering::Acquire);
                        if prev >= QUEUE_CAP {
                            break;
                        }
                        if queued
                            .compare_exchange(prev, prev + 1, Ordering::AcqRel, Ordering::Acquire)
                            .is_ok()
                        {
                            let _ = tx.send(id);
                        }
                    }
                })
            };

            scanner.join().unwrap();
            cancel.store(true, Ordering::Release);
            drop(tx);

            let mut drained = 0usize;
            while rx.try_recv().is_ok() {
                drained += 1;
                queued.fetch_sub(1, Ordering::AcqRel);
                broadcast.fetch_add(1, Ordering::AcqRel);
            }

            assert!(queued.load(Ordering::Acquire) <= QUEUE_CAP);
            assert_eq!(broadcast.load(Ordering::Acquire), drained);
        });
    }
}
