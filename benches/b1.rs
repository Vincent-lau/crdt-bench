use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::{cell::RefCell, rc::Rc};

use bench_utils::generate_random_string;
use crdt_bench::{bam::BenchAM, bench_utils, byrs::BenchYrs, crdt::Crdt};

static SAMPLE_SIZE: usize = 30;

static N: usize = 6000;

fn insert_1b1(text: String, doc: Rc<RefCell<dyn Crdt>>) {
    let mut doc = doc.borrow_mut();
    for (i, c) in text.chars().enumerate() {
        doc.insert_text(i, &c.to_string());
    }
}

fn insert1big(text: String, doc: Rc<RefCell<dyn Crdt>>) {
    let mut doc = doc.borrow_mut();
    doc.insert_text(0, &text);
}

fn insert1big_with_update(text: String, doc: Rc<RefCell<dyn Crdt>>) -> Vec<u8> {
    let mut doc = doc.borrow_mut();
    doc.insert_text_update(0, &text)
}

fn insert_1b1_with_updates(doc: Rc<RefCell<dyn Crdt>>) -> Vec<Vec<u8>> {
    // fine to generate text here since we won't use this for benchmark anyway
    let text = generate_random_string(N);
    let mut doc = doc.borrow_mut();
    let mut updates = Vec::new();
    for (i, c) in text.chars().enumerate() {
        let update = doc.insert_text_update(i, &c.to_string());
        updates.push(update);
    }
    updates
}

fn load(doc: Rc<RefCell<dyn Crdt>>, data: Vec<u8>) {
    let doc2 = doc.borrow().load(data);
    debug_assert_eq!(doc.borrow().text(), doc2.borrow().text());
}

fn bench_loading(c: &mut Criterion) {
    let doca = Rc::new(RefCell::new(BenchAM::default()));
    let docy = Rc::new(RefCell::new(BenchYrs::default()));
    let docs: Vec<Rc<RefCell<dyn Crdt>>> = vec![doca, docy];

    let mut group = c.benchmark_group("Loading doc");
    for doc in docs {
        group.sample_size(SAMPLE_SIZE);

        let bench_name = String::from(doc.borrow().name());
        group.bench_function(BenchmarkId::new("load", bench_name), |b| {
            b.iter_batched(
                || {
                    let doc = { doc.borrow().new() };
                    let text = generate_random_string(N);
                    insert_1b1(text, doc.clone());
                    let doc = doc.clone();
                    doc.borrow_mut().encoded_state()
                },
                |data| load(doc.clone(), data),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn bench_1b1_insert_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group("Insert characters one by one");
    group.sample_size(SAMPLE_SIZE);
    for doc in docs {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter_batched(
                || generate_random_string(N),
                |text| insert_1b1(text, doc.clone()),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_1b1_apply_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group("Apply inserted chars one by one");
    group.sample_size(SAMPLE_SIZE);
    for doc1 in docs {
        let name: String = String::from(doc1.borrow().name());
        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    let doc1 = doc1.borrow().new();
                    let state = doc1.borrow_mut().encoded_state();
                    let doc2 = { doc1.borrow().load(state) };
                    (doc1.clone(), doc2, insert_1b1_with_updates(doc1.clone()))
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
    let mut group = c.benchmark_group("Append one big text");
    group.sample_size(SAMPLE_SIZE);
    for doc in docs {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter_batched(
                || generate_random_string(N),
                |text| {
                    insert1big(text, doc.clone());
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_bulk_apply_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group("Apply one big change");
    group.sample_size(SAMPLE_SIZE);
    for doc1 in docs {
        let name: String = String::from(doc1.borrow().name());
        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    let doc1 = doc1.borrow().new();
                    let state = doc1.borrow_mut().encoded_state();
                    let doc2 = { doc1.borrow().load(state) };
                    (
                        doc1.clone(),
                        doc2,
                        insert1big_with_update(generate_random_string(N), doc1.clone()),
                    )
                },
                |(doc1, doc2, update)| {
                    doc2.borrow_mut().apply_update(update);
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
    bench_1b1_insert_time,
    bench_1b1_apply_time,
    bench_bulk_insert_time,
    bench_bulk_apply_time
);

criterion_group!(load_time, bench_loading);

criterion_main!(update_time, load_time);
