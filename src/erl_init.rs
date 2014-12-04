use alloc;
use world;

static EMULATOR_TYPE: &'static str = "Rusty BEAM";
static ERLANG_VERSION: &'static str = "18.0";
static ERTS_DEFAULT_MAX_PROCESSES: uint = 1 << 18;
static ERTS_DEFAULT_MAX_PORTS: uint = 1 << 18;

// Core configuration and global variables for erts
#[allow(dead_code)]
pub struct ErlInit {
  erts_no_crash_dump: bool,
  erts_init_module: String,
  erts_init_fun: String,

  ncpu: uint,
  proc_tab_sz: uint,
  port_tab_sz: uint,
  port_tab_sz_ignore_files: uint,
}

impl ErlInit {
  pub fn new() -> ErlInit {
	  ErlInit {
	    erts_no_crash_dump: false,
	    erts_init_module: "init".to_string(),
	    erts_init_fun: "boot".to_string(),
	    ncpu: 1,
	    proc_tab_sz: ERTS_DEFAULT_MAX_PROCESSES,
	    port_tab_sz: ERTS_DEFAULT_MAX_PORTS,
	    port_tab_sz_ignore_files: 0
	  }
  }
}

pub fn start(args: &Vec<String>) -> Result<world::State, ()>
{
  let mut i: uint = 1;
  let mut init = ErlInit::new();

  while i < args.len() {
    match args[i].as_slice() {
    "-V" => {
            print_version();
            return Err(());
            },

    //"-h" => // set default heap size

    // never produce crash dumps
    "-d" => init.erts_no_crash_dump = true,

    // define name of module for initial function
    "-i" => match get_arg(args, &mut i) {
              Ok(v) => init.erts_init_module = v,
              Err(()) => return Err(())
            },
    // define name of initial function
    "-b" => match get_arg(args, &mut i) {
              Ok(v) => init.erts_init_fun = v,
              Err(()) => return Err(())
            },
    _    => {
            print_unknown_flag(&args[0], &args[i]);
            return Err(());
            },
    }
    i += 1;
  }

  let mut state: world::State = world::State::new(init);
  alloc::init(args, &mut state);
  erl_init(&mut state);
  return Ok(state);
}

fn print_version() {
  println!("Erlang ({em_type}) emulator version {erl_ver}",
           em_type = EMULATOR_TYPE,
           erl_ver = ERLANG_VERSION);
}

fn print_unknown_flag(app: &String, f: &String) {
  println!("{app} unknown flag {flag}", app=app, flag=f);
  print_usage();
}

fn print_usage() {
}

fn get_arg(args: &Vec<String>, i: &mut uint) -> Result<String, ()> {
  if *i < args.len() - 1 {
    *i += 1;
    return Ok(args[*i].clone());
  }
  return Err(());
}

fn erl_init(_state: &mut world::State) {
  //init_benchmarking();
  //erts_init_monitors();
  //erts_init_time();
  //erts_init_sys_common_misc();
  //erts_init_process(ncpu, proc_tab_sz, legacy_proc_tab);
  /*erts_init_scheduling(no_schedulers,
                       no_schedulers_online
#ifdef ERTS_DIRTY_SCHEDULERS
                       , no_dirty_cpu_schedulers,
                       no_dirty_cpu_schedulers_online,
                       no_dirty_io_schedulers
#endif
                      );*/
  //erts_init_cpu_topology(); /* Must be after init_scheduling */
  //erts_init_gc(); /* Must be after init_scheduling */
  //erts_alloc_late_init();

  //H_MIN_SIZE      = erts_next_heap_size(H_MIN_SIZE, 0);
  //BIN_VH_MIN_SIZE = erts_next_heap_size(BIN_VH_MIN_SIZE, 0);

  //erts_init_trace();
  //erts_init_bits();
  //erts_code_ix_init();
  //erts_init_fun_table();      ++
  //init_atom_table(state);     ++
  //init_export_table();        --
  //init_module_table();        ++
  //init_register_table();
  //init_message();
  //erts_bif_info_init();
  //erts_ddll_init();
  //init_emulator();
  //erts_ptab_init(); /* Must be after init_emulator() */
  //erts_init_binary(); /* Must be after init_emulator() */
  //erts_bp_init();
  //init_db(); /* Must be after init_emulator */
  //erts_bif_timer_init();
  //erts_init_node_tables();
  //init_dist();
  //erl_drv_thr_init();
  //erts_init_async();
  //erts_init_io(port_tab_sz, port_tab_sz_ignore_files, legacy_port_tab);
  //init_load();
  //erts_init_bif();
  //erts_init_bif_chksum();
  //erts_init_bif_binary();
  //erts_init_bif_re();
  //erts_init_unicode(); /* after RE to get access to PCRE unicode */
  //erts_init_external();
  //erts_delay_trap = erts_export_put(am_erlang, am_delay_trap, 2);
  //erts_late_init_process();
/*#if HAVE_ERTS_MSEG
  erts_mseg_late_init(); // Must be after timer (erts_init_time()) and thread
            //initializations
#endif*/
/*#ifdef HIPE
  hipe_mode_switch_init(); // Must be after init_load/beam_catches/init
#endif*/
  //packet_parser_init();
  //erl_nif_init();
}
