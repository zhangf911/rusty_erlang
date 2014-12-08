use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::hash::sip::SipState;
use std::rc::{Rc, Weak};
use term;

pub type Uint = u64;
pub type Sint = i64;
pub const UINT_SIZEOF: uint = 8; // bytes

pub type Uword = Uint;
pub type Sword = Sint;

// refers to existing Eterm in some heap
// TODO: refer to some eterm heap + offset?
//pub type EtermPtr  = Uint;
#[deriving(Eq)]
pub struct MFArity {
  m:      Rc<term::Eterm>,
  f:      Rc<term::Eterm>,
  arity:  uint,
}
impl Clone for MFArity {
  fn clone(&self) -> MFArity {
    MFArity{
      m: self.m.clone(),
      f: self.f.clone(),
      arity: self.arity,
    }
  }
  fn clone_from(&mut self, source: &MFArity) {
    self.m = source.m.clone();
    self.f = source.f.clone();
    self.arity = source.arity;
  }
}
impl MFArity {
  pub fn new(m: &Rc<term::Eterm>, f: &Rc<term::Eterm>, arity: uint) -> MFArity {
    MFArity{
      m:      m.clone(),
      f:      f.clone(),
      arity:  arity,
    }
  }
}
impl PartialEq for MFArity {
  fn eq(&self, other: &MFArity) -> bool {
    self.m == other.m && self.f == other.f && self.arity == other.arity
  }
}
impl Hash for MFArity {
  fn hash(&self, state: &mut SipState) {
    self.m.hash(state);
    self.f.hash(state);
    self.arity.hash(state);
  }
}

pub type Pid = Uint;

pub type ApproxTime = Uint;
