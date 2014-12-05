use std::sync::atomic::{AtomicUint, Ordering};

pub const NUM_CODE_IX: uint = 3;

pub struct CodeIndex {
  staging: AtomicUint,
  active: AtomicUint,
}

impl CodeIndex {
  pub fn new() -> CodeIndex {
    CodeIndex {
      staging:  AtomicUint::new(0),
      active:   AtomicUint::new(0),
    }
  }

  pub fn get_active(&self) -> uint {
    return self.active.load(Ordering::Relaxed);
  }
}
