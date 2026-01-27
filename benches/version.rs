use std::{any::Any, cell::RefCell, rc::Rc};

use automerge::ChangeHash;
use crdt_bench::{
    bam::BenchAM,
    bench_utils::{self, am_doc, generate_random_string, yrs_doc},
    byrs::BenchYrs,
    crdt::Crdt,
};
use criterion::{Criterion, criterion_group, criterion_main};
use yrs::{
    ReadTxn, Subscription,
    updates::{decoder::Decode, encoder::Encode},
};

static N: usize = 10;
static NUM_VERS: usize = 1000;
static VER_TO_GO: usize = 19;
static SAMPLE_SIZE: usize = 20;

fn track_version(doc: Rc<RefCell<dyn Any>>) -> (Rc<RefCell<Vec<Vec<u8>>>>, Option<Subscription>) {
    // Collect updates as they happen
    let doc = doc.borrow();
    if let Some(doc) = doc.downcast_ref::<BenchYrs>() {
        let updates: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));

        let updates_clone = updates.clone();
        let sub = doc
            .observe_update_v1(move |_, event| {
                updates_clone.borrow_mut().push(event.update.clone());
            })
            .unwrap();
        (updates, Some(sub))
    } else {
        (Rc::new(RefCell::new(vec![])), None)
    }
}

fn checkpoint_am(doc: Rc<RefCell<dyn Any>>) -> Option<Vec<ChangeHash>> {
    let mut doc = doc.borrow_mut();
    if let Some(doc) = doc.downcast_mut::<BenchAM>() {
        return Some(doc.get_heads());
    }
    return None;
}

fn revert_version(
    doc: Rc<RefCell<dyn Any>>,
    updates: Rc<RefCell<Vec<Vec<u8>>>>,
    checkpoint: Option<Vec<ChangeHash>>,
) {
    let mut doc = doc.borrow_mut();
    if let Some(_doc) = doc.downcast_mut::<BenchYrs>() {
        let mut doc2 = BenchYrs::default();

        let updates = updates.borrow();
        for update_bytes in updates.iter().take(VER_TO_GO * N) {
            doc2.apply_update(update_bytes);
        }
        debug_assert_eq!(doc2.text().len(), VER_TO_GO * N);
    } else if let Some(doc) = doc.downcast_mut::<BenchAM>() {
        let doc2 = doc.fork_at(&checkpoint.unwrap()).unwrap();
        debug_assert_eq!(doc2.text().len(), VER_TO_GO * N);
    } else {
        unreachable!("unknown doc type");
    }
}

fn edit_and_go_back(c: &mut Criterion) {
    let docs = bench_utils::all_docs();
    let mut group = c.benchmark_group("Version control");
    group.sample_size(SAMPLE_SIZE);

    for doc in docs {
        let name: String = String::from(doc.borrow().name());

        group.bench_function(name, |b| {
            b.iter_batched(
                || {
                    let new_doc = doc.borrow().new();
                    let mut texts = Vec::new();
                    for _ in 0..NUM_VERS {
                        texts.push(generate_random_string(N));
                    }
                    (new_doc, texts)
                },
                |(doc, texts)| {
                    let (updates, _sub) = track_version(doc.clone());
                    let mut checkpoint = None;
                    let mut ofs: usize = 0;
                    for (i, t) in texts.iter().enumerate() {
                        bench_utils::insert_1b1(&t, ofs, doc.clone());
                        if i == VER_TO_GO {
                            checkpoint = checkpoint_am(doc.clone());
                        }
                        ofs += t.len();
                    }
                    revert_version(doc.clone(), updates, checkpoint);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
}

fn ver_to_diff() -> impl Iterator<Item = usize> {
    0..std::cmp::min(19, NUM_VERS - 1)
}

fn edit_and_diff(c: &mut Criterion) {
    let mut texts = Vec::new();
    for _ in 0..NUM_VERS {
        texts.push(generate_random_string(N));
    }

    let am = |(doc, texts): (Rc<RefCell<BenchAM>>, Vec<String>)| {
        let mut ofs: usize = 0;
        let mut checkpoints = Vec::new();
        for t in &texts {
            bench_utils::insert_1b1(t, ofs, doc.clone());
            checkpoints.push(doc.borrow_mut().get_heads());
            ofs += t.len();
        }

        debug_assert_eq!(doc.borrow().text().len(), ofs);
        let mut doc = doc.borrow_mut();
        let heads = doc.get_heads();
        for v in ver_to_diff() {
            let patch = doc.diff(&checkpoints[v], &heads);
            assert!(patch.len() > 0);
        }
    };

    // For yrs, if we want to diff against every single change, then we need to
    // store the state vector. If we want to go back to arbitrary points, then we
    // need to store all states.
    let yrs = |(doc, texts): (Rc<RefCell<BenchYrs>>, Vec<String>)| {
        let mut checkpoints = Vec::new();
        let mut ofs = 0;
        for t in &texts {
            bench_utils::insert_1b1(t, ofs, doc.clone());
            ofs += t.len();
            let doc = doc.borrow();
            let txn = doc.transact();
            checkpoints.push((
                txn.state_vector().encode_v1(),
                txn.encode_state_as_update_v1(&yrs::StateVector::default()),
            ))
        }
        debug_assert_eq!(doc.borrow().text().len(), ofs);

        for v in ver_to_diff() {
            let sv = yrs::StateVector::decode_v1(&checkpoints[v].0).unwrap();
            let patch = doc.borrow().get_changes(&sv);
            // println!("yrs patch size {}", patch.len());
            assert!(patch.len() > 0);
        }
    };

    let mut group = c.benchmark_group("Version control");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("Automerge", |b| {
        b.iter_batched(
            || {
                let new_doc = am_doc();
                let mut texts = Vec::new();
                for _ in 0..NUM_VERS {
                    texts.push(generate_random_string(N));
                }
                (new_doc, texts)
            },
            am,
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("Yrs", |b| {
        b.iter_batched(
            || {
                let new_doc = yrs_doc();
                let mut texts = Vec::new();
                for _ in 0..NUM_VERS {
                    texts.push(generate_random_string(N));
                }
                (new_doc, texts)
            },
            yrs,
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(version, edit_and_diff);

criterion_main!(version);
