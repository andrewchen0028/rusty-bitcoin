use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

/// A message that can be serialized and sent between RBTC nodes.
/// # REWRITE
#[derive(Debug, Deserialize, Serialize)]
pub enum Msg {
  /// Sent to confirm that a TCP/IP connection is still valid.
  Ping,

  /// Sent in response to a Ping message.
  Pong,

  /// Provides a list of known node IP addresses on the network.
  Addr(Vec<Ipv4Addr>),
}
