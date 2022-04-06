use std::{array::TryFromSliceError, fmt::Display};

use rand::{thread_rng, RngCore};

use crate::util::{
  constants::{
    NetworkID, ADDR_INDEX_NETWORK_ID, ADDR_RANGE_CHECKSUM,
    ADDR_RANGE_UNDERLYING, ADDR_SIZE, ADDR_SIZE_CHECKSUM, ADDR_SIZE_UNDERLYING,
    MAINNET_ID_BYTE, TESTNET_ID_BYTE,
  },
  hashes::{ripemd160, sha256},
};

/// A 25-byte RBTC wallet address, comprised of a one-byte network ID, a 20-byte
/// address, and a 4-byte checksum.
pub struct Addr {
  /// 25-byte wallet address, comprised of network ID byte (1), address (20),
  /// checksum (4).
  bytes: [u8; ADDR_SIZE],
}

impl Addr {
  /// Create a wallet address from two 32-byte integers and a network ID.
  pub fn new(network_id: NetworkID) -> Result<Self, Error> {
    // Initialize random integers x and y.
    // TODO: Replace with integers derived from ECDSA public key.
    let (mut x, mut y) = ([0u8; 32], [0u8; 32]);
    match thread_rng().try_fill_bytes(&mut x) {
      Ok(_) => {},
      Err(err) => println!("Failed to fill with random bytes: {}", err),
    }
    match thread_rng().try_fill_bytes(&mut y) {
      Ok(_) => {},
      Err(err) => println!("Failed to fill with random bytes: {}", err),
    }

    // Concatenate x & y into [0x04, x, y].
    // TODO: Figure out what 0x04 is and replace if necessary.
    let mut v = vec![0x04u8];
    v.extend_from_slice(&x);
    v.extend_from_slice(&y);

    // Take ripemd160(sha256([0x04, x, y])) to get 20-byte array.
    let hash = ripemd160(&sha256(&v));

    // Concatenate network ID byte & 20-byte array into [network_id_byte, arr].
    let mut v = vec![network_id.byte()];
    v.extend_from_slice(&hash);

    // Get checksum from sha256(sha256([network_id_byte, arr])).
    let hash = sha256(&sha256(&v));
    let checksum = &hash[..ADDR_SIZE_CHECKSUM];

    // Concatenate [network_id_byte, arr] & checksum to get 25-byte address.
    v.extend_from_slice(checksum);
    match v.try_into() {
      Ok(addr_bytes) => Ok(Self { bytes: addr_bytes }),
      Err(bytes) => Err(Error::IntoArrayError(bytes)),
    }
  }

  /// Get the network ID of this wallet address.
  ///
  /// TODO: Make this exhaustive against the variants of ```NetworkID```.
  pub fn network_id(&self) -> Result<NetworkID, Error> {
    match self.bytes[ADDR_INDEX_NETWORK_ID] {
      MAINNET_ID_BYTE => Ok(NetworkID::Mainnet),
      TESTNET_ID_BYTE => Ok(NetworkID::Testnet),
      byte => Err(Error::InvalidNetworkID(byte)),
    }
  }

  /// Get the 20 underlying bytes of this wallet address.: std::error::Error
  pub fn get_underlying(&self) -> Result<[u8; ADDR_SIZE_UNDERLYING], Error> {
    Ok(self.bytes[ADDR_RANGE_UNDERLYING].try_into()?)
  }

  /// Get the 4-byte checksum of this wallet address.
  pub fn get_checksum(&self) -> Result<[u8; ADDR_SIZE_CHECKSUM], Error> {
    Ok(self.bytes[ADDR_RANGE_CHECKSUM].try_into()?)
  }

  /// Base58Check-encode a wallet address to a string.
  fn encode(&self) -> String {
    bs58::encode(self.bytes).into_string()
  }

  /// Base58Check-decode a string to a wallet address.
  fn decode(s: String) -> Result<Self, Error> {
    let vec = bs58::decode(s).into_vec()?;
    match vec.try_into() {
      Ok(bytes) => Ok(Self { bytes }),
      Err(bytes) => Err(Error::IntoArrayError(bytes)),
    }
  }
}

impl std::fmt::Display for Addr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.encode())
  }
}

impl std::str::FromStr for Addr {
  type Err = Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    Addr::decode(s.to_string())
  }
}

/// The custom error type for ```Addr```.
#[derive(Debug)]
pub enum Error {
  /// Indicates an error decoding a string using Base58.
  Base58DecodeError(bs58::decode::Error),

  /// Indicates an error converting a ```Vec<u8>``` into an array.
  IntoArrayError(Vec<u8>),

  /// Indicates an invalid network ID byte.
  InvalidNetworkID(u8),

  /// Indicates an error converting from a slice.
  TryFromSliceError(std::array::TryFromSliceError),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::Base58DecodeError(err) => write!(f, "Base58DecodeError: {}", err),
      Error::IntoArrayError(v) => write!(f, "IntoArrayError: {:?}", v),
      Error::InvalidNetworkID(b) => write!(f, "InvalidNetworkID: {}", b),
      Error::TryFromSliceError(err) => write!(f, "TryFromSliceError: {}", err),
    }
  }
}

impl From<bs58::decode::Error> for Error {
  fn from(err: bs58::decode::Error) -> Self {
    Error::Base58DecodeError(err)
  }
}

impl From<TryFromSliceError> for Error {
  fn from(err: TryFromSliceError) -> Self {
    Error::TryFromSliceError(err)
  }
}

impl std::error::Error for Error {}
