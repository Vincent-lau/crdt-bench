use std::{cell::RefCell, rc::Rc};

use yrs::{
    Doc, GetString, ReadTxn, StateVector, Subscription, Text, Transact, Transaction, TransactionAcqError, TransactionMut, Update, UpdateEvent, updates::{decoder::Decode, encoder::Encode}
};

use crate::crdt::{Crdt, CrdtLib};

pub struct BenchYrs {
    doc: Doc,
}

impl Default for BenchYrs {
    fn default() -> Self {
        Self { doc: Doc::new() }
    }
}

impl Crdt for BenchYrs {
    fn new(&self) -> Rc<RefCell<dyn Crdt>> {
        Rc::new(RefCell::new(BenchYrs::default()))
    }

    fn load(&self, data: &[u8]) -> Rc<RefCell<dyn Crdt>> {
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

    fn delete_text(&mut self, _index: usize, _len: u32) {
        todo!()
    }

    fn text(&self) -> String {
        self.doc
            .get_or_insert_text("text")
            .get_string(&self.doc.transact())
    }

    fn crdt_lib(&self) -> crate::crdt::CrdtLib {
        CrdtLib::Yrs
    }
}

impl BenchYrs {
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

    pub fn encode_state_as_update_v1_from_start(&self) -> Vec<u8> {
        self.doc.transact().encode_state_as_update_v1(&StateVector::default())
    }

    pub fn get_changes(&self, state_vector: &StateVector) -> Vec<u8> {
        self.doc.transact().encode_diff_v1(state_vector)
    }

}
