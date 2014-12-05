use std::collections::{HashMap};
use types::{Uint, Sint, Eterm, ApproxTime, Pid};
use beam;
use world;

pub type ProcDict = HashMap<String, Eterm>;

#[allow(dead_code)]
pub struct MessageQueue {
  //ErlMessage *first;
  //ErlMessage **last;  /* point to the last next pointer */
  //ErlMessage **save;
  len: Sint,  // queue length

  /*
   * The following two fields are used by the recv_mark/1 and
   * recv_set/1 instructions.
   */
  //BeamInstr *mark;    /* address to rec_loop/2 instruction */
  //ErlMessage **saved_last;  /* saved last pointer */
}

#[allow(dead_code)]
pub struct Process {
  heap_top:   Eterm,
  stack_top:  Eterm,
  heap_start: Eterm,
  heap_end:   Eterm,
  heap_sz:    Uint,     // heap size in words
  min_heap_size: Uint,  // Minimum size of heap (in words)
  min_vheap_size: Uint, // Minimum size of virtual heap (in words)

  //
  // Saved x registers.
  //
  //arity: Uint,     // Number of live argument registers (only valid
                   // when process is *not* running).
  //Eterm *arg_reg;   /* Pointer to argument registers. */
  //unsigned max_arg_reg; /* Maximum number of argument registers available. */
  //Eterm def_arg_reg[6]; /* Default array for argument registers. */

  cp: beam::Pointer,   // (untagged) Continuation pointer (for threaded code)
  i:  beam::Pointer,   // Program counter for threaded code
  //Sint catches;   // Number of catches on stack
  //Sint fcalls;    // Number of reductions left to execute
                    // Only valid for the current process

  //Uint32 rcount;    /* suspend count */
  //int  schedule_count;  /* Times left to reschedule a low prio process */
  //Uint reds;      /* No of reductions for this process  */
  //Eterm group_leader;   /* Pid in charge (can be boxed) */
  //Uint flags;     /* Trap exit, etc (no trace flags anymore) */
  //Eterm fvalue;   /* Exit & Throw value (failure reason) */
  //Uint freason;   /* Reason for detected failure */
  //Eterm ftrace;   /* Latest exception stack trace dump */

  //Process *next;    /* Pointer to next process in run queue */

  //struct ErtsNodesMonitor_ *nodes_monitors;

  //ErtsSuspendMonitor *suspend_monitors; // Processes suspended by this process
                                          // via erlang:suspend_process/1

  msg: MessageQueue,

  //union {
  //  ErtsBifTimer *bif_timers; /* Bif timers aiming at this process */
  //  void *terminate;
  //} u;

  dictionary: ProcDict,      // Process dictionary, may be NULL

  //Uint seq_trace_clock;
  //Uint seq_trace_lastcnt;
  //Eterm seq_trace_token;  /* Sequential trace token (tuple size 5 see below) */

//#ifdef USE_VM_PROBES
//  Eterm dt_utag;              /* Place to store the dynamc trace user tag */
//  Uint dt_utag_flags;         /* flag field for the dt_utag */
//#endif
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
  parent: Eterm,   // Pid of process that created this process
  approx_started: ApproxTime, // Time when started

  /* This is the place, where all fields that differs between memory
   * architectures, have gone to.
   */

  //Eterm *high_water;
  //Eterm *old_hend;            /* Heap pointers for generational GC. */
  //Eterm *old_htop;
  //Eterm *old_heap;
  //Uint16 gen_gcs;   /* Number of (minor) generational GCs. */
  //Uint16 max_gen_gcs;   /* Max minor gen GCs before fullsweep. */
  //ErlOffHeap off_heap;  /* Off-heap data updated by copy_struct(). */
  //ErlHeapFragment *mbuf;  /* Pointer to message buffer list */
  //Uint mbuf_sz;   /* Size of all message buffers */
  //ErtsPSD *psd;   /* Rarely used process specific data */

  //Uint64 bin_vheap_sz;  /* Virtual heap block size for binaries */
  //Uint64 bin_vheap_mature;  /* Virtual heap block size for binaries */
  //Uint64 bin_old_vheap_sz;  /* Virtual old heap block size for binaries */
  //Uint64 bin_old_vheap; /* Virtual old heap size for binaries */

  //ErtsProcSysTaskQs *sys_task_qs;

  //erts_smp_atomic32_t state;  /* Process state flags (see ERTS_PSFLG_*) */

/*#ifdef ERTS_SMP
  ErlMessageInQueue msg_inq;
  ErtsPendExit pending_exit;
  erts_proc_lock_t lock;
  ErtsSchedulerData *scheduler_data;
  Eterm suspendee;
  ErtsPendingSuspend *pending_suspenders;
  erts_smp_atomic_t run_queue;
#ifdef HIPE
  struct hipe_process_state_smp hipe_smp;
#endif
#endif*/

//#ifdef CHECK_FOR_HOLES
//  Eterm *last_htop;   /* No need to scan the heap below this point. */
//  ErlHeapFragment *last_mbuf; /* No need to scan beyond this mbuf. */
//#endif

//#ifdef DEBUG
//  Eterm *last_old_htop;
         // No need to scan the old heap below this point
         // when looking for invalid pointers into the new heap or
         // heap fragments.
//#endif

//#ifdef FORCE_HEAP_FRAGS
//  Uint space_verified;        /* Avoid HAlloc forcing heap fragments when */
//  Eterm *space_verified_from; /* we rely on available heap space (TestHeap) */
//#endif
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

pub fn first_process_otp(state: &mut world::Erts,
                     mod_name: String,
                     code: Option<beam::Code>)
                     -> Result<(), String>
{
  let start_mod = state.atoms.put(&mod_name);
  match state.find_exported_fun(start_mod, state.atoms.am_start, 2,
                                state.code_ix.get_active()) {
    Err(())    => return Err("No function ".to_string() + mod_name + ":start/2"),
    Ok(export) => return Ok(())
  }
}
