pub type Uint = u64;
pub type Sint = i64;
pub const UINT_SIZEOF: uint = 8; // bytes

pub type Uword = Uint;
pub type Sword = Sint;

// Erlang Term, which is bit combination for special values
// Eterm is used as index in EtermIndex, which is a global hashmap for whole
// Erts system (a way to mimic integer/pointer conversion in C BEAM engine).
// Entries in EtermIndex refer to a value stored in some separate process heap.
// Heap stores EtermValues which are Rust enums of several possible types
// (which is a way to mimic "polymorphic" memory structure of Erlang values)
pub type Eterm     = Uint;

// refers to existing Eterm in some heap
// TODO: refer to some eterm heap + offset?
pub type EtermPtr  = Uint;
pub type MFA = (Eterm, Eterm, uint);

pub type Pid = Uint;

pub type ApproxTime = Uint;
