use serde::{Deserialize, Serialize};

use super::amount::Amount;
use crate::util::{constants::SHA256_HASH_SIZE, hashes::sha256};

/// A transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
  amount: Amount,
}

impl Transaction {
  /// Initialize a new transaction of the provided amount.
  pub fn new(amount: Amount) -> Self {
    Self { amount }
  }

  /// Return the hash of this transaction's data.
  pub fn hash(&self) -> [u8; SHA256_HASH_SIZE] {
    sha256(&sha256(
      &bincode::serialize(self).expect("Failed to serialize transaction"),
    ))
  }
}
