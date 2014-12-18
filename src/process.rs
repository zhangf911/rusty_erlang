use std::collections::{HashMap};
use types::{ApproxTime, Pid, MFArity};
use {beam, world, term, message};
use std::rc::Rc;

// Keys and values are any eterm
pub type ProcDict = HashMap<term::Eterm, term::Eterm>;


#[allow(dead_code)]
pub struct Process {
  // TODO: separate heaps & gc
  heap:   term::heap::Heap,

  cp: beam::code::Pointer, // weak continuation pointer (for threaded code)
  i:  beam::code::Pointer, // program counter for threaded code

  msg: message::Queue,

  dictionary: ProcDict,      // Process dictionary, may be NULL

  //Initial module(0), function(1), arity(2), often used instead
  //of pointer to funcinfo instruction, hence the BeamInstr datatype
  initial: MFArity,
  initial_args: Rc<term::Eterm>,

  // Current Erlang function, part of the funcinfo:
  // module(0), function(1), arity(2)
  // (module and functions are tagged atoms; arity an untagged integer).
  current: beam::code::Pointer,
  code:    Rc<beam::code::Code>,

  //
  // Information mainly for post-mortem use (erl crash dump)
  //
  parent:         Pid,   // Pid of process that created this process
  approx_started: ApproxTime, // Time when started
}

#[allow(dead_code)]
pub struct ProcessTable {
  entries: HashMap<Pid, Process>
}
pub type Table = ProcessTable;

impl ProcessTable {
  pub fn new() -> ProcessTable {
    ProcessTable{ entries: HashMap::new(), }
  }
}

// Locates start_mod module and spawns root process for the whole thing.
pub fn first_process_otp(state: &mut world::Erts,
                     mod_name: String,
                     _code: Option<Rc<beam::code::Code>>)
{
  let start_mod = state.atoms.put(&mod_name);
  let mfa = MFArity::new(&start_mod, &state.atoms.am_start, 2);
  match state.find_exported_fun(&mfa,
                                state.code_ix.get_active()) {
    None    => panic!(format!("No function {}:start/2", mod_name)),
    Some(_) => return
  }

  let mut p     = Process::new(&term::Eterm::Nil, &mfa);
  let args_vec  = vec! [term::Eterm::Nil,];
  // Create args on new process heap
  let args = p.heap.make_list(&args_vec, &mut state.terms);
  p.set_start_args(&args);
}

impl Process {
  // Creates new process with given Mod,Fun,Args. Process is not registered or
  // started anywhere yet.
  pub fn new(parent: &term::Eterm, mfa: &MFArity) -> Process
  {
    Process{
      heap:       term::heap::Heap::new(),
      cp:         beam::code::Pointer::new_empty(),
      i:          beam::code::Pointer::new_empty(),
      msg:        message::Queue::new(),
      dictionary:     HashMap::new(),

      initial:        mfa.clone(),
      initial_args:   Rc::new(term::Eterm::Nil), // todo: some premade consts?

      code:           beam::code::make_empty_code(),      // fill this
      current:        beam::code::Pointer::new_empty(),   // fill this
      parent:         parent.get_pid(),
      approx_started: 0,      // fill this
    }
  }

  // Begins execution from given m:f, with args a
  pub fn set_start_args(&mut self, args: &Rc<term::Eterm>)
  {
  }
}
