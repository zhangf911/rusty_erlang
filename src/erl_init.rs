use erl_alloc;

static EMULATOR_TYPE: &'static str = "Rusty BEAM";
static ERLANG_VERSION: &'static str = "18.0";
static ERTS_DEFAULT_MAX_PROCESSES: uint = 1 << 18;
static ERTS_DEFAULT_MAX_PORTS: uint = 1 << 18;

// Core configuration and global variables for erts
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

pub fn start(args: &Vec<String>) -> Result<ErlInit, ()> {
  erl_alloc::init(args);

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

  return erl_init(init);
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

fn erl_init(init: ErlInit) -> Result<ErlInit, ()> {
  return Ok(init);
}