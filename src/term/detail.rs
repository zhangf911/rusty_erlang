use std::hash::Hash;
use std::hash::sip::SipState;

use types::Pid;
use term::Eterm;

// TODO: Redo following Eterm* types with byte-precision on the heap
#[deriving(Eq)]
pub struct EtermList {
  value: Box<Eterm>,
  tail:  Box<Eterm>,
}
impl PartialEq for EtermList {
  fn eq(&self, other: &EtermList) -> bool {
    self.value == other.value && self.tail == other.tail
  }
}
impl Hash for EtermList {
  fn hash(&self, state: &mut SipState) {
    self.value.hash(state);
    self.tail.hash(state);
  }
}

#[deriving(Eq)]
pub struct EtermBinary {
  bytes: Vec<u8>,
}
impl PartialEq for EtermBinary {
  fn eq(&self, other: &EtermBinary) -> bool {
    self.bytes == other.bytes
  }
}
impl Hash for EtermBinary {
  fn hash(&self, state: &mut SipState) {
    self.bytes.hash(state);
  }
}

#[deriving(Eq)]
pub struct EtermTuple {
  values: Box<Vec<Eterm>>,
}
impl PartialEq for EtermTuple {
  fn eq(&self, other: &EtermTuple) -> bool {
    self.values == other.values
  }
}
impl Hash for EtermTuple {
  fn hash(&self, state: &mut SipState) {
    self.values.hash(state);
  }
}

#[deriving(Eq)]
pub struct EtermPid {
  pub pid:    Pid,
}
impl PartialEq for EtermPid {
  fn eq(&self, other: &EtermPid) -> bool {
    self.pid == other.pid
  }
}
