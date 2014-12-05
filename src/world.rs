use {atom, beam, beam_op, erl_init, export, fun, module, process, process_reg};
use types::{Uint, Eterm};
use code_index::{CodeIndex, NUM_CODE_IX};
//use export::ExportTable;

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
    }
  }

  pub fn find_exported_fun(&self, m: Eterm, f: Eterm, a: uint, code_ix: uint)
      -> Result<export::Export, ()> {
    match self.exports[code_ix].find(&(m, f, a)) {
      Ok(export) => {
          if export.addressv[code_ix].equals(&export.code, 3)
            && export.code[3] != beam_op::op_i_generic_breakpoint {
            return Err(());
          }
          return Ok(export)
        },
      Err(()) => Err(())
    }
  }
}
