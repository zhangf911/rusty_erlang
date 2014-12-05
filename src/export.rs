use std::collections::HashMap;
use code_index::{NUM_CODE_IX};
use types::{MFA};
use beam;

#[allow(dead_code)]
#[deriving(Clone)]
pub struct Export {
  // Pointer to code for function
  pub addressv: Vec<beam::Pointer>,

  // code[0]: Tagged atom for module.
  // code[1]: Tagged atom for function.
  // code[2]: Arity (untagged integer).
  // code[3]: This entry is 0 unless the 'address' field points to it.
  //          Threaded code instruction to load function
  //    (em_call_error_handler), execute BIF (em_apply_bif),
  //    or a breakpoint instruction (op_i_generic_breakpoint).
  // code[4]: Function pointer to BIF function (for BIFs only),
  //    or pointer to threaded code if the module has an
  //    on_load function that has not been run yet, or pointer
  //         to code for function code[3] is a breakpont instruction.
  //    Otherwise: 0.
  pub code:   beam::Code,
}

#[allow(dead_code)]
pub struct ExportTable {
  entries: HashMap<MFA, Export>
}
pub type Table = ExportTable;

impl ExportTable {
  pub fn new() -> ExportTable {
    ExportTable{
      entries: HashMap::new(),
    }
  }

  pub fn find(&self, key: &MFA) -> Result<Export, ()> {
    if !self.entries.contains_key(key) {
      return Err(());
    }
    return Ok(self.entries[*key].clone());
  }
}
