use async_std::channel::Receiver;

use crate::{logln, networking::error::Error, ui::cli::Commands};

/// Start thread with the following responsibilities:
/// - Handle:
///   - commands from UI thread
///   - updates from miner thread
///   - inbound TCP messages from other nodes
/// - Send:
///   - updates to UI thread
///   - updates to miner thread
///   - outbound TCP messages as necessary
///
/// TODO: This thread receives Commands from the UI thread now. Make this thread
/// actually do stuff.
pub async fn start_networking(
  r_commands: Receiver<Commands>,
) -> Result<(), Error> {
  // Number of outbound connections: 8
  // Number of inbound connections: 125 - 8
  loop {
    logln!("Entered networking loop");
    let command = r_commands.recv().await?;
    logln!("Received command: {:?}", command);
  }
}
