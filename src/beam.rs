use std::rc::Rc;
use types::{Uword, Uint};

pub type Instr    = Uword;
pub type InstrVec = Vec<Instr>;
pub type Code     = Rc<InstrVec>; // refcounted

// pointer to beam instruction in beam table
#[deriving(Clone)]
pub struct Pointer {
  code:   Code, // refcounted
  offset: Uint,
}

impl Pointer {
  pub fn equals(&self, code: &Code, offset: Uint) -> bool {
    return self.code == *code && self.offset == offset;
  }
  pub fn get_op(&self, offset: uint) -> Instr {
    return self.code[offset];
  }
}
