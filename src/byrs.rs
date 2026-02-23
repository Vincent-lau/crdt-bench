use std::{cell::RefCell, rc::Rc};

use yrs::{
    Doc, GetString, Options, ReadTxn, StateVector, Subscription, Text, TextRef, Transact,
    Transaction, TransactionAcqError, TransactionMut, Update, UpdateEvent,
    updates::{decoder::Decode, encoder::Encode},
};

use crate::crdt::{Crdt, CrdtLib, DocRef};

pub struct BenchYrs {
    doc: Doc,
}

impl Default for BenchYrs {
    fn default() -> Self {
        Self { doc: Doc::new() }
    }
}

impl Crdt for BenchYrs {
    fn new(&self) -> DocRef {
        Rc::new(RefCell::new(BenchYrs::default()))
    }

    fn load(&self, data: &[u8]) -> DocRef {
        let doc = Doc::new();
        doc.transact_mut()
            .apply_update(Update::decode_v1(data).expect("decode data fine"))
            .unwrap();
        Rc::new(RefCell::new(BenchYrs { doc }))
    }

    fn encoded_state(&mut self) -> Vec<u8> {
        self.doc
            .transact_mut()
            .encode_state_as_update_v1(&StateVector::default())
    }

    fn apply_update(&mut self, update: &[u8]) {
        self.doc
            .transact_mut()
            .apply_update(Update::decode_v1(update).expect("decode data fine"))
            .unwrap();
    }

    fn insert_text(&mut self, index: usize, text: &str) {
        let btext = self.doc.get_or_insert_text("text");
        let mut txn = self.doc.transact_mut();
        btext.insert(&mut txn, index as u32, text);
    }

    fn insert_text_update(&mut self, index: usize, text: &str) -> Vec<u8> {
        let btext = self.doc.get_or_insert_text("text");
        let mut txn = self.doc.transact_mut();
        btext.insert(&mut txn, index as u32, text);
        txn.encode_update_v1()
    }

    fn delete_text(&mut self, index: usize, len: isize) {
        let btext = self.doc.get_or_insert_text("text");
        let mut txn = self.doc.transact_mut();
        btext.remove_range(&mut txn, index as u32, len as u32);
    }

    fn insert_delete_text(&mut self, index: usize, del: isize, text: &str) {
        self.delete_text(index, del);
        self.insert_text(index, text);
    }

    fn text(&self) -> String {
        self.doc
            .get_or_insert_text("text")
            .get_string(&self.doc.transact())
    }

    fn crdt_lib(&self) -> crate::crdt::CrdtLib {
        CrdtLib::Yrs
    }

    fn fork(&mut self) -> crate::crdt::DocRef {
        let doc2 = BenchYrs::default();
        let ts = doc2.transact().state_vector();
        let update = self.doc.transact().encode_diff_v1(&ts);
        doc2.transact_mut()
            .apply_update(Update::decode_v1(&update).unwrap())
            .unwrap();
        Rc::new(RefCell::new(doc2))
    }

    fn merge(&mut self, other: Rc<RefCell<dyn std::any::Any>>) {
        let mut other = other.borrow_mut();
        if let Some(doc) = other.downcast_mut::<BenchYrs>() {
            let mut txn = self.doc.transact_mut();
            let ts = txn.state_vector();
            let update = doc.transact().encode_diff_v1(&ts);
            txn.apply_update(Update::decode_v1(&update).unwrap())
                .unwrap();
        } else {
            unreachable!("Incorrect doc type");
        }
    }
}

impl BenchYrs {
    pub fn new_no_gc() -> Self {
        BenchYrs {
            doc: Doc::with_options(Options {
                skip_gc: true,
                ..Options::default()
            }),
        }
    }

    pub fn observe_update_v1<F>(&self, f: F) -> Result<Subscription, TransactionAcqError>
    where
        F: Fn(&TransactionMut<'_>, &UpdateEvent) + 'static,
    {
        self.doc.observe_update_v1(f)
    }

    pub fn checkpoint(&self) -> (Vec<u8>, Vec<u8>) {
        let txn = self.doc.transact();
        (
            txn.state_vector().encode_v1(),
            txn.encode_state_as_update_v1(&StateVector::default()),
        )
    }

    pub fn transact(&self) -> Transaction<'_> {
        self.doc.transact()
    }

    pub fn transact_mut(&self) -> TransactionMut<'_> {
        self.doc.transact_mut()
    }

    pub fn encode_state_as_update_v1_from_start(&self) -> Vec<u8> {
        self.doc
            .transact()
            .encode_state_as_update_v1(&StateVector::default())
    }

    pub fn get_changes(&self, state_vector: &StateVector) -> Vec<u8> {
        self.doc.transact().encode_diff_v1(state_vector)
    }

    pub fn get_text_obj(&self) -> TextRef {
        self.doc.get_or_insert_text("text")
    }
}
