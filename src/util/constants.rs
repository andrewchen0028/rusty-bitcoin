use std::{net::Ipv4Addr, ops::RangeInclusive};

use crate::util::errors::{Error::InvalidNetworkIDByte, Result};

/// Conversion factors from underlying currency (nanoRBTC) to other units.
pub const NANO_FROM_NANO: f64 = 1e0;
pub const NANO_FROM_MICRO: f64 = 1e3;
pub const NANO_FROM_MILLI: f64 = 1e6;
pub const NANO_FROM_UNIT: f64 = 1e9;
pub const NANO_FROM_KILO: f64 = 1e12;
pub const NANO_FROM_MEGA: f64 = 1e15;
pub const NANO_FROM_GIGA: f64 = 1e18;

/// The designated inbound TCP port to be used by the Rusty Bitcoin network.
pub const RBTC_PORT: u16 = 42069;

/// The range of outbound TCP ports to be used by the Rusty Bitcoin network.
pub const RBTC_PORT_RANGE: RangeInclusive<u16> =
  RangeInclusive::new(49152, 64738);

/// The number of bytes that make up a Rusty Bitcoin message.
pub const MSG_SIZE: usize = 4;

/// The number of bootstrap nodes in the Rusty Bitcoin network.
pub const BOOTSTRAP_COUNT: usize = 2;

/// The list of boostrap node IP addresses in the Rusty Bitcoin network.
pub const BOOTSTRAP_IP_ADDRS: [Ipv4Addr; BOOTSTRAP_COUNT] =
  [Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(127, 0, 0, 2)];

/// The number of seconds to wait before timing out of a TCP connection attempt.
pub const CONNECT_TIMEOUT_SECS: u64 = 10;

/// The index of the network ID byte in an RBTC wallet.
pub const WALLET_ADDR_INDEX_NETWORK_ID: usize = 0;

/// The size of an RBTC wallet's underlying bytes.
pub const WALLET_ADDR_SIZE_UNDERLYING: usize = 20;

/// The size of an RBTC wallet checksum, in bytes.
pub const WALLET_ADDR_SIZE_CHECKSUM: usize = 4;

/// The size of an RBTC wallet address, in bytes.
pub const WALLET_ADDR_SIZE: usize =
  1 + WALLET_ADDR_SIZE_UNDERLYING + WALLET_ADDR_SIZE_CHECKSUM;

/// The indices of the underlying bytes in an RBTC wallet.
pub const WALLET_ADDR_RANGE_UNDERLYING: RangeInclusive<usize> =
  RangeInclusive::new(1, WALLET_ADDR_SIZE_UNDERLYING);

/// The indices of the checksum bytes in an RBTC wallet.
pub const WALLET_ADDR_RANGE_CHECKSUM: RangeInclusive<usize> =
  RangeInclusive::new(
    WALLET_ADDR_SIZE_UNDERLYING,
    WALLET_ADDR_SIZE_UNDERLYING + WALLET_ADDR_SIZE_CHECKSUM,
  );

/// The size of a RIPEMD-160 hash, in bytes.
pub const RIPEMD160_HASH_SIZE: usize = 20;

/// The size of a SHA-256 hash, in bytes.
pub const SHA256_HASH_SIZE: usize = 32;

/// A network ID.
pub enum NetworkID {
  /// The mainnet network ID (byte: 0x00).
  Mainnet,

  /// The testnet network ID (byte: 0x6f).
  Testnet,
}

impl NetworkID {
  /// Get a network ID from a given network ID byte.
  pub fn new(network_id_byte: u8) -> Result<Self> {
    match network_id_byte {
      MAINNET_ID_BYTE => Ok(Self::Mainnet),
      TESTNET_ID_BYTE => Ok(Self::Testnet),
      other => Err(Box::new(InvalidNetworkIDByte(other))),
    }
  }

  /// Get the network ID byte for this network ID.
  pub fn byte(&self) -> u8 {
    match self {
      NetworkID::Mainnet => MAINNET_ID_BYTE,
      NetworkID::Testnet => TESTNET_ID_BYTE,
    }
  }
}

/// Mainnet network ID byte.
pub const MAINNET_ID_BYTE: u8 = 0x00;

/// Testnet network ID byte.
pub const TESTNET_ID_BYTE: u8 = 0x6f;
