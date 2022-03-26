use std::io::Result;

use crate::util::networking::addr_from_node_number;

/// Start mining.
pub fn start_miner_thread(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  println!(
    "Constructed addr from node number in miner thread:\t{}",
    addr
  );
  Ok(())
}
