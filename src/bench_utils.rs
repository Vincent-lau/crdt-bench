use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{bam::BenchAM, byrs::BenchYrs, crdt::Crdt};

pub fn am_doc() -> Rc<RefCell<dyn Crdt>> {
    Rc::new(RefCell::new(BenchAM::default()))
}

pub fn yrs_doc() -> Rc<RefCell<dyn Crdt>> {
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