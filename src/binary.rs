pub struct Binary {
  bytes: Box<Vec<u8>>,
}

pub fn new(&mut self, process: &Process, data: &Vec<u8>) -> Eterm {
  // TODO: Onheap bin limit
  return Gc<data.clone()>
}
