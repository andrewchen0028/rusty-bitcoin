use std::{io::Result, net::Ipv4Addr, process, thread::JoinHandle};

use crate::log;

/// Check for expected number of arguments after `main` (one). Parse argument
/// to Ipv4Addr if possible and start execution threads. If argument cannot be
/// parsed to Ipv4Addr, exit with error.
pub fn parse_args_to_local_ip_addr(args: Vec<String>) -> Ipv4Addr {
  match args.len() - 1 {
    1 => args[1].parse::<Ipv4Addr>().unwrap_or_else(|err| {
      log!("\t{}", &err);
      process::exit(1)
    }),
    num => {
      log!("\tExpected one argument after `main`, found {}", num);
      process::exit(1);
    },
  }
}

/// Join a thread.
pub fn join_thread(join_handle: JoinHandle<Result<()>>) -> Result<()> {
  match join_handle.join() {
    Ok(_) => {},
    Err(err) => {
      log!("\tCouldn't join thread, {:?}", err);
    },
  };
  Ok(())
}
