use std::{env::args, io::Result, net::Ipv4Addr, thread::spawn};

use rbtc::{
  inbound::start_inbound,
  log,
  mining::start_mining,
  outbound::start_outbound,
  util::{join_thread, parse_args_to_local_ip_addr},
};

/// Start and join all execution threads.
fn start_and_join_threads(local_ip_addr: Ipv4Addr) -> Result<()> {
  log!(
    "\tStarting and joining threads with local IP address {}",
    local_ip_addr
  );
  // Start threads.
  let thread_inbound = spawn(move || start_inbound(&local_ip_addr));
  let thread_mining = spawn(move || start_mining(&local_ip_addr));
  let thread_outbound = spawn(move || start_outbound(&local_ip_addr));

  // Join threads.
  join_thread(thread_inbound)?;
  join_thread(thread_mining)?;
  join_thread(thread_outbound)?;
  Ok(())
}

/// Parse error-checked arguments and spawn node threads.
fn main() -> Result<()> {
  // Collect arguments into string vector.
  let args: Vec<String> = args().collect();

  // Parse the arguments for the desired local IP address.
  let local_ip_addr = parse_args_to_local_ip_addr(args);

  // Start and join all execution threads using the local IP address.
  start_and_join_threads(local_ip_addr)?;

  Ok(())
}
