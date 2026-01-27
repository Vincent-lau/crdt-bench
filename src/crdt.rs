use std::{any::Any, cell::RefCell, fmt, rc::Rc};

#[derive(Debug)]
pub enum CrdtLib {
    Automerge,
    Yrs,
}

impl fmt::Display for CrdtLib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CrdtLib::Automerge => "Automerge",
            CrdtLib::Yrs => "Yjs",
        };
        write!(f, "{}", name)
    }
}

pub trait Crdt: Any {
    fn new(&self) -> Rc<RefCell<dyn Crdt>>;

    fn load(&self, data: &[u8]) -> Rc<RefCell<dyn Crdt>>;

    fn crdt_lib(&self) -> CrdtLib;

    fn name(&self) -> String {
        self.crdt_lib().to_string()
    }

    fn encoded_state(&mut self) -> Vec<u8>;

    fn apply_update(&mut self, update: &[u8]);

    fn insert_text(&mut self, index: usize, text: &str);

    /// operations will return the update
    fn insert_text_update(&mut self, index: usize, text: &str) -> Vec<u8>;

    fn delete_text(&mut self, index: usize, len: u32);

    fn text(&self) -> String;
}
