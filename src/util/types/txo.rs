use serde::{Deserialize, Serialize};

use crate::util::constants::RIPEMD160_HASH_SIZE;

/// A transaction output.
#[derive(Debug, Deserialize, Serialize)]
pub struct Txo {
  /// The value of this transaction output.
  value: u64,

  /// RIPEMD160-SHA256 hash of the recipient's public key.
  /// NOTE: Not sure if this is right.
  pubkey_hash: [u8; RIPEMD160_HASH_SIZE],
}

impl Txo {
  /// Initialize a new transaction output from the provided values.
  pub fn new(value: u64, pubkey_hash: [u8; RIPEMD160_HASH_SIZE]) -> Self {
    Self { value, pubkey_hash }
  }
}
