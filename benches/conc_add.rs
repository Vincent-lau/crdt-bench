use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use bench_utils::generate_random_string;
use crdt_bench::bench_utils::{self, insert_1b1_with_updates};

static SAMPLE_SIZE: usize = 10;

static N: usize = 6000;
static NUM_CLIENTS: usize = 4;

fn multi_client_sync_time(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group(format!(
        "{} clients making {} character changes concurrently, total apply time",
        NUM_CLIENTS,
        N / 10
    ));
    group.sample_size(SAMPLE_SIZE);
    for doc in docs {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter_batched(
                || {
                    let doc = doc.borrow().new();
                    let t = generate_random_string(N);

                    bench_utils::insert_1b1(&t, 0, doc.clone());
                    let mut client_docs = vec![doc.clone()];

                    let data = doc.borrow_mut().encoded_state();
                    for _ in 0..NUM_CLIENTS - 1 {
                        let doc2 = doc.borrow().load(&data);
                        client_docs.push(doc2);
                    }
                    let mut updates = Vec::new();

                    for i in 0..NUM_CLIENTS {
                        let t = generate_random_string(N / 10);
                        let index = client_docs[i].borrow().text().len();
                        updates.push(insert_1b1_with_updates(&t, index, client_docs[i].clone()));
                    }

                    (client_docs, updates)
                },
                |(client_docs, updates)| {
                    for (i, d) in client_docs.iter().enumerate() {
                        for (j, us) in updates.iter().enumerate() {
                            if i == j {
                                continue;
                            }
                            us.iter().for_each(|u| d.borrow_mut().apply_update(u));
                        }
                        debug_assert_eq!(d.borrow().text().len(), N + (N / 10) * NUM_CLIENTS);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(apply_time, multi_client_sync_time);

criterion_main!(apply_time);
