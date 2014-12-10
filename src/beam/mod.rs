use std::rc::{Rc};
use world;
use beam::raw_beam::RawBeam;

pub mod code;
pub mod op;
pub mod raw_beam;

pub fn load_preloaded(state: &mut world::Erts) {
  let mods = vec!["otp_ring0", "init", "prim_eval", "prim_inet", "prim_file",
                  "zlib", "prim_zip", "erl_prim_loader", "erlang",
                  "erts_internal"];
  for mod_name in mods.iter() {
    let str_mod_name: String = (*mod_name).to_string();
    let mod_atom: Rc<_> = state.atoms.put(&str_mod_name);
    let raw_beam = RawBeam::load(&"preload".to_string(), &str_mod_name);
  }
}
