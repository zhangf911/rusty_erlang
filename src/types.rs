use term;

pub type Uint = u64;
pub type Sint = i64;
pub const UINT_SIZEOF: uint = 8; // bytes

pub type Uword = Uint;
pub type Sword = Sint;

// refers to existing Eterm in some heap
// TODO: refer to some eterm heap + offset?
pub type EtermPtr  = Uint;
pub type MFA = (term::Eterm, term::Eterm, uint);

pub type Pid = Uint;

pub type ApproxTime = Uint;
