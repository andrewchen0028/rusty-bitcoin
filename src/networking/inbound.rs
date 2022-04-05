use std::{
  io::Read,
  net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use crate::{
  log,
  util::constants::{MSG_SIZE, RBTC_PORT},
  util::errors::Result,
};

/// Start listening for messages.
/// # REWRITE
pub fn start_inbound(local_ip_addr: &Ipv4Addr) -> Result<()> {
  // Get local socket address from provided local IP address.
  let local_socket_addr = SocketAddrV4::new(*local_ip_addr, RBTC_PORT);

  // Try binding a TCP listener to local address.
  let listener = TcpListener::bind(local_socket_addr)?;

  // Continuously handle incoming streams.
  for stream_result in listener.incoming() {
    // Log progress.
    log!("\tEntered output loop");

    // Extract stream and read to buffer.
    let mut stream = stream_result?;
    let mut buf = [0u8; MSG_SIZE];
    stream.read_exact(&mut buf)?;

    // Print buffer.
    log!(
      "\tNode {} received message {:?} from node {}",
      stream.local_addr()?.ip(),
      buf,
      stream.peer_addr()?.ip()
    );
  }
  log!("\tExited for loop, done listening for streams");

  Ok(())
}
