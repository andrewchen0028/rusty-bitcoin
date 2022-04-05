use std::{
  io::{stdin, Result, Write},
  net::{Ipv4Addr, SocketAddrV4, TcpStream},
  process, thread,
  time::Duration,
};

use rand::{prelude::SliceRandom, thread_rng, Rng};
use socket2::{Domain, Socket, Type};

use crate::{
  log,
  util::constants::{BOOTSTRAP_IP_ADDRS, MSG_SIZE, RBTC_PORT, RBTC_PORT_RANGE},
};

/// Start sending messages on TCP streams.
/// # REWRITE
pub fn start_outbound(local_ip_addr: &Ipv4Addr) -> Result<()> {
  // Initialize empty vector to hold streams.
  let mut streams: Vec<TcpStream> = Vec::new();

  // Keep trying to push streams from the bootstrap node IP addresses until at
  // least one succeeds.
  // TODO: This hangs indefinitely when trying to connect to a bootstrap node
  // that doesn't yet exist. a) Make it time out, b) make it try several
  // connections concurrently.
  loop {
    // DEBUG: Limit the search rate.
    thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
    log!("Entered search loop");

    // Choose a random bootstrap node from the bootstrap node IP address list.
    let bootstrap_ip_addr = BOOTSTRAP_IP_ADDRS
      .choose(&mut thread_rng())
      .unwrap_or_else(|| {
        log!("\tFailed to choose random bootstrap node for initial stream");
        process::exit(1);
      });
    log!("Chose bootstrap IP address {}", bootstrap_ip_addr);

    // Skip if this node finds its own address while iterating through the
    // bootstrap node address list.
    if bootstrap_ip_addr == local_ip_addr {
      log!("Skipping own IP address");
      continue;
    }

    // Initialize local and peer socket addresses.
    let local_socket_addr = SocketAddrV4::new(
      *local_ip_addr,
      thread_rng().gen_range(RBTC_PORT_RANGE),
    );
    let peer_socket_addr = SocketAddrV4::new(*bootstrap_ip_addr, RBTC_PORT);

    // Initialize an empty socket, bind it to the local socket address, and
    // connect it to the peer socket address.
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.bind(&local_socket_addr.into())?;
    socket.connect_timeout(&peer_socket_addr.into(), Duration::from_secs(1))?;

    log!(
      "\tConnected from {} to {}",
      socket.local_addr()?.as_socket_ipv4().unwrap(),
      socket.peer_addr()?.as_socket_ipv4().unwrap(),
    );

    // Convert the socket to a TcpStream and push it to the `streams` vector.
    streams.push(socket.into());
    break;
  }

  // DEBUG: At this point, the node has connected to one bootstrap node.

  // Continuously get inputs from stdin and send them on each stream.
  loop {
    // Try reading input from stdin. If this fails, restart the input loop.
    let mut buf_string = String::new();
    log!("\tEntered input loop");
    match stdin().read_line(&mut buf_string) {
      Ok(_) => {
        let buf_bytes = buf_string.trim_end().as_bytes();
        if buf_bytes.len() != MSG_SIZE {
          log!("\tExpected {} bytes, got {}", MSG_SIZE, buf_bytes.len());
          continue;
        }
        match streams[0].write_all(buf_bytes) {
          Ok(_) => match streams[0].flush() {
            Ok(_) => log!("\tRead input, wrote stream, and flushed"),
            Err(err) => log!(
              "\tRead input and wrote to stream, but failed to flush, {}",
              err
            ),
          },
          Err(err) => {
            log!("\tRead input but failed to write to stream, {}", err)
          },
        }
      },
      Err(err) => log!("\tFailed to read input to buffer, {}", err),
    };
  }
}
