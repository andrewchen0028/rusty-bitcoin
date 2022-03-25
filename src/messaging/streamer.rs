use std::{io::Result, net::SocketAddrV4};

/// Starts sending streams from the given address.
pub fn start_streamer(addr: SocketAddrV4) -> Result<()> {
  println!("Received addr in streamer:\t{}", addr);
  Ok(())
}
