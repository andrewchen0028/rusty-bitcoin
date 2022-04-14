use std::time::{SystemTime, UNIX_EPOCH};

use ethnum::u256;

use crate::util::{constants::SHA256_HASH_SIZE, hashes::sha256};

/// A block header.
#[derive(Debug)]
pub struct Header {
  version: u32,
  prev_block_hash: [u8; SHA256_HASH_SIZE],
  merkle_root: [u8; SHA256_HASH_SIZE],
  timestamp: u32,
  bits: u32,
  nonce: u32,
}

impl Header {
  /// Initialize a new header from the provided values.
  pub fn new(
    version: u32,
    prev_block_hash: [u8; SHA256_HASH_SIZE],
    merkle_root: [u8; SHA256_HASH_SIZE],
    timestamp: u32,
    bits: u32,
    nonce: u32,
  ) -> Self {
    Self { version, prev_block_hash, merkle_root, timestamp, bits, nonce }
  }

  /// Return the genesis block header.
  pub fn genesis() -> Self {
    Self {
      version: 0,
      prev_block_hash: [0u8; SHA256_HASH_SIZE],
      merkle_root: [0u8; SHA256_HASH_SIZE],
      timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since UNIX epoch")
        .as_secs() as u32,
      bits: 0,
      nonce: 0,
    }
  }

  /// Return the double-SHA-256 hash of this `header`.
  pub fn hash(&self) -> [u8; SHA256_HASH_SIZE] {
    let mut header_bytes = Vec::<u8>::new();
    header_bytes.extend_from_slice(&self.version.to_le_bytes());
    header_bytes.extend_from_slice(&self.prev_block_hash);
    header_bytes.extend_from_slice(&self.merkle_root);
    header_bytes.extend_from_slice(&self.timestamp.to_le_bytes());
    header_bytes.extend_from_slice(&self.bits.to_le_bytes());
    header_bytes.extend_from_slice(&self.nonce.to_le_bytes());
    sha256(&sha256(&header_bytes))
  }

  /// Return the target from this header's `bits` field.
  pub fn target(&self) -> u256 {
    let be_bytes = self.bits.to_be_bytes();

    let mut coeff_bytes = [0u8; 4];
    coeff_bytes[1..be_bytes.len()].copy_from_slice(&be_bytes[1..]);

    let coeff = u256::from(u32::from_be_bytes(coeff_bytes));
    let exp = 8 * (be_bytes[0] - 3) as u32;
    let base = u256::from(2u8);
    coeff * base.pow(exp)
  }

  /// Return this header's `nonce`.
  pub fn nonce(&self) -> u32 {
    self.nonce
  }

  /// Return this header's `prev_block_hash`.
  pub fn prev_block_hash(&self) -> [u8; SHA256_HASH_SIZE] {
    self.prev_block_hash
  }
}

impl Default for Header {
  /// NOTE: For debugging purposes only.
  fn default() -> Self {
    Self {
      version: 0,
      prev_block_hash: [0u8; SHA256_HASH_SIZE],
      merkle_root: [0u8; SHA256_HASH_SIZE],
      timestamp: 0,
      bits: 0,
      nonce: 0,
    }
  }
}
