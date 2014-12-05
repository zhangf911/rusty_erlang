use std::collections::{HashMap};
use std::sync::atomic::{AtomicUint, Ordering};

use types::{Uint};
use term;

#[allow(dead_code)]
pub struct AtomTable {
  // atomic counter for atom index
  index:    AtomicUint,
  // maps atom name to Eterm
  entries:  HashMap<String, Box<term::Eterm>>,

  pub am_start: Box<term::Eterm>,
}
pub type Table = AtomTable;

impl AtomTable {
  pub fn new() -> AtomTable {
    let mut a = AtomTable{
      index:    AtomicUint::new(0),
      entries:  HashMap::new(),
      am_start: box term::Eterm::Nil,
    };
    a.am_start = a.put(&"start".to_string());
    return a
  }

  // Adds an atom to atom table. Returns boxed atom Eterm (containing index)
  pub fn put(&mut self, name: &String) -> Box<term::Eterm> {
    match self.entries.get(name) {
      Some(x) => return *x,
      None    => ()
    }
    let index: uint = self.index.fetch_add(1, Ordering::SeqCst);
    let at = box term::Eterm::Atom(term::make_atom(index));
    self.entries.insert(*name, at);
    return at;
  }
}
