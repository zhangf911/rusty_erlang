use std::collections::{HashMap};
use std::sync::atomic::{AtomicUint, Ordering};

use types::{Eterm, Uint};
use term;

// Atom is an integer index in atom table
pub type Atom = Eterm;

#[allow(dead_code)]
pub struct AtomTable {
  index:    AtomicUint,
  entries:  HashMap<String, Eterm>
}
pub type Table = AtomTable;

impl AtomTable {
  pub fn new() -> AtomTable {
    AtomTable{
      index: AtomicUint::new(0),
      entries: HashMap::new(),
    }
  }

  pub fn put(&mut self, name: &String) -> Eterm {
    let index: uint = self.index.fetch_add(1, Ordering::SeqCst);
    let at: Eterm = term::make_atom(index as Uint);
    self.entries[*name] = at;
    return at;
  }
}
