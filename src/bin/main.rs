use std::{io::Result, panic, thread::spawn};

use rbtc::{mining::thread::start_mining, ui::thread::start_ui};

/// Parse error-checked arguments and spawn node threads.
/// TODO: Rewrite or heavily scrutinize all files marked with "REWRITE".
/// # REWRITE
fn main() -> Result<()> {
  // Start execution threads.
  let mining_thread = spawn(start_mining);

  let ui_thread = spawn(start_ui);

  // TODO: Figure out wtf this "panic::resume_unwind()" thing does.
  // TODO: Make threads not compete for stdout space.
  match mining_thread.join() {
    Ok(_) => println!("Exited mining thread"),
    Err(err) => panic::resume_unwind(err),
  }
  match ui_thread.join() {
    Ok(_) => println!("Exited UI thread"),
    Err(err) => panic::resume_unwind(err),
  }
  Ok(())
}
