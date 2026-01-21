use std::hint::black_box;

use rand::Rng;

use crate::{bam::BenchAM, crdt::Crdt};

// #[cfg(feature = "dhat-heap")]
// #[global_allocator]
// static ALLOC: dhat::Alloc = dhat::Alloc;

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.random_range(0..CHARSET.len());
        result.push(CHARSET[idx] as char);
    }
    result
}



pub fn run() {
    let mut doc = BenchAM::default();

    let text = generate_random_string(600);

    for (i, c) in text.chars().enumerate() {
        doc.insert_text(i, &c.to_string());
    }
    let compressed = doc.encoded_state();

    let doc_again = doc.load(compressed);
    black_box(doc_again);
    println!("{}", doc.text());
}
