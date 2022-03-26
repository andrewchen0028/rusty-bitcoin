use std::{io::Result, process, thread};

use rbtc::{
  client::thread::start_client_thread, miner::thread::start_miner_thread,
  server::thread::start_server_thread,
};

/// Start and join all execution threads.
fn start_threads(node_number: u8) {
  // Start threads.
  let client_thread = thread::spawn(move || start_client_thread(&node_number));
  let server_thread = thread::spawn(move || start_server_thread(&node_number));
  let miner_thread = thread::spawn(move || start_miner_thread(&node_number));

  // Join threads.
  match client_thread.join() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("ERROR:\tcould not join client thread, {:?}", error)
    },
  }
  match server_thread.join() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("ERROR:\tcould not join server thread, {:?}", error)
    },
  }
  match miner_thread.join() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("ERROR:\tcould not join miner thread, {:?}", error)
    },
  }
}

/// Parse error-checked arguments and spawn node threads.
fn main() -> Result<()> {
  // Collect arguments into string vector.
  let args: Vec<String> = std::env::args().collect();

  // Check for expected number of arguments after `main` (one). Parse argument
  // to node number (u8) if possible and start execution threads. If argument
  // cannot be parsed to u8, exit with error.
  match args.len() - 1 {
    1 => match args[1].parse::<u8>() {
      Ok(node_number) => {
        start_threads(node_number);
      },
      Err(error) => {
        eprintln!("ERROR:\tcould not parse input to u8, {}", error);
        process::exit(1);
      },
    },
    num => {
      eprintln!("ERROR:\texpected one argument after `main`, found {}", num);
      process::exit(1);
    },
  }
  Ok(())
}
