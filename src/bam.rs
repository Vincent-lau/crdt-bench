use std::{cell::RefCell, rc::Rc};

use automerge::{AutoCommit, ObjId, ReadDoc, transaction::Transactable};

use crate::crdt::Crdt;

pub struct BenchAM {
    doc: AutoCommit,
    btext: ObjId,
    // barray: Option<ObjId>,
    // bmap: Option<ObjId>,
}

impl Default for BenchAM {
    fn default() -> Self {
        let mut doc = AutoCommit::new();
        // let barray = doc
        //     .put_object(automerge::ROOT, "barray", automerge::ObjType::List)
        //     .unwrap();
        // let bmap = doc
        //     .put_object(automerge::ROOT, "bmap", automerge::ObjType::Map)
        //     .unwrap();
        let btext = doc
            .put_object(automerge::ROOT, "btext", automerge::ObjType::Text)
            .unwrap();

        Self { doc, btext: btext }
    }
}

impl Crdt for BenchAM {
    fn new(&self) -> Rc<RefCell<dyn Crdt>> {
        Rc::new(RefCell::new(BenchAM::default()))
    }

    fn load(&self, data: Vec<u8>) -> Rc<RefCell<dyn Crdt>> {
        let doc = AutoCommit::load(&data).expect("loading doc success");
        let btext = doc.get(automerge::ROOT, "btext").unwrap().unwrap().1;
        Rc::new(RefCell::new(Self { doc, btext }))
    }

    fn name(&self) -> &str {
        "Automerge"
    }

    fn encoded_state(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    fn apply_update(&mut self, update: Vec<u8>) {
        self.doc
            .load_incremental(&update)
            .expect("apply update success");
    }

    fn insert_text(&mut self, index: usize, text: &str) {
        self.doc
            .splice_text(self.btext.clone(), index, 0, text)
            .unwrap();
    }

    fn insert_text_update(&mut self, index: usize, text: &str) -> Vec<u8> {
        self.insert_text(index, text);
        self.doc.save_incremental()
    }

    fn delete_text(&mut self, _index: usize, _len: u32) {
        todo!()
    }

    fn text(&self) -> String {
        self.doc.text(self.btext.clone()).unwrap()
    }
}
