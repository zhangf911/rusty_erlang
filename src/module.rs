use std::collections::HashMap;
use types::{BeamPtr, Uint};
use atom::Atom;

#[allow(dead_code)]
pub struct ModuleInstance {
  code: BeamPtr,
  code_length: Uint,    // Length of loaded code in bytes
  //catches: Uint,
  //struct erl_module_nif *nif;
  //int num_breakpoints;
  //int num_traced_exports;
}

#[allow(dead_code)]
pub struct Module {
  module: Atom,     // Atom for module (not tagged) TODO: Use Eterm?

  curr: ModuleInstance,
  old: ModuleInstance, // protected by "old_code" rwlock
}

#[allow(dead_code)]
pub struct ModuleTable {
  entries:  HashMap<String, Module>
}
pub type Table = ModuleTable;

impl ModuleTable {
  pub fn new() -> ModuleTable {
    ModuleTable{ entries: HashMap::new(), }
  }
}
