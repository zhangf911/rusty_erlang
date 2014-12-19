//use std::str::from_utf8_owned;
use std::io::File;

pub struct RawBeam {
  pub mod_name: String,
  bytes:        Vec<u8>,
}

impl RawBeam {
  pub fn load(dir: &String, mod_name: &String) -> RawBeam {
    let path = Path::new(format!("{}/{}.beam", dir, mod_name));
    let mut hw_file = File::open(&path);
    match hw_file.read_to_end() {
      Ok(bytes_read) => RawBeam {
                    mod_name: mod_name.clone(),
                    bytes:    bytes_read
                  },
      Err(ioerr) => {
          panic!(format!("raw beam load error: {}", ioerr));
        }
    }
  }
}
