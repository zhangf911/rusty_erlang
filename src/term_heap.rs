use std::collections::{HashMap};
use std::rc::Rc;
use term;

// Heap stores different types of Erlang objects type-safely
// Heap is referred from EtermIndex using HeapPtr's which are refcounted
pub struct Heap {
  terms:   HashMap<term::Eterm, term::Eterm>,
}

pub type HeapPtr = Rc<Heap>;

// Global Eterm to heap mapping. When some term is created on some heap, it is
// registered here (a Erts-global field) and it points to that heap. When heap
// is destroyed, values are dropped from global index too (ugly solution for now
// until we give up Rust memory management and implement our own).
pub type EtermIndex = HashMap<term::Eterm, HeapPtr>;
pub fn new_eterm_index() -> EtermIndex {
  HashMap::new()
}

impl Heap {
  pub fn new() -> Heap {
    Heap{
      terms: HashMap::new()
    }
  }

  pub fn make_list(&mut self, values: &Vec<term::Eterm>) -> term::Eterm {
    return term::Eterm::Nil
  }
}
