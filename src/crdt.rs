use std::{cell::RefCell, rc::Rc};

pub trait Crdt {
    fn new(&self) -> Rc<RefCell<dyn Crdt>>;

    fn load(&self, data: Vec<u8>) -> Rc<RefCell<dyn Crdt>>;

    fn name(&self) -> &str;

    fn encoded_state(&mut self) -> Vec<u8>;

    fn apply_update(&mut self, update: Vec<u8>);

    fn insert_text(&mut self, index: usize, text: &str);

    /// operations will return the update
    fn insert_text_update(&mut self, index: usize, text: &str) -> Vec<u8>;

    fn delete_text(&mut self, index: usize, len: u32);

    fn text(&self) -> String;
}
