use std::io::Result;

use crate::util::networking::addr_from_node_number;

/// Start sending messages on TCP streams.
pub fn start_client_thread(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  println!(
    "Constructed addr from node number in client thread:\t{}",
    addr
  );
  Ok(())
}
