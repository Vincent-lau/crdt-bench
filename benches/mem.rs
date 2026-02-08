use std::fs;
use std::hint::black_box;

use crdt_bench::bam::BenchAM;
use crdt_bench::byrs::BenchYrs;
use crdt_bench::crdt::{Crdt, CrdtLib};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

static FILE_SZ: &str = "6k";

fn bench_mem(lib: CrdtLib) {
    let file_to_load = match lib {
        CrdtLib::Automerge => format!("./benches/data/rnd-{}.am", FILE_SZ),
        CrdtLib::Yrs => format!("./benches/data/rnd-{}.yrs", FILE_SZ),
    };

    let data = fs::read(file_to_load).unwrap();
    let doc: Box<dyn Crdt> = match lib {
        CrdtLib::Automerge => Box::new(BenchAM::default()),
        CrdtLib::Yrs => Box::new(BenchYrs::default()),
    };

    let doc = doc.load(&data);
    let stats = dhat::HeapStats::get();
    println!("current allocation {:?}", stats);

    black_box(doc);
}

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    let lib = CrdtLib::Yrs;

    println!(
        "memory profiling for {:?} by loading file size {}",
        lib, FILE_SZ
    );
    bench_mem(lib);
}
