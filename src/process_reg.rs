use std::collections::HashMap;
use types::{Pid};
use atom::{Atom};

#[allow(dead_code)]
enum RegItem {
  Process(Pid),
  Port(Pid)
}

#[allow(dead_code)]
pub struct ProcessRegistry {
  entries: HashMap<Atom, RegItem>
}
pub type Table = ProcessRegistry;

impl ProcessRegistry {
  pub fn new() -> ProcessRegistry {
    ProcessRegistry{ entries: HashMap::new(), }
  }
}
