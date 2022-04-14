use serde::{Deserialize, Serialize};

use crate::util::constants::SHA256_HASH_SIZE;

/// A transaction input.
#[derive(Debug, Deserialize, Serialize)]
pub struct Txi {
  /// Previous transaction hash.
  prev_txn_hash: [u8; SHA256_HASH_SIZE],

  /// Previous transaction output index.
  prev_txo_index: usize,

  /// Previous transaction output owner's digital signature.
  /// NOTE: Not sure if this is right.
  prev_txn_sig: [u8; SHA256_HASH_SIZE],
}

impl Txi {
  /// Initialize a new transaction input from the provided values.
  pub fn new(
    prev_txn_hash: [u8; SHA256_HASH_SIZE],
    prev_txo_index: usize,
    prev_txn_sig: [u8; SHA256_HASH_SIZE],
  ) -> Self {
    Self { prev_txn_hash, prev_txo_index, prev_txn_sig }
  }
}
