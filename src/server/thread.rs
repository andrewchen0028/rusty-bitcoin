use std::io::Result;

use crate::util::networking::addr_from_node_number;

/// Start listening for messages on TCP listener.
pub fn start_server_thread(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  println!(
    "Constructed addr from node number in server thread:\t{}",
    addr
  );
  Ok(())
}
