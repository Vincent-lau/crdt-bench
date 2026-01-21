use std::fs;
use std::hint::black_box;

use crdt_bench::bam::BenchAM;
use crdt_bench::byrs::BenchYrs;
use crdt_bench::crdt::Crdt;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[derive(Debug)]
enum CrdtLib {
    Automerge,
    Yrs,
}

fn bench_mem(lib: CrdtLib) {
    let file_to_load = match lib {
        CrdtLib::Automerge => "./benches/data/b1.am",
        CrdtLib::Yrs => "./benches/data/b1.yrs",
    };

    let data = fs::read(file_to_load).unwrap();
    let doc: Box<dyn Crdt> = match lib {
        CrdtLib::Automerge => Box::new(BenchAM::default()),
        CrdtLib::Yrs => Box::new(BenchYrs::default()),
    };

    let doc = doc.load(data);
    black_box(doc);
}

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    let lib = CrdtLib::Automerge;

    println!("memory profiling for {:?}", lib);
    bench_mem(lib);
}
