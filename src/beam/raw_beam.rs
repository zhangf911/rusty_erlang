//use std::str::from_utf8_owned;
use std::io::File;

pub struct RawBeam {
  bytes: Vec<u8>,
}

impl RawBeam {
  pub fn load(dir: &String, mod_name: &String) -> RawBeam {
    let path = Path::new(*dir + "/" + mod_name + ".beam");
    let mut hw_file = File::open(&path);
    match hw_file.read_to_end() {
      Ok(bytes_read) => RawBeam {
                    bytes: bytes_read
                  },
      Err(ioerr) => {
          println!("raw beam load error: {}", ioerr);
          panic!("raw beam load error")
        }
    }
  }
}
