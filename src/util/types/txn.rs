use serde::{Deserialize, Serialize};

use super::{txi::Txi, txo::Txo};
use crate::util::{constants::SHA256_HASH_SIZE, hashes::sha256};

/// A transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct Txn {
  version: u32,
  txi_count: u32,
  txi_list: Vec<Txi>,
  txo_count: u32,
  txo_list: Vec<Txo>,
}

impl Txn {
  /// Initialize a new transaction from the provided values.
  pub fn new(
    version: u32,
    txi_count: u32,
    txi_list: Vec<Txi>,
    txo_count: u32,
    txo_list: Vec<Txo>,
  ) -> Self {
    Self { version, txi_count, txi_list, txo_count, txo_list }
  }

  /// Return the hash of this transaction's data.
  pub fn hash(&self) -> [u8; SHA256_HASH_SIZE] {
    sha256(&sha256(
      &bincode::serialize(self).expect("Failed to serialize transaction"),
    ))
  }
}
