#![crate_id = "rusty_erlang#18.0"]
use std::os;

mod beam_types;
mod erl_init;
mod erl_alloc;

fn main() {
  match erl_init::start(&os::args()) {
  Ok(erlinit)  => println!("ok go!"),
  Err(erlinit) => return
  }
}
