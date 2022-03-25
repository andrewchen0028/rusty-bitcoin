use std::{io::Result, net::SocketAddrV4};

/// Starts listening for streams on the given address.
pub fn start_listener(addr: SocketAddrV4) -> Result<()> {
  println!("Received addr in listener:\t{}", addr);
  Ok(())
}
