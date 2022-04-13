use serde::{ser::SerializeSeq, Serialize};

use super::transaction::Transaction;
use crate::util::{constants::SHA256_HASH_SIZE, hashes::sha256};

/// A block.
#[derive(Debug)]
pub struct Block {
  nonce: u32,
  prev_block_hash: [u8; SHA256_HASH_SIZE],
  txns: Vec<Transaction>,
}

impl Block {
  /// Initialize a new block from the given values.
  pub fn new(
    nonce: u32,
    prev_block_hash: [u8; SHA256_HASH_SIZE],
    txns: Vec<Transaction>,
  ) -> Self {
    Self { nonce, prev_block_hash, txns }
  }

  /// Return the double-SHA-256 hash of this block's data.
  pub fn hash(&self) -> [u8; SHA256_HASH_SIZE] {
    sha256(&sha256(
      &bincode::serialize(self).expect("Failed to serialize block"),
    ))
  }
}

impl Serialize for Block {
  /// Serialize this block.
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.txns.len()))?;
    for e in &self.txns {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
