use std::collections::TreeSet;

// Use String as atom, it is slow, redo later
pub type Atom = String;

#[allow(dead_code)]
pub struct AtomTable {
  entries: TreeSet<String>
}
pub type Table = AtomTable;

impl AtomTable {
  pub fn new() -> AtomTable {
    AtomTable{ entries: TreeSet::new(), }
  }
}
