use types::{Sint, Pid};
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::hash::sip::SipState;
//use num::bigint;

pub mod bits;
pub mod heap;
mod detail;

// Erlang Term, which is bit combination for special values
// Eterm is used as index in EtermIndex, which is a global hashmap for whole
// Erts system (a way to mimic integer/pointer conversion in C BEAM engine).
// Entries in EtermIndex refer to a value stored in some separate process heap.
// Heap stores EtermValues which are Rust enums of several possible types
// (which is a way to mimic "polymorphic" memory structure of Erlang values)
#[deriving(Eq)]
pub enum Eterm {
  Nil,
  List(detail::EtermList),
  Tuple(detail::EtermTuple),
  Binary(detail::EtermBinary),
  Atom(uint),
  Integer(Sint), // TODO: Split into machine int and bigint
  //Float(f64),
  Pid(detail::EtermPid)
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
      Eterm::Pid(ref p) => return p.pid,
      _                 => panic!("Eterm::get_pid not a pid")
    }
  }
}

impl PartialEq for Eterm {
  fn eq(&self, other: &Eterm) -> bool {
    match *self {
      Eterm::Nil =>
        match *other {
          Eterm::Nil => true,
          _          => false
        },
      Eterm::Pid(ref a) =>
        match *other {
          Eterm::Pid(ref b) => a.pid == b.pid,
          _                 => false
        },
      Eterm::Atom(a) =>
        match *other {
          Eterm::Atom(b) => a == b,
          _              => false
        },
      Eterm::Integer(a) =>
        match *other {
          Eterm::Integer(b) => a == b,
          _                 => false,
        },
      Eterm::Tuple(ref a) =>
        match *other {
          Eterm::Tuple(ref b) => *a == *b,
          _                   => false,
        },
      Eterm::List(ref a) =>
        match *other {
          Eterm::List(ref b)  => *a == *b,
          _                   => false,
        },
      _ => false,
    }
  }
}
// TODO: Reimplement proper hash algorithms from C BEAM for DETS compatibility
impl Hash for Eterm {
  fn hash(&self, state: &mut SipState) {
    match *self {
      Eterm::Nil            => 0_u64.hash(state),
      Eterm::Atom(u)        => bits::make_atom(u).hash(state),
      Eterm::Pid(ref p)     => p.pid.hash(state),
      Eterm::Integer(i)     => i.hash(state),
      Eterm::List(ref l)    => l.hash(state),
      Eterm::Tuple(ref t)   => t.hash(state),
      Eterm::Binary(ref b)  => b.hash(state),
    }
  }
}
