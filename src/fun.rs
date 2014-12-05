use std::collections::HashMap;
use std::sync::atomic::AtomicInt;
use types::Uint;
use beam;
use term;

#[allow(dead_code)]
pub struct FunEntry {
  module_md5: String,         // md5 for module
  index:      Uint,           // new style index
  address:    beam::Pointer,  // pointer to code for fun
  arity:      Uint,
  module:     term::Eterm,    // tagged atom for module
  refc:       AtomicInt, // Reference count: One for code + one for each
                         // fun object in each process
}

#[allow(dead_code)]
pub struct FunTable {
  entries:  HashMap<String, FunEntry>
}
pub type Table = FunTable;

impl FunTable {
  pub fn new() -> FunTable {
    FunTable{ entries: HashMap::new(), }
  }
}
