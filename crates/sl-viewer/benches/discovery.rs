use std::{hint::black_box, path::PathBuf, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use sl_viewer::{load_sessions, DataSource};

fn fixture_database() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("forge_fixture.db")
}

fn configured_criterion() -> Criterion {
    let mut criterion = Criterion::default().configure_from_args();

    if let Ok(sample_size) = std::env::var("SESSION_LEDGER_VIEWER_DISCOVERY_SAMPLE_SIZE") {
        criterion = criterion.sample_size(
            sample_size
                .parse::<usize>()
                .expect("SESSION_LEDGER_VIEWER_DISCOVERY_SAMPLE_SIZE must be an integer"),
        );
    }
    if let Ok(seconds) = std::env::var("SESSION_LEDGER_VIEWER_DISCOVERY_WARM_UP_SECONDS") {
        criterion = criterion.warm_up_time(Duration::from_secs_f64(
            seconds
                .parse::<f64>()
                .expect("SESSION_LEDGER_VIEWER_DISCOVERY_WARM_UP_SECONDS must be a number"),
        ));
    }
    if let Ok(seconds) = std::env::var("SESSION_LEDGER_VIEWER_DISCOVERY_MEASUREMENT_SECONDS") {
        criterion = criterion.measurement_time(Duration::from_secs_f64(
            seconds
                .parse::<f64>()
                .expect("SESSION_LEDGER_VIEWER_DISCOVERY_MEASUREMENT_SECONDS must be a number"),
        ));
    }

    criterion
}

fn viewer_discovery_benches(c: &mut Criterion) {
    let fixture = fixture_database();
    assert!(fixture.is_file(), "missing fixture database: {}", fixture.display());

    c.bench_function("viewer_discovery/forge_fixture_load_2_sessions", |b| {
        b.iter(|| {
            let sessions = load_sessions(black_box(&DataSource::ForgeDb(fixture.clone())))
                .expect("fixture discovery must load");
            assert_eq!(sessions.len(), 2, "fixture contract changed");
            black_box(sessions)
        });
    });
}

criterion_group! {
    name = benches;
    config = configured_criterion();
    targets = viewer_discovery_benches
}
criterion_main!(benches);
