use std::io::Result;

use super::{listener::start_listener, streamer::start_streamer};
use crate::util::networking::addr_from_node_number;

/// Start messaging thread and create listener and streamers.
pub fn start_messaging(node_number: &u8) -> Result<()> {
  let addr = addr_from_node_number(*node_number);
  println!(
    "Constructed addr from node number in messaging thread:\t{}",
    addr
  );

  start_listener(addr)?;
  start_streamer(addr)?;
  Ok(())
}
