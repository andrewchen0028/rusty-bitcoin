use std::fmt::Display;

use super::block::{self, Block};
use crate::util::constants::SHA256_HASH_SIZE;

/// The active local chain.
pub struct ActiveChain {
  blocks: Vec<Block>,
}

impl ActiveChain {
  /// Initialize and return the active chain with the genesis block.
  pub fn new() -> Self {
    Self { blocks: vec![Block::genesis()] }
  }

  /// Validate and push a block to the end of this chain.
  ///
  /// Returns `IncorrectPrevBlockHash` if block to be pushed has value of
  /// `prev_block_hash` which does not match this chain's `last_hash`.
  pub fn validate_and_push(&mut self, block: Block) -> Result<(), Error> {
    match block.verify_nonce() {
      Ok(_) => {
        if block.prev_block_hash() == self.last_block_hash() {
          self.blocks.push(block);
          Ok(())
        } else {
          Err(Error::IncorrectPrevBlockHash)
        }
      },
      Err(err) => Err(Error::BlockError(err)),
    }
  }

  /// Get the hash of the last block in this chain.
  pub fn last_block_hash(&self) -> [u8; SHA256_HASH_SIZE] {
    self
      .blocks
      .last()
      .expect("Attempted to get last hash of empty chain")
      .hash()
  }

  /// Get the total relative work of all blocks in this chain.
  fn total_relative_work(&self) -> f64 {
    let mut total_relative_work = 0.0;
    for block in &self.blocks {
      total_relative_work += block.relative_work();
    }
    total_relative_work
  }
}

impl Default for ActiveChain {
  fn default() -> Self {
    Self::new()
  }
}

impl PartialEq for ActiveChain {
  fn eq(&self, other: &Self) -> bool {
    self.total_relative_work() == other.total_relative_work()
  }
}

impl PartialOrd for ActiveChain {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self
      .total_relative_work()
      .partial_cmp(&other.total_relative_work())
  }
}

/// Error type for `Chain` objects.
#[derive(Debug)]
pub enum Error {
  IncorrectPrevBlockHash,
  BlockError(block::Error),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IncorrectPrevBlockHash => write!(
        f,
        "Attempted to push block with incorrect previous block hash"
      ),
      Error::BlockError(err) => {
        write!(f, "BlockError: {}", err)
      },
    }
  }
}
