use std::rc::{Rc, Weak};
use types::{Uword, Uint};

pub type Instr  = Uword;
pub type Code   = Vec<Instr>;

pub fn make_empty_code() -> Rc<Code> {
  Rc::new(Vec::new())  // ugly!
}
pub fn make_empty_weak_code() -> Weak<Code> {
  Rc::new(Vec::new()).downgrade()  // ugly! will create instantly invalid weak
}

// Weak refcounted pointer to beam instruction in beam table. Will not prevent
// destruction of refcounted data, may resolve into None if data has been
// destroyed
#[deriving(Clone)]
pub struct Pointer {
  code:   Weak<Code>, // refcounted weak, may fail if Rc is destroyed
  offset: Uint,
}

impl Pointer {
  pub fn new_empty() -> Pointer {
    Pointer{
      code:   make_empty_weak_code(),
      offset: 0,
    }
  }

  pub fn equals(&self, code: &Rc<Code>, offset: Uint) -> bool {
    match self.code.upgrade() {
      Some(rc_code) =>
        return rc_code == *code && self.offset == offset,
      None =>
        false
    }
  }

  // Attempts to upgrade weak self to strong Rc pointer, may fail returning None
  // Destroy this as soon as you finished using it, prefer storing weaks instead
  pub fn upgrade(&self) -> Option<StrongPointer> {
    match self.code.upgrade() {
      Some(rc_code) => Some(StrongPointer{
                         code: rc_code,
                         offset: self.offset,
                       }),
      None => None
    }
  }
}

// Stronger code pointer which stores Refcounted code ref, use this temporarily
// for code operations and drop when not needed. Try store weak Pointers where
// possible for longer periods of time.
pub struct StrongPointer {
  code:   Rc<Code>, // refcounted weak, may fail if Rc is destroyed
  offset: Uint,
}
impl StrongPointer {
  pub fn get_op(&self, offset: uint) -> Instr {
    return self.code[offset];
  }
}
