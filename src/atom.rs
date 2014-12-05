use std::collections::{HashMap};
use std::sync::atomic::{AtomicUint, Ordering};

use types::{Eterm, Uint};
use term;

#[allow(dead_code)]
pub struct AtomTable {
  // atomic counter for atom index
  index:    AtomicUint,
  // maps atom name to Eterm
  entries:  HashMap<String, Eterm>,

  pub am_start: Eterm,
}
pub type Table = AtomTable;

impl AtomTable {
  pub fn new() -> AtomTable {
    let mut a = AtomTable{
      index:    AtomicUint::new(0),
      entries:  HashMap::new(),
      am_start: 0,
    };
    a.am_start = a.put(&"start".to_string());
    return a
  }

  // Adds an atom to atom table. Returns index of new element as Eterm
  pub fn put(&mut self, name: &String) -> Eterm {
    if self.entries.contains_key(name) {
      return self.entries[*name];
    }
    let index: uint = self.index.fetch_add(1, Ordering::SeqCst);
    let at: Eterm = term::make_atom(index as Uint);
    self.entries[*name] = at;
    return at;
  }
}
