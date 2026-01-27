use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{bam::BenchAM, byrs::BenchYrs, crdt::Crdt};

pub fn am_doc() -> Rc<RefCell<BenchAM>> {
    Rc::new(RefCell::new(BenchAM::default()))
}

pub fn yrs_doc() -> Rc<RefCell<BenchYrs>> {
    Rc::new(RefCell::new(BenchYrs::default()))
}

pub fn all_docs() -> Vec<Rc<RefCell<dyn Crdt>>> {
    vec![am_doc(), yrs_doc()]
}

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.random_range(0..CHARSET.len());
        result.push(CHARSET[idx] as char);
    }
    result
}

pub fn insert_1b1(text: &str, index: usize, doc: Rc<RefCell<dyn Crdt>>) {
    let mut doc = doc.borrow_mut();
    for (i, c) in text.chars().enumerate() {
        doc.insert_text(index + i, &c.to_string());
    }
}

pub fn insert1big(text: &str, index: usize, doc: Rc<RefCell<dyn Crdt>>) {
    let mut doc = doc.borrow_mut();
    doc.insert_text(index, &text);
}

pub fn insert1big_with_update(text: &str, index: usize, doc: Rc<RefCell<dyn Crdt>>) -> Vec<u8> {
    let mut doc = doc.borrow_mut();
    doc.insert_text_update(index, &text)
}

pub fn insert_1b1_with_updates(
    text: &str,
    index: usize,
    doc: Rc<RefCell<dyn Crdt>>,
) -> Vec<Vec<u8>> {
    let mut doc = doc.borrow_mut();
    let mut updates = Vec::new();
    for (i, c) in text.chars().enumerate() {
        let update = doc.insert_text_update(index + i, &c.to_string());
        updates.push(update);
    }
    updates
}
