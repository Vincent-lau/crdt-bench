// use std::hint::black_box;

use std::{cell::RefCell, fs, rc::Rc};

use automerge::{AutoCommit, ReadDoc, transaction::Transactable};
use crdt_bench::{bam::BenchAM, bench_utils, byrs::BenchYrs, crdt::Crdt};



static N: usize = 60000;

fn insert_1b1(text: String, doc: Rc<RefCell<dyn Crdt>>) {
    let mut doc = doc.borrow_mut();
    for (i, c) in text.chars().enumerate() {
        doc.insert_text(i, &c.to_string());
    }
}

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


fn _f () {

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

fn main() {
    let d = Rc::new(RefCell::new(BenchYrs::default()));
    insert_1b1(bench_utils::generate_random_string(N), d.clone());
    let data = d.borrow_mut().encoded_state();
    fs::write("benches/data/b1.yrs", &data).unwrap();
}
