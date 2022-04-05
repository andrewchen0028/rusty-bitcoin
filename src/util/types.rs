use std::{fmt::Display, str::FromStr};

use super::constants::{
  NANO_FROM_GIGA, NANO_FROM_KILO, NANO_FROM_MEGA, NANO_FROM_MICRO,
  NANO_FROM_MILLI, NANO_FROM_NANO, NANO_FROM_UNIT,
};
use crate::util::{
  constants::{
    NetworkID, WALLET_ADDR_INDEX_NETWORK_ID, WALLET_ADDR_RANGE_CHECKSUM,
    WALLET_ADDR_RANGE_UNDERLYING, WALLET_ADDR_SIZE, WALLET_ADDR_SIZE_CHECKSUM,
    WALLET_ADDR_SIZE_UNDERLYING,
  },
  errors::{
    Error::{IntegerOverflow, IntoArrayError, InvalidUnitSuffix},
    Result,
  },
  hashes::{ripemd160, sha256},
};

/// An amount of RBTC.
#[derive(Debug)]
pub struct Amount {
  /// The underlying amount, in indivisible nanoRBTC.
  pub underlying: u64,
}

impl FromStr for Amount {
  type Err = Box<dyn std::error::Error>;

  /// Parse a string to an amount, rounding any decimals after the nano- digit.
  /// Requires one of the following unit suffixes:
  /// - ```nRBTC``` (nano)
  /// - ```uRBTC``` (micro)
  /// - ```mRBTC``` (milli)
  /// - ```RBTC``` (unit)
  /// - ```kRBTC``` (kilo)
  /// - ```MRBTC``` (mega)
  /// - ```GRBTC``` (giga)
  fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
    // Assign factor from string suffix and strip suffix.
    let factor = if s.ends_with("nRBTC") {
      s = s
        .strip_suffix("nRBTC")
        .expect("Failed to strip suffix \"nRBTC\".");
      NANO_FROM_NANO
    } else if s.ends_with("uRBTC") {
      s = s
        .strip_suffix("uRBTC")
        .expect("Failed to strip suffix \"uRBTC\".");
      NANO_FROM_MICRO
    } else if s.ends_with("mRBTC") {
      s = s
        .strip_suffix("mRBTC")
        .expect("Failed to strip suffix \"mRBTC\".");
      NANO_FROM_MILLI
    } else if s.ends_with("kRBTC") {
      s = s
        .strip_suffix("kRBTC")
        .expect("Failed to strip suffix \"kRBTC\".");
      NANO_FROM_KILO
    } else if s.ends_with("MRBTC") {
      s = s
        .strip_suffix("MRBTC")
        .expect("Failed to strip suffix \"MRBTC\".");
      NANO_FROM_MEGA
    } else if s.ends_with("GRBTC") {
      s = s
        .strip_suffix("GRBTC")
        .expect("Failed to strip suffix \"GRBTC\".");
      NANO_FROM_GIGA
    } else if s.ends_with("RBTC") {
      s = s
        .strip_suffix("RBTC")
        .expect("Failed to strip suffix \"RBTC\".");
      NANO_FROM_UNIT
    } else {
      return Err(Box::new(InvalidUnitSuffix(s.to_string())));
    };

    // Parse remainder of string to amount and multiply by factor, with u64
    // bounds checking.
    match s.parse::<f64>() {
      Ok(float) => {
        if float * factor > u64::MAX as f64 {
          Err(Box::new(IntegerOverflow))
        } else {
          Ok(Self { underlying: (float * factor) as u64 })
        }
      },
      Err(err) => Err(Box::new(err)),
    }
  }
}

impl Display for Amount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.underlying < NANO_FROM_NANO as u64 {
      write!(f, "0 RBTC")
    } else if self.underlying < NANO_FROM_MICRO as u64 {
      write!(f, "{} nRBTC", self.underlying)
    } else if self.underlying < NANO_FROM_MILLI as u64 {
      write!(f, "{} uRBTC", self.underlying as f64 / NANO_FROM_MICRO)
    } else if self.underlying < NANO_FROM_UNIT as u64 {
      write!(f, "{} mRBTC", self.underlying as f64 / NANO_FROM_MILLI)
    } else if self.underlying < NANO_FROM_KILO as u64 {
      write!(f, "{} RBTC", self.underlying as f64 / NANO_FROM_UNIT)
    } else if self.underlying < NANO_FROM_MEGA as u64 {
      write!(f, "{} kRBTC", self.underlying as f64 / NANO_FROM_KILO)
    } else if self.underlying < NANO_FROM_GIGA as u64 {
      write!(f, "{} MRBTC", self.underlying as f64 / NANO_FROM_MEGA)
    } else {
      write!(f, "{} GRBTC", self.underlying as f64 / NANO_FROM_GIGA)
    }
  }
}

/// A 25-byte RBTC wallet address, comprised of a one-byte network ID, a 20-byte
/// address, and a 4-byte checksum.
pub struct WalletAddr {
  /// 25-byte wallet address, comprised of network ID byte (1), address (20),
  /// checksum (4).
  bytes: [u8; WALLET_ADDR_SIZE],
}

impl WalletAddr {
  /// Create a wallet address from two 32-byte integers and a network ID.
  /// TODO: Replace hardcoded input integer sizes with integers derived from
  /// ECDSA public key.
  pub fn new(x: [u8; 32], y: [u8; 32], network_id: NetworkID) -> Result<Self> {
    // Concatenate x & y into [0x04, x, y].
    let mut v = vec![0x04u8];
    v.extend_from_slice(&x);
    v.extend_from_slice(&y);

    // Take r160s256([0x04, x, y]) to get 20-byte array.
    let hash = ripemd160(&sha256(&v));

    // Concatenate network ID byte & 20-byte array into [network_id_byte, arr].
    let mut v = vec![network_id.byte()];
    v.extend_from_slice(&hash);

    // Get checksum from s256s256([network_id_byte, arr]).
    let hash = sha256(&sha256(&v));
    let checksum = &hash[..WALLET_ADDR_SIZE_CHECKSUM];

    // Concatenate [network_id_byte, arr] & checksum to get 25-byte address.
    v.extend_from_slice(checksum);
    match v.try_into() {
      Ok(addr_bytes) => Ok(Self { bytes: addr_bytes }),
      Err(bytes) => Err(Box::new(IntoArrayError(bytes))),
    }
  }

  /// Get the network ID of this wallet address.
  pub fn network_id(&self) -> Result<NetworkID> {
    NetworkID::new(self.bytes[WALLET_ADDR_INDEX_NETWORK_ID])
  }

  /// Get the 20 underlying bytes of this wallet address.
  pub fn get_underlying(&self) -> [u8; WALLET_ADDR_SIZE_UNDERLYING] {
    self.bytes[WALLET_ADDR_RANGE_UNDERLYING]
      .try_into()
      .expect("Failed to get wallet address underlying bytes")
  }

  /// Get the 4-byte checksum of this wallet address.
  pub fn get_checksum(&self) -> [u8; WALLET_ADDR_SIZE_CHECKSUM] {
    self.bytes[WALLET_ADDR_RANGE_CHECKSUM]
      .try_into()
      .expect("Failed to get wallet address checksum bytes")
  }

  /// Base58Check-encode a wallet address to a string.
  fn encode(&self) -> String {
    bs58::encode(self.bytes).into_string()
  }

  /// Base58Check-decode a string to a wallet address.
  fn decode(s: String) -> Result<Self> {
    match bs58::decode(s).into_vec() {
      Ok(vec) => match vec.try_into() {
        Ok(addr_bytes) => Ok(Self { bytes: addr_bytes }),
        Err(bytes) => Err(Box::new(IntoArrayError(bytes))),
      },
      Err(bs58_err) => Err(Box::new(bs58_err)),
    }
  }
}

impl std::fmt::Display for WalletAddr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.encode())
  }
}

impl std::str::FromStr for WalletAddr {
  type Err = Box<dyn std::error::Error>;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    WalletAddr::decode(s.to_string())
  }
}
