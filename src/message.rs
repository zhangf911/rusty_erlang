use term;

#[allow(dead_code)]
pub struct Queue {
  items:  Vec<term::EtermValue>,
  //ErlMessage *first;
  //ErlMessage **last;  /* point to the last next pointer */
  //ErlMessage **save;
  //len: Sint,  // queue length

  /*
   * The following two fields are used by the recv_mark/1 and
   * recv_set/1 instructions.
   */
  //BeamInstr *mark;    /* address to rec_loop/2 instruction */
  //ErlMessage **saved_last;  /* saved last pointer */
}

impl Queue {
  pub fn new() -> Queue {
    Queue{
      items: Vec::new()
    }
  }
}
