//#![crate_id = "rusty_erlang#18.0"]
#[allow(unused_variables)]
use std::os;

mod alloc;
mod atom;
mod types;
mod erl_init;
mod fun;
mod module;
mod process;
mod process_reg;
mod term;
mod world;

fn main() {
  match erl_init::start(&os::args()) {
  Ok(_world)  => println!("ok go!"),
  Err(()) => return
  }
}
