use crate::networking::error::Error;

/// Start thread with the following responsibilities:
/// - Handle:
///   - commands from UI thread
///   - updates from miner thread
///   - inbound TCP messages from other nodes
/// - Send:
///   - updates to UI thread
///   - updates to miner thread
///   - outbound TCP messages as necessary
pub fn start_networking() -> Result<(), Error> {
  // Number of outbound connections: 8
  // Number of inbound connections: 125 - 8

  Ok(())
}
