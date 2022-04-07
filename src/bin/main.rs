use std::{io::Result, panic, thread::spawn};

use async_std::task::block_on;
use rbtc::{
  logln,
  mining::thread::start_mining,
  networking::thread::start_networking,
  ui::{cli::Commands, thread::start_ui},
};

/// Parse error-checked arguments and spawn node threads.
/// TODO: Rewrite or heavily scrutinize all files marked with "REWRITE".
fn main() -> Result<()> {
  // Initialize inter-thread communication channels.
  let (s_commands, r_commands) = async_std::channel::unbounded::<Commands>();
  logln!("Initialized channels");

  // Start execution threads.
  let mining_thread = spawn(start_mining);
  logln!("Spawned mining thread");
  let networking_thread = spawn(|| block_on(start_networking(r_commands)));
  logln!("Spawned networking thread");
  let ui_thread = spawn(|| block_on(start_ui(s_commands)));
  logln!("Spawned UI thread");

  // TODO: Figure out wtf this "panic::resume_unwind()" thing does.
  // TODO: Make threads not compete for stdout space.
  match mining_thread.join() {
    Ok(_) => println!("Exited mining thread"),
    Err(err) => panic::resume_unwind(err),
  }
  match networking_thread.join() {
    Ok(_) => println!("Exited networking thread"),
    Err(err) => panic::resume_unwind(err),
  }
  match ui_thread.join() {
    Ok(_) => println!("Exited UI thread"),
    Err(err) => panic::resume_unwind(err),
  }
  Ok(())
}
