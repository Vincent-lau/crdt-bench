use automerge::{Automerge, LoadOptions};
use crdt_bench::{
    bench_utils::all_docs,
    crdt::{Crdt, CrdtLib},
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::{cell::RefCell, fs, hint::black_box, path::Path, rc::Rc};
use yrs::{Transact, updates::decoder::Decode};

static FILENAME: &str = "./benches/data/rnd-6k";

fn load_file(doc: Rc<RefCell<dyn Crdt>>, check: bool) {
    let doc = doc.borrow_mut();

    match doc.crdt_lib() {
        CrdtLib::Automerge => {
            let name = format!("{}.{}", FILENAME, "am");
            let file_path = Path::new(&name);
            let opts = if check {
                LoadOptions::new().verification_mode(automerge::VerificationMode::Check)
            } else {
                LoadOptions::new().verification_mode(automerge::VerificationMode::DontCheck)
            };
            let data = fs::read(file_path).unwrap();
            let d = Automerge::load_with_options(&data, opts).unwrap();
            black_box(d);
        }
        CrdtLib::Yrs => {
            let name = format!("{}.{}", FILENAME, "yrs");
            let file_path = Path::new(&name);
            let doc = yrs::Doc::new();
            let data = fs::read(file_path).unwrap();
            doc.transact_mut()
                .apply_update(yrs::Update::decode_v1(&data).unwrap())
                .unwrap();
            black_box(doc);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Loading doc");
    group.sample_size(10);
    let docs = all_docs();
    for doc in docs {
        let name = doc.borrow().name();
        group.bench_function(BenchmarkId::new("loading doc", name), |b| {
            b.iter(|| load_file(doc.clone(), true))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
