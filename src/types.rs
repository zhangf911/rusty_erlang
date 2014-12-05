pub type Uint = u64;
pub type Sint = i64;
pub const UINT_SIZEOF: uint = 8; // bytes

pub type Uword = Uint;
pub type Sword = Sint;

// erlang term or bit combination for special values
pub type Eterm     = Uint;
// refers to existing Eterm in some heap
// TODO: refer to some eterm heap + offset?
pub type EtermPtr  = Uint;
pub type MFA = (Eterm, Eterm, uint);

pub type Pid = Uint;

pub type ApproxTime = Uint;
