use types::{Sint, Pid};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::hash::sip::SipState;
//use num::bigint;

pub mod bits;
pub mod heap;

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
  pid:    Pid,
}
impl PartialEq for EtermPid {
  fn eq(&self, other: &EtermPid) -> bool {
    self.pid == other.pid
  }
}

// Erlang Term, which is bit combination for special values
// Eterm is used as index in EtermIndex, which is a global hashmap for whole
// Erts system (a way to mimic integer/pointer conversion in C BEAM engine).
// Entries in EtermIndex refer to a value stored in some separate process heap.
// Heap stores EtermValues which are Rust enums of several possible types
// (which is a way to mimic "polymorphic" memory structure of Erlang values)
#[deriving(Eq)]
pub enum Eterm {
  Nil,
  List(EtermList),
  Tuple(EtermTuple),
  Binary(EtermBinary),
  Atom(uint),
  Integer(Sint), // TODO: Split into machine int and bigint
  //Float(f64),
  Pid(EtermPid)
}

impl Eterm {
  // attempt to extract atom index from an Eterm, panic if it isn't an atom
  pub fn get_atom(&self) -> uint {
    match *self {
      Eterm::Atom(u) => return u,
      _              => panic!("Eterm::get_atom not an atom")
    }
  }
  // attempt to extract a pid from an Eterm, panic if its not a pid
  pub fn get_pid(&self) -> Pid {
    match *self {
      Eterm::Pid(p) => return p.pid,
      _             => panic!("Eterm::get_pid not a pid")
    }
  }
}

impl PartialEq for Eterm {
  fn eq(&self, other: &Eterm) -> bool {
    match (self, other) {
      (&Eterm::Nil, &Eterm::Nil)                => true,
      (&Eterm::Pid(a), &Eterm::Pid(b))          => a.pid == b.pid,
      (&Eterm::Atom(a), &Eterm::Atom(b))        => a == b,
      (&Eterm::Integer(a), &Eterm::Integer(b))     => a == b,
      (&Eterm::Tuple(ref a), &Eterm::Tuple(ref b)) => *a == *b,
      (&Eterm::List(ref a), &Eterm::List(ref b))   => *a == *b,
      _ => false,
    }
  }
}
// TODO: Reimplement proper hash algorithms from C BEAM for DETS compatibility
impl Hash for Eterm {
  fn hash(&self, state: &mut SipState) {
    match self {
      &Eterm::Nil        => 0_u64.hash(state),
      &Eterm::Atom(u)    => bits::make_atom(u).hash(state),
      &Eterm::Pid(p)     => p.pid.hash(state),
      &Eterm::Integer(i) => i.hash(state),
      &Eterm::List(ref l)    => l.hash(state),
      &Eterm::Tuple(ref t)   => t.hash(state),
      &Eterm::Binary(ref b)  => b.hash(state),
    }
  }
}
