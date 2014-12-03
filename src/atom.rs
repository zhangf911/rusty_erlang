use std::collections::TreeSet;

#[allow(dead_code)]
pub struct AtomTable {
  entries: TreeSet<String>
}

impl AtomTable {
  pub fn new() -> AtomTable {
    AtomTable{ entries: TreeSet::new(), }
  }
}
