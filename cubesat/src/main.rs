#![allow(unused_variables)]

struct GroundStation;
impl GroundStation {
  fn send(&self, mailbox: &mut MailBox, to: &CubeSat, msg: Message) {
    mailbox.post(msg);
  }
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat {
      id: sat_id,
      mailbox: MailBox { messages: vec![] },
    }
  }
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

#[derive(Debug)]
struct Message {
  to: u64,
  content: String,
}

#[derive(Debug)]
struct MailBox {
  messages: Vec<Message>,
}
impl MailBox {
  fn post(&mut self, msg: Message) {
    self.messages.push(msg);
  }

  fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
    for i in 0..self.messages.len() {
      if self.messages[i].to == recipient.id {
        let msg = self.messages.remove(i);
        return Some(msg);
      }
    }
    None
  }
}

#[derive(Debug)]
struct CubeSat {
  id: u64,
}
impl CubeSat {
  fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
    mailbox.deliver(&self)
  }
}

fn check_status(sat_id: CubeSat) -> CubeSat {
  println!("{}: {:?}", sat_id.id, StatusMessage::Ok);
  sat_id
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1, 2, 3]
}

fn main() {
  let base = GroundStation {};

  let sat_ids = fetch_sat_ids();

  for sat_id in sat_ids {
    // satの生存期間が短くなった
    let mut sat = base.connect(sat_id);
    base.send(&mut sat, Message::from("hello"));
  }

  // let mut sat_a = CubeSat {
  //   id: 0,
  //   mailbox: MailBox { messages: vec![] },
  // };
  // let mut sat_b = CubeSat {
  //   id: 1,
  //   mailbox: MailBox { messages: vec![] },
  // };
  // let mut sat_c = CubeSat {
  //   id: 2,
  //   mailbox: MailBox { messages: vec![] },
  // };

  // println!("t0: {:?}", sat_a);

  // base.send(&mut sat_a, Message::from("hello there!"));

  // println!("t1: {:?}", sat_a);

  // let message = sat_a.recv();

  // println!("t2: {:?}", sat_a);
  // println!("msg: {:?}", message);
}
