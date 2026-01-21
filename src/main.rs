#![allow(unused)]

// use std::hint::black_box;

use std::{
    cell::RefCell,
    fs,
    hint::black_box,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Instant,
};

use automerge::{AutoCommit, ReadDoc, transaction::Transactable};
use crdt_bench::{
    bam::BenchAM,
    bench_utils::{self, am_doc, generate_random_string},
    byrs::BenchYrs,
    crdt::Crdt,
};
use yrs::{Text, Transact};

// fn load_file(check: bool) {
//     let filename = "./benches/moby-dick.amrg";
//     let data = std::fs::read(filename).unwrap();
//     let opts = if check {
//         LoadOptions::new().verification_mode(automerge::VerificationMode::Check)
//     } else {
//         LoadOptions::new().verification_mode(automerge::VerificationMode::DontCheck)
//     };
//     let result = Automerge::load_with_options(&data, opts).unwrap();
//     black_box(result);
// }

fn _f() {
    let mut doc = AutoCommit::new();
    let o = doc
        .put_object(automerge::ROOT, "test", automerge::ObjType::Text)
        .unwrap();
    doc.splice_text(o, 0, 0, "hi").unwrap();
    let update = doc.save_incremental();
    println!("update size {}", update.len());

    let mut doc2 = AutoCommit::new();
    doc2.load_incremental(&update).unwrap();
    let (_, p) = doc2.get(automerge::ROOT, "test").unwrap().unwrap();
    println!("doc2 {}", doc2.text(p).unwrap());
}

fn _g() {
    let d = Rc::new(RefCell::new(BenchYrs::default()));
    // bench_utils::insert_1b1(bench_utils::generate_random_string(N), d.clone());
    let data = d.borrow_mut().encoded_state();
    fs::write("benches/data/b1.yrs", &data).unwrap();
}

// fn a() {
//     let doc = Rc::new(RefCell::new(BenchAM::default()));
//     let text = bench_utils::generate_random_string(6000);
//     let text2 = bench_utils::generate_random_string(6000);

//     let v1 = doc.borrow_mut().get_heads();
//     bench_utils::insert_1b1(&text, doc.clone());
//     bench_utils::insert_1b1(&text2, doc.clone());

//     let mut doc = doc.borrow_mut();
//     // let changes1 = doc.get_changes(&v1);

//     let doc2 = doc.fork_at(&v1).unwrap();
//     black_box(doc2);
// }

fn y() {
    let doc = Rc::new(RefCell::new(BenchYrs::default()));

    // Collect updates as they happen
    let updates: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));

    let updates_clone = updates.clone();
    let _sub = doc
        .borrow()
        .observe_update_v1(move |_, event| {
            updates_clone.borrow_mut().push(event.update.clone());
        })
        .unwrap();

    // Make edits (each generates an update)
    let text = generate_random_string(6000);
    let text2 = generate_random_string(6000);
    bench_utils::insert_1b1(&text, doc.clone());
    bench_utils::insert_1b1(&text2, doc.clone());

    let all_updates = updates.borrow();
    println!("Captured {} updates", all_updates.len());
}




fn main() {





}
