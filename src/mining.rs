use std::{io::Result, net::Ipv4Addr, thread, time::Duration};

use crate::log;

/// Start mining.
pub fn start_mining(local_ip_addr: &Ipv4Addr) -> Result<()> {
  loop {
    thread::sleep(Duration::from_secs(5));
    // log!("\tMining on node {}...", local_ip_addr);
  }
}
