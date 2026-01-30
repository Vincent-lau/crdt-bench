use automerge::{Automerge, LoadOptions};
use crdt_bench::{
    bam::BenchAM,
    bench_utils::{all_docs, generate_random_string, insert_1b1},
    byrs::BenchYrs,
    crdt::{Crdt, CrdtLib},
};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::{cell::RefCell, fs, hint::black_box, path::Path, rc::Rc};
use yrs::{Transact, updates::decoder::Decode};

static N: usize = 6000;
static SAMPLE_SIZE: usize = 30;

static FILESIZE: &str = "6k";
static FILENAME: &str = "./benches/data/rnd";

fn load(doc: Rc<RefCell<dyn Crdt>>, data: Vec<u8>) {
    let doc2 = doc.borrow().load(&data);
    debug_assert_eq!(doc.borrow().text(), doc2.borrow().text());
}

fn load_from_mem(c: &mut Criterion) {
    let doca = Rc::new(RefCell::new(BenchAM::default()));
    let docy = Rc::new(RefCell::new(BenchYrs::default()));
    let docs: Vec<Rc<RefCell<dyn Crdt>>> = vec![doca, docy];

    let mut group = c.benchmark_group("Time loading doc from memory");
    for doc in docs {
        group.sample_size(SAMPLE_SIZE);

        let bench_name = String::from(doc.borrow().name());
        group.bench_function(BenchmarkId::new("load", bench_name), |b| {
            b.iter_batched(
                || {
                    let doc = { doc.borrow().new() };
                    let text = generate_random_string(N);
                    insert_1b1(&text, 0, doc.clone());
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

fn load_file(doc: Rc<RefCell<dyn Crdt>>, check: bool) {
    let doc = doc.borrow_mut();
    let filename = format!("{}-{}", FILENAME, FILESIZE);
    match doc.crdt_lib() {
        CrdtLib::Automerge => {
            let name = format!("{}.{}", filename, "am");
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
            let name = format!("{}.{}", filename, "yrs");
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

fn load_from_file(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("Time loading doc of size {} from disk", FILESIZE));
    group.sample_size(SAMPLE_SIZE);
    let docs = all_docs();
    for doc in docs {
        let name = doc.borrow().name();
        group.bench_function(BenchmarkId::new("loading doc", name), |b| {
            b.iter(|| load_file(doc.clone(), true))
        });
    }
    group.finish();
}

criterion_group!(benches, load_from_file, load_from_mem);
criterion_main!(benches);
