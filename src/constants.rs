use std::{net::Ipv4Addr, ops::RangeInclusive};

/// The designated inbound TCP port to be used by the Rusty Bitcoin network.
pub const RBTC_PORT: u16 = 42069;

/// The range of outbound TCP ports to be used by the Rusty Bitcoin network.
pub const RBTC_PORT_RANGE: RangeInclusive<u16> = 49152..=64738;

/// The number of bytes that make up a Rusty Bitcoin message.
pub const MSG_SIZE: usize = 4;

/// The number of bootstrap nodes in the Rusty Bitcoin network.
pub const BOOTSTRAP_COUNT: usize = 2;

/// The list of boostrap node IP addresses in the Rusty Bitcoin network.
pub const BOOTSTRAP_IP_ADDRS: [Ipv4Addr; BOOTSTRAP_COUNT] =
  [Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(127, 0, 0, 2)];

/// The number of seconds to wait before timing out of a TCP connection attempt.
pub const CONNECT_TIMEOUT_SECS: u64 = 10;
