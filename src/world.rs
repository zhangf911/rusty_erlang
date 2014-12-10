use {atom, beam, erl_init, export, fun, module, process, process_reg};
use types::{Uint, MFArity};
use term;
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
  pub terms:      term::heap::EtermIndex,
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
      terms:      term::heap::new_eterm_index(),
    }
  }

  // Find the export entry for a loaded function.
  // Returns None if the given function is not loaded, or a pointer to the
  // export entry.
  //
  // Note: This function never returns export entries for BIFs
  // or functions which are not yet loaded.  This makes it suitable
  // for use by the erlang:function_exported/3 BIF or whenever you
  // cannot depend on the error_handler.
  pub fn find_exported_fun(&self,
                          mfa: &MFArity,
                          code_ix: uint)
      -> Option<export::Export> {
    match self.exports[code_ix].find(mfa) {
      Some(export) => {
          if export.addressv[code_ix].points_to(&export.code, 3)
            && export.code[3] != beam::op::OP_I_GENERIC_BREAKPOINT {
            return None;
          }
          return Some(export)
        },
      None => None
    }
  }
}
