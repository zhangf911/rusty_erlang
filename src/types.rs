pub type Uint = u64;
pub type Sint = i64;

pub type Uword = Uint;
pub type Sword = Sint;

pub type BeamInstr = Uword;
pub type BeamPtr   = Uint;    // pointer to beam instruction in beam table
pub type Eterm     = Uint;    // erlang term or bit combination for special values
