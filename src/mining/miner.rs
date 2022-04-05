use std::{io::Result, thread, time::Duration};

use crate::logln;

/// Start mining.
/// # REWRITE
pub fn start_mining() -> Result<()> {
  loop {
    thread::sleep(Duration::from_secs(5));
    logln!("\tMining...");
  }
}
