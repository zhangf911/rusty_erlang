type Eterm = u64;

type Uint = u64;
type Sint = i64;

type Uword = Uint;
type Sword = Sint;
type BeamInstr = Uword;

struct Preload {
  name: String,   // Name of module 
  size: Uint,     // Size of code 
  code: Vec<u8>,  // Code pointer 
}
