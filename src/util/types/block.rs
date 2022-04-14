use std::fmt::Display;

use ethnum::u256;
use serde::{ser::SerializeSeq, Serialize};

use super::{header::Header, txn::Txn};
use crate::util::{constants::SHA256_HASH_SIZE, hashes::sha256};

/// A block.
#[derive(Debug)]
pub struct Block {
  header: Header,
  txn_count: u32,
  txns: Vec<Txn>,
}

impl Block {
  /// Initialize a new block from the given values.
  pub fn new(header: Header, txn_count: u32, txns: Vec<Txn>) -> Self {
    Self { header, txn_count, txns }
  }

  /// Return the genesis block.
  pub fn genesis() -> Self {
    Self { header: Header::genesis(), txn_count: 0, txns: Vec::new() }
  }

  /// Return the double-SHA-256 hash of this block's `header`.
  pub fn hash(&self) -> [u8; SHA256_HASH_SIZE] {
    self.header.hash()
  }

  /// Return this block's `prev_block_hash`.
  pub fn prev_block_hash(&self) -> [u8; SHA256_HASH_SIZE] {
    self.header.prev_block_hash()
  }

  /// Return the relative work done to mine this block.
  pub fn relative_work(&self) -> f64 {
    u256::MAX.as_f64() / self.header.target().as_f64()
  }

  /// Verify that this block's nonce is valid. Called in the networking thread
  /// to validate incoming blocks.
  pub fn verify_nonce(&self) -> Result<(), Error> {
    let txns_and_nonce = [
      bincode::serialize(&self.txns)
        .expect("Failed to serialize block transactions"),
      self.header.nonce().to_le_bytes().to_vec(),
    ]
    .concat();
    if u256::from_be_bytes(sha256(&sha256(&txns_and_nonce)))
      > self.header.target()
    {
      Err(Error::NonceFailedVerification)
    } else {
      Ok(())
    }
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

impl Default for Block {
  /// NOTE: For debugging purposes only.
  fn default() -> Self {
    Self { header: Header::default(), txn_count: 0, txns: Vec::new() }
  }
}

/// Error type for `Block`.
#[derive(Debug)]
pub enum Error {
  NonceFailedVerification,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::NonceFailedVerification => write!(f, "NonceFailedVerification"),
    }
  }
}
