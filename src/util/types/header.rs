use std::time::Duration;

use ethnum::u256;

/// A block header.
#[allow(dead_code)]
pub struct Header {
  version: u32,
  hash_prev_block: u256,
  hash_merkle_root: u256,
  time: Duration,
  bits: u256,
  nonce: u32,
}

impl Header {
  pub fn new(
    version: u32,
    hash_prev_block: u256,
    hash_merkle_root: u256,
    time: Duration,
    bits: u256,
    nonce: u32,
  ) -> Self {
    Self { version, hash_prev_block, hash_merkle_root, time, bits, nonce }
  }
}
