use std::{
  thread::{sleep, Result},
  time::Duration,
};

/// Start mining.
pub fn start_mining() -> Result<()> {
  loop {
    sleep(Duration::from_secs(5));
  }
}
