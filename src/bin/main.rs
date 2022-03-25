use std::{io::Result, process, thread};

use rbtc::{messaging::thread::start_messaging, mining::thread::start_mining};

/// Start and join the messaging and mining threads.
fn start_threads(node_number: u8) {
  // Start threads.
  let messaging_thread = thread::spawn(move || start_messaging(&node_number));
  let mining_thread = thread::spawn(move || start_mining(&node_number));

  // Join threads.
  match messaging_thread.join() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("ERROR:\tcould not join messaging thread, {:?}", error)
    },
  }
  match mining_thread.join() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("ERROR:\tcould not join mining thread, {:?}", error)
    },
  }
}

/// Parse error-checked arguments and spawn node threads.
fn main() -> Result<()> {
  // Collect arguments into string vector.
  let args: Vec<String> = std::env::args().collect();

  // Check for expected number of arguments.
  if args.len() != 2 {
    eprintln!(
      "WARN:\texpected one argument after binary path, found {}",
      args.len() - 1
    );
  }

  // Parse argument to node number if possible and start execution threads.
  // Otherwise if argument cannot be parsed to u8, exit with error.
  match args[1].parse::<u8>() {
    Ok(node_number) => {
      start_threads(node_number);
    },
    Err(error) => {
      eprintln!("ERROR:\tcould not parse input to u8, {}", error);
      process::exit(1);
    },
  }
  Ok(())
}
