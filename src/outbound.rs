use std::{
  io::{stdin, Read, Result, Write},
  net::{Ipv4Addr, SocketAddrV4, TcpStream},
  process, thread,
  time::Duration,
};

use rand::{prelude::SliceRandom, thread_rng, Rng};
use socket2::{Domain, Socket, Type};

use crate::{
  constants::{BOOTSTRAP_IP_ADDRS, MSG_SIZE, RBTC_PORT, RBTC_PORT_RANGE},
  log,
};

/// Start sending messages on TCP streams.
pub fn start_outbound(local_ip_addr: &Ipv4Addr) -> Result<()> {
  // Initialize empty vector of streams.
  let mut streams: Vec<TcpStream> = Vec::new();

  // Keep trying to push streams from the bootstrap node IP addresses until at
  // least one succeeds.
  loop {
    // DEBUG: Limit the search rate.
    thread::sleep(Duration::from_secs(1));

    // Choose a random bootstrap node from the bootstrap node IP address list.
    let bootstrap_ip_addr = BOOTSTRAP_IP_ADDRS
      .choose(&mut thread_rng())
      .unwrap_or_else(|| {
        log!("Failed to choose random bootstrap node for initial stream");
        process::exit(1);
      });

    // Skip if this node finds its own address while iterating through the
    // bootstrap node address list.
    if bootstrap_ip_addr == local_ip_addr {
      log!(
        "\tSkipping own address in bootstrap node address list: {}",
        local_ip_addr
      );
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
    match socket.bind(&local_socket_addr.into()) {
      Ok(_) => {
        log!("\tBound to local address {}", local_socket_addr);
      },
      Err(err) => {
        log!("\tFailed to bind to local address: {}", err);
        continue;
      },
    };
    match socket.connect(&peer_socket_addr.into()) {
      Ok(_) => {
        log!("\tConnected to peer address {}", peer_socket_addr);
      },
      Err(err) => {
        log!(
          "\tFailed to connect to peer address {}: {}",
          peer_socket_addr,
          err
        );
        continue;
      },
    };

    log!(
      "\tConnected stream from {} to {}...",
      socket.local_addr()?.as_socket_ipv4().unwrap_or_else(|| {
        log!("\tFailed to unwrap local socket address");
        process::exit(1);
      }),
      socket.peer_addr()?.as_socket_ipv4().unwrap_or_else(|| {
        log!("\tFailed to unwrap peer socket address");
        process::exit(1);
      }),
    );

    // Convert the socket to a TcpStream and push it to the `streams` vector.
    streams.push(socket.into());
    break;
  }

  // DEBUG: At this point, the node has connected to one bootstrap node.

  // Continuously get inputs from stdin and send them on each stream.
  loop {
    // Try reading input from stdin. If this fails, restart the input loop.
    let mut buf = [0u8; MSG_SIZE];
    match stdin().read_exact(&mut buf) {
      Ok(_) => match streams[0].write_all(&buf) {
        Ok(_) => match streams[0].flush() {
          Ok(_) => log!("Wrote and flushed {:?} to stream", buf),
          Err(err) => log!("Failed to flush stream, {}", err),
        },
        Err(err) => log!("Failed to write to stream, {}", err),
      },
      Err(err) => log!("Failed to read input, {}", err),
    };
  }
}
