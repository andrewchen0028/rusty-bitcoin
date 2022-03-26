/// The TCP port to be used by the Rusty Bitcoin network.
pub const RBTC_PORT: u16 = 42069;

/// The number of bytes that make up a Rusty Bitcoin message.
pub const MSG_SIZE: usize = 1;

/// The number of bootstrap nodes in the Rusty Bitcoin network.
/// NOTE: BOOTSTRAP_NODE_COUNT should really be a `u8` for IPv4 addresses, but
/// it needs to be a `usize` to work as an array length.
pub const BOOTSTRAP_NODE_COUNT: usize = 2;

/// The list of boostrap node addresses in the Rusty Bitcoin network.
/// NOTE: BOOTSTRAP_NODE_COUNT should really be a `u8` for IPv4 addresses, but
/// it needs to be a `usize` to work as an array length.
pub const BOOTSTRAP_NODE_NUMBERS: [u8; BOOTSTRAP_NODE_COUNT] = [2, 3];
