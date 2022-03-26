use std::{io::Result, thread, time::Duration};

use crate::util::networking::addr_from_node_number;

/// Start mining.
pub fn start_miner_thread(node_number: &u8) -> Result<()> {
  // Get local address from provided node number.
  let addr = addr_from_node_number(*node_number);
  loop {
    thread::sleep(Duration::from_secs(5));
    println!("Mining on address {}...", &addr);
  }
}
