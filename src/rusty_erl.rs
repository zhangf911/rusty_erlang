//#![crate_id = "rusty_erlang#18.0"]
#[allow(unused_variables)]
use std::os;

mod alloc;
mod atom;
mod beam;
mod code_index;
mod erl_init;
mod export;
mod fun;
mod module;
mod process;
mod process_reg;
mod term;
mod types;
mod world;

fn main() {
  match erl_init::start(&os::args()) {
  Ok(_world)  => println!("ok go!"),
  Err(()) => return
  }
}
