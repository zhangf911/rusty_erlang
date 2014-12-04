pub type Uint = u64;
pub type Sint = i64;

pub type Uword = Uint;
pub type Sword = Sint;

pub type BeamInstr = Uword;

// pointer to beam instruction in beam table
// TODO: refer to some beam storage object + offset?
pub type BeamPtr   = Uint;

// erlang term or bit combination for special values
pub type Eterm     = Uint;
// refers to existing Eterm in some heap
// TODO: refer to some eterm heap + offset?
pub type EtermPtr  = Uint;

pub type Pid = Uint;

pub type ApproxTime = Uint;
