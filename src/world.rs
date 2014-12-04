use {atom, erl_init, fun, module, process, process_reg};
use types::{Uint};

// Global runtime configuration, atoms, tables and so on
#[allow(dead_code)]
pub struct State {
  erl_init:   erl_init::ErlInit,
  atoms:      atom::Table,
  funs:       fun::Table,
  modules:    module::Table,
  processes:  process::Table,
  preg:       process_reg::Table,
  clock:      Uint
}

impl State {
  pub fn new(init: erl_init::ErlInit) -> State {
    State{
      erl_init:   init,
      atoms:      atom::AtomTable::new(),
      funs:       fun::FunTable::new(),
      modules:    module::ModuleTable::new(),
      clock:      0, // TODO: get_time and all that stuff
      preg:       process_reg::ProcessRegistry::new(),
      processes:  process::ProcessTable::new(),
    }
  }
}
