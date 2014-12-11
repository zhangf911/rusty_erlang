use std::rc::{Rc};
use {world, term};
use beam::raw_beam::RawBeam;

pub fn load_preloaded(state: &mut world::Erts) {
  let mods = vec!["otp_ring0", "init", "prim_eval", "prim_inet", "prim_file",
                  "zlib", "prim_zip", "erl_prim_loader", "erlang",
                  "erts_internal"];
  for mod_name in mods.iter() {
    let str_mod_name    = (*mod_name).to_string();
    let mod_atom: Rc<_> = state.atoms.put(&str_mod_name);
    let raw_beam        = RawBeam::load(&"preload".to_string(), &str_mod_name);
  }
}

struct LoaderState {
  /*
   * The current logical file within the binary.
   */

  file_name: String, // Name of file we are reading (usually chunk name)
  //byte *file_p;   // Current pointer within file
  //unsigned file_left;   // Number of bytes left in file
  //ErlDrvBinary *bin;    // Binary holding BEAM file (or NULL)

  /*
   * The following are used mainly for diagnostics.
   */

  //Eterm group_leader;   // Group leader (for diagnostics)
  //Eterm module;     // Tagged atom for module name
  //Eterm function;   // Tagged atom for current function * (or 0 if none)
  //unsigned arity;   // Arity for current function

  /*
   * All found chunks.
   */

  //struct {
  //  byte *start;    // Start of chunk (in binary)
  //  unsigned size;    // Size of chunk
  //} chunks[NUM_CHUNK_TYPES];
  chunks: Vec<Vec<u8>>,

  /*
   * Used for code loading (mainly).
   */

  //byte *code_start;   // Start of code file
  //unsigned code_size;   // Size of code file
  //int specific_op;    // Specific opcode (-1 if not found)
  //int num_functions;    // Number of functions in module
  //int num_labels;   // Number of labels
  //int code_buffer_size; // Size of code buffer in words.
  //BeamInstr *code;    // Loaded code
  //int ci;     // Current index into loaded code
  //Label *labels;
  //StringPatch *string_patches; // Linked list of position into string table to patch
  //BeamInstr catches;    // Linked list of catch_yf instructions
  //unsigned loaded_size; // Final size of code when loaded
  //byte mod_md5[16];   // MD5 for module code
  may_load_nif: bool, // true if NIFs may later be loaded for this module

  //int on_load;    // Index in the code for the on_load function
         // (or 0 if there is no on_load function)

  /*
   * Atom table.
   */

  //int num_atoms;    // Number of atoms in atom table
  //Eterm *atom;    // Atom table
  atoms: Vec<Rc<term::Eterm>>,

  //int num_exps;   // Number of exports
  //ExportEntry *export;  // Pointer to export table

  //int num_imports;    // Number of imports
  //ImportEntry *import;  // Import entry (translated information)

  /*
   * Generic instructions.
   */
  //GenOp *genop;   // The last generic instruction seen
  //GenOp *free_genop;    // List of free genops
  //GenOpBlock *genop_blocks; // List of all block of allocated genops

  /*
   * Lambda table.
   */

  //int num_lambdas;    // Number of lambdas in table
  //int lambdas_allocated;  // Size of allocated lambda table
  //Lambda *lambdas;    // Pointer to lambdas
  //Lambda def_lambdas[16]; // Default storage for lambda table
  //char *lambda_error;   // Delayed missing 'FunT' error

  /*
   * Literals (constant pool).
   */

  //int num_literals;   // Number of literals in table
  //int allocated_literals; // Number of literal entries allocated
  //Literal *literals;    // Array of literals
  //LiteralPatch *literal_patches; // Operands that need to be patched
  //Uint total_literal_size;  // Total heap size for all literals

  /*
   * Line table.
   */
  //BeamInstr *line_item; // Line items from the BEAM file
  //int num_line_items;   // Number of line items
  //LineInstr *line_instr;  // Line instructions
  //int num_line_instrs;  // Maximum number of line instructions
  //int current_li;   // Current line instruction
  //int *func_line;   // Mapping from function to first line instr
  //Eterm *fname;   // List of file names
  //int num_fnames;   // Number of filenames in fname table
  //int loc_size;   // Size of location info in bytes (2/4)
}
