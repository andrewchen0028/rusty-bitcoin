use std::{io::Result, thread, time::Duration};

use crate::util::networking::addr_from_node_number;

/// Start mining.
pub fn start_miner_thread(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  loop {
    println!("Mining on address {}...", &addr);
    thread::sleep(Duration::from_secs(1));
  }
}
