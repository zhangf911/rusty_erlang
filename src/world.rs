use {atom, beam_op, erl_init, export, fun, module, process, process_reg};
use {term_heap};
use types::{Uint, Eterm};
use code_index::{CodeIndex};

// Global runtime configuration, atoms, tables and so on
#[allow(dead_code)]
pub struct Erts {
  pub init:   erl_init::ErlInit,

  pub atoms:  atom::Table,
  funs:       fun::Table,
  modules:    module::Table,
  exports:    Vec<export::ExportTable>,

  processes:  process::Table,
  preg:       process_reg::Table,
  clock:      Uint,
  pub code_ix:  CodeIndex,
  // Term index maps Eterm value range to heaps of processes
  terms:      term_heap::EtermIndex,
}

impl Erts {
  pub fn new(init: erl_init::ErlInit) -> Erts {
    Erts{
      init:       init, // startup params and command line args

      atoms:      atom::AtomTable::new(),
      funs:       fun::FunTable::new(),
      modules:    module::ModuleTable::new(),
      exports:    vec![export::ExportTable::new(),
                   export::ExportTable::new(),
                   export::ExportTable::new()],

      clock:      0, // TODO: get_time and all that stuff
      preg:       process_reg::ProcessRegistry::new(),
      processes:  process::ProcessTable::new(),
      code_ix:    CodeIndex::new(),
      terms:      term_heap::new_eterm_index(),
    }
  }

  pub fn find_exported_fun(&self, m: Eterm, f: Eterm, a: uint, code_ix: uint)
      -> Result<export::Export, ()> {
    match self.exports[code_ix].find(&(m, f, a)) {
      Ok(export) => {
          if export.addressv[code_ix].equals(&export.code, 3)
            && export.code[3] != beam_op::OP_I_GENERIC_BREAKPOINT {
            return Err(());
          }
          return Ok(export)
        },
      Err(()) => Err(())
    }
  }
}
