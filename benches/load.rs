use automerge::{Automerge, LoadOptions};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn load_file(check: bool) {
    let filename = "./benches/b1.amrg";
    let data = std::fs::read(filename).unwrap();
    let opts = if check {
        LoadOptions::new().verification_mode(automerge::VerificationMode::Check)
    } else {
        LoadOptions::new().verification_mode(automerge::VerificationMode::DontCheck)
    };
    let result = Automerge::load_with_options(&data, opts).unwrap();
    black_box(result);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Loading doc");
    group.sample_size(10);
    group.bench_function("loading without checking", |b| b.iter(|| load_file(false)));
    group.bench_function("loading with checking", |b| b.iter(|| load_file(true)));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
