use std::{io::Result, thread, time::Duration};

use crate::util::networking::addr_from_node_number;

/// Start mining thread and begin mining.
pub fn start_mining(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  println!(
    "Constructed addr from node number in mining thread:\t{}",
    addr
  );
  loop {
    thread::sleep(Duration::from_secs(1));
    println!("Mining...");
  }
}
