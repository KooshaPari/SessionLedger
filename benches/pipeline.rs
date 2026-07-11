use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use session_ledger::domain::session::{Corpus, Message, Role, Session};

fn representative_session() -> Session {
    let mut session = Session::new("bench-session", Corpus::Forge);
    session.cwd = Some("/workspace/session-ledger".into());
    session.title = Some("Benchmark the continuation pipeline".into());

    for index in 0..100 {
        session.messages.push(Message::new(
            Role::User,
            format!(
                "Iteration {index}: update src/distill/compiler.rs, preserve the public API, and run cargo test"
            ),
        ));
        session.messages.push(Message::new(
            Role::Assistant,
            format!("Iteration {index}: updated the compiler and verified the requested behavior"),
        ));
    }

    session
}

fn pipeline_benches(c: &mut Criterion) {
    let session = representative_session();
    let bundle = session_ledger::distill::compile(&session);

    let mut group = c.benchmark_group("pipeline");
    group.throughput(Throughput::Elements(session.messages.len() as u64));

    group.bench_function("distill_compile_200_messages", |b| {
        b.iter(|| session_ledger::distill::compile(black_box(&session)));
    });
    group.bench_function("okf_export_200_messages", |b| {
        b.iter(|| {
            session_ledger::export_to_okf(black_box(&bundle), black_box(session.corpus.as_str()))
        });
    });
    group.bench_function("inject_render_200_messages", |b| {
        b.iter(|| {
            session_ledger::render_prompt(black_box(&bundle))
                .expect("compiled bundle should remain injectable")
        });
    });

    group.finish();
}

criterion_group!(benches, pipeline_benches);
criterion_main!(benches);
