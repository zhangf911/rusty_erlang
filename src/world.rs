use {erl_init, atom, fun};

// Global runtime configuration, atoms, tables and so on
#[allow(dead_code)]
pub struct State {
  erl_init: erl_init::ErlInit,
  atoms:    atom::AtomTable,
  funs:     fun::FunTable,
}

impl State {
  pub fn new(init: erl_init::ErlInit) -> State {
    State{
      erl_init: init,
      atoms:  atom::AtomTable::new(),
      funs:   fun::FunTable::new(),
    }
  }
}