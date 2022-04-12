use crate::util::constants::SHA256_HASH_SIZE;

/// A block.
///
/// TODO: Make this actually store data.
pub struct Block {
  block_tx_data: [u8; SHA256_HASH_SIZE],
  nonce: u32,
}

impl Block {
  pub fn new(block_tx_data: [u8; SHA256_HASH_SIZE], nonce: u32) -> Self {
    Self { block_tx_data, nonce }
  }
}
