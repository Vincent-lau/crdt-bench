use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use bench_utils::generate_random_string;
use crdt_bench::{
    bench_utils::{
        self, am_doc, insert_1b1, insert_1b1_with_updates, insert1big, insert1big_with_update,
    },
    crdt::Crdt,
};

static SAMPLE_SIZE: usize = 10;

static N: usize = 6000;

fn bench_1b1_insert_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group(format!("Insert {N} characters one by one"));
    group.sample_size(SAMPLE_SIZE);
    for doc in docs {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter_batched(
                || generate_random_string(N),
                |text| insert_1b1(&text, 0, doc.clone()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_1b1_apply_time(c: &mut Criterion) {
    // let docs = bench_utils::all_docs();
    let docs = vec![am_doc()];
    let mut group = c.benchmark_group(format!("Apply {N} inserted chars one by one"));
    group.sample_size(SAMPLE_SIZE);
    for doc1 in docs {
        // let name: String = String::from(doc1.borrow().name());
        let name = "automerge";
        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    let state = doc1.borrow_mut().encoded_state();
                    let doc2 = doc1.borrow().load(&state);
                    let t = generate_random_string(N);
                    let updates = insert_1b1_with_updates(&t, 0, doc1.clone());
                    (doc1.clone(), doc2, updates)
                },
                |(doc1, doc2, updates)| {
                    for update in updates {
                        doc2.borrow_mut().apply_update(&update);
                    }
                    debug_assert_eq!(doc1.borrow().text(), doc2.borrow().text());
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_bulk_insert_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group(format!("Append one big text of size {N}"));
    group.sample_size(SAMPLE_SIZE);
    for doc in docs {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter_batched(
                || generate_random_string(N),
                |text| {
                    insert1big(&text, 0, doc.clone());
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_bulk_apply_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group(format!("Apply one big change of size {N}"));
    group.sample_size(SAMPLE_SIZE);
    for doc1 in docs {
        let name: String = String::from(doc1.borrow().name());
        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    let doc1 = doc1.borrow().new();
                    let state = doc1.borrow_mut().encoded_state();
                    let doc2 = { doc1.borrow().load(&state) };
                    let t = generate_random_string(N);
                    (
                        doc1.clone(),
                        doc2,
                        insert1big_with_update(&t, 0, doc1.clone()),
                    )
                },
                |(doc1, doc2, update)| {
                    doc2.borrow_mut().apply_update(&update);
                    debug_assert_eq!(doc1.borrow().text(), doc2.borrow().text());
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

criterion_group!(
    update_time,
    // bench_1b1_insert_time,
    bench_1b1_apply_time,
    // bench_bulk_insert_time,
    // bench_bulk_apply_time
);

criterion_main!(update_time);
