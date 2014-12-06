use std::collections::{HashMap};
use types::{ApproxTime, Pid, MFArity};
use {beam, world, term, term_heap, message};

// Keys and values are any eterm
pub type ProcDict = HashMap<term::Eterm, term::Eterm>;


#[allow(dead_code)]
pub struct Process {
  // TODO: separate heaps & gc
  heap:   term_heap::Heap,

  cp: beam::Pointer,   // (untagged) Continuation pointer (for threaded code)
  i:  beam::Pointer,   // Program counter for threaded code

  msg: message::Queue,

  dictionary: ProcDict,      // Process dictionary, may be NULL

  //Initial module(0), function(1), arity(2), often used instead
  //of pointer to funcinfo instruction, hence the BeamInstr datatype
  initial: beam::Code,
  // Current Erlang function, part of the funcinfo:
  // module(0), function(1), arity(2)
  // (module and functions are tagged atoms;
  // arity an untagged integer). BeamInstr * because it references code
  current: beam::Pointer,

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
                     _code: Option<beam::Code>)
                     -> Result<(), String>
{
  let start_mod = state.atoms.put(&mod_name);
  let mfa = MFArity::new(&start_mod, &state.atoms.am_start, 2);
  match state.find_exported_fun(&mfa,
                                state.code_ix.get_active()) {
    Err(())    => return Err("No function ".to_string() + mod_name + ":start/2"),
    Ok(_export) => {}
  }

  let mut p     = Process::new(&term::Eterm::Nil);
  let args_vec  = vec! [term::Eterm::Nil,];
  let args = p.heap.make_list(&args_vec, &mut state.terms);
  p.start_from(&*start_mod, &*state.atoms.am_start, &*args);
  return Ok(())
}

impl Process {
  // Creates new process with given Mod,Fun,Args. Process is not registered or
  // started anywhere yet.
  pub fn new(parent: &term::Eterm) -> Process
  {
    Process{
      heap:       term_heap::Heap::new(),
      cp:         beam::Pointer::new_empty(),
      i:          beam::Pointer::new_empty(),
      msg:        message::Queue::new(),
      dictionary:     HashMap::new(),
      initial:        beam::make_empty_code(),      // fill this
      current:        beam::Pointer::new_empty(),   // fill this
      parent:         parent.get_pid(),
      approx_started: 0,      // fill this
    }
  }

  // Begins execution from given m:f, with args a
  pub fn jump(&mut self, m: &term::Eterm, f: &term::Eterm, a: &term::Eterm)
  {
  }
}
