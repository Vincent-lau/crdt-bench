use std::{fs, path::Path};

use crdt_bench::bench_utils::all_docs;
use criterion::{Criterion, criterion_group, criterion_main};

static SAMPLE_SIZE: usize = 10;
static SEQ_TRACE_DIR: &str = "benches/data/editing-traces/sequential_traces/ascii_only";
static CONC_TRACE_DIR: &str = "benches/data/editing-traces/concurrent_traces/";

fn load_seq(c: &mut Criterion, path: &Path) {
    let mut group = c.benchmark_group(format!(
        "Time loading sequential docs {}",
        path.as_os_str().display()
    ));
    let traces = fs::read_to_string(path).unwrap();
    let parsed = json::parse(&traces).unwrap();
    group.sample_size(SAMPLE_SIZE);
    for doc in all_docs() {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter(|| {
                let doc = doc.borrow().new();
                for txn in parsed["txns"].members() {
                    for patch in txn["patches"].members() {
                        let idx = patch[0].as_usize().unwrap();
                        let del = patch[1].as_isize().unwrap();
                        let text = patch[2].as_str().unwrap();
                        doc.borrow_mut().insert_delete_text(idx, del, text);
                    }
                }
                debug_assert_eq!(doc.borrow().text(), parsed["endContent"].as_str().unwrap());
            })
        });
    }
    group.finish();
}

fn load_conc(c: &mut Criterion, path: &Path) {
    let mut group = c.benchmark_group(format!(
        "Time loading concurrent docs {}",
        path.as_os_str().display()
    ));
    let traces = fs::read_to_string(path).unwrap();
    let parsed = json::parse(&traces).unwrap();
    assert_eq!(parsed["numAgents"].as_usize().unwrap(), 2);
    group.sample_size(SAMPLE_SIZE);
    for doc in all_docs() {
        let bench_name = String::from(doc.borrow().name());
        group.bench_function(bench_name, |b| {
            b.iter(|| {
                // versioned_docs: (txn_index, doc) = doc state after that transaction
                let mut versioned_docs = vec![(0usize, doc.borrow().new())];
                for (i, txn) in parsed["txns"].members().enumerate() {
                    let parents = &txn["parents"];
                    let actioned_doc;

                    if parents.len() == 0 {
                        // Empty parents: start from empty doc (we have one placeholder at (0, new()))
                        let (_j, d) = versioned_docs.pop().unwrap();
                        for patch in txn["patches"].members() {
                            let idx = patch[0].as_usize().unwrap();
                            let del = patch[1].as_isize().unwrap();
                            let text = patch[2].as_str().unwrap();
                            d.borrow_mut().insert_delete_text(idx, del, text);
                        }
                        actioned_doc = Some(d.clone());
                        versioned_docs.push((i, d));
                    } else if parents.len() == 1 {
                        // Single parent: doc state = state after that txn; apply all patches in sequence
                        let p = parents[0].as_usize().unwrap();
                        let pos = versioned_docs.iter().position(|(j, _)| *j == p).unwrap();
                        let d = versioned_docs[pos].1.clone();
                        for patch in txn["patches"].members() {
                            let idx = patch[0].as_usize().unwrap();
                            let del = patch[1].as_isize().unwrap();
                            let text = patch[2].as_str().unwrap();
                            d.borrow_mut().insert_delete_text(idx, del, text);
                        }
                        actioned_doc = Some(d.clone());
                        versioned_docs[pos] = (i, d);
                    } else {
                        // Multiple parents: merge all parent states, then apply all patches
                        let mut to_merge = Vec::new();
                        for p in parents.members() {
                            to_merge.push(
                                versioned_docs
                                    .iter()
                                    .position(|(j, _)| *j == p.as_usize().unwrap())
                                    .unwrap(),
                            );
                        }
                        let d0 = versioned_docs[to_merge[0]].1.clone();
                        for &idx_merge in &to_merge[1..] {
                            d0.borrow_mut().merge(versioned_docs[idx_merge].1.clone());
                        }
                        for patch in txn["patches"].members() {
                            let idx = patch[0].as_usize().unwrap();
                            let del = patch[1].as_isize().unwrap();
                            let text = patch[2].as_str().unwrap();
                            d0.borrow_mut().insert_delete_text(idx, del, text);
                        }
                        actioned_doc = Some(d0.clone());
                        versioned_docs[to_merge[0]] = (i, d0);
                        // Remove other merged indices in descending order so indices stay valid
                        let mut to_remove = to_merge[1..].to_vec();
                        to_remove.sort_by(|a, b| b.cmp(a));
                        for idx_remove in to_remove {
                            versioned_docs.remove(idx_remove);
                        }
                    }

                    // numChildren future txns use this state as parent; we already have 1 copy, fork the rest
                    let num_children = txn["numChildren"].as_u32().unwrap() as usize;
                    // println!("doc text {}", actioned_doc.as_ref().unwrap().borrow().text());
                    for _ in 0..num_children - 1 {
                        versioned_docs
                            .push((i, actioned_doc.as_ref().unwrap().borrow_mut().fork()));
                    }
                }
                // println!("{}", doc.borrow().text());
                assert_eq!(versioned_docs.len(), 1);
                assert_eq!(
                    versioned_docs[0].1.borrow().text(),
                    parsed["endContent"].as_str().unwrap()
                );
            })
        });
    }
}

fn am_paper(c: &mut Criterion) {
    let path = Path::new(SEQ_TRACE_DIR).join("automerge-paper.json");
    load_seq(c, &path);
}

fn seph_blog(c: &mut Criterion) {
    let path = Path::new(SEQ_TRACE_DIR).join("seph-blog1.json");
    load_seq(c, &path);
}

fn friendsforever(c: &mut Criterion) {
    let path = Path::new(CONC_TRACE_DIR).join("friendsforever.json");
    load_conc(c, &path);
}

criterion_group!(seq, am_paper, seph_blog);
criterion_group!(conc, friendsforever);
criterion_main!(conc);
