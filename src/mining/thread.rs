use std::{
  thread::{sleep, Result},
  time::Duration,
};

use crate::logln;

/// Start mining.
/// # REWRITE
pub fn start_mining() -> Result<()> {
  loop {
    sleep(Duration::from_secs(5));
    logln!("\tMining...");
  }
}
