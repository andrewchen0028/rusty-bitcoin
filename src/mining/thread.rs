use std::{
  process,
  time::{Duration, Instant},
};

use async_std::channel::{Receiver, Sender, TryRecvError};
use ethnum::u256;

use super::constants::CYCLE_BLOCK_LIMIT;
use crate::{
  mining::constants::{MAX_FACTOR, MIN_FACTOR, TARGET_BLOCK_TIME},
  util::{
    constants::SHA256_HASH_SIZE,
    hashes::sha256,
    types::{block::Block, txn::Txn},
  },
};

/// # Mining thread
/// Mines blocks. Before each hash attempt, check if thread has received an
/// incoming transaction or block from networking thread.
pub async fn start_mining(
  blks_from_network: Receiver<Block>,
  txns_from_network: Receiver<Txn>,
  blks_to_network: Sender<Block>,
  txns_to_network: Sender<Txn>,
) {
  // Initialize block counter, hash target, and mempool.
  let mut block_count = 0;
  let mut target = u256::MAX;

  // Measure start time of first block in cycle.
  let mut start_time = Instant::now();

  // Mine a block or update the local chain.
  loop {
    // Initialize previous block hash, mempool and nonce.
    let mut _prev_block_hash = [0u8; SHA256_HASH_SIZE];
    let mempool = Vec::<Txn>::new();
    let mut nonce: u32 = 0;

    // Concatenate mempool and nonce.
    let mut mempool_and_nonce = [
      bincode::serialize(&mempool).expect("Failed to serialize mempool"),
      nonce.to_le_bytes().to_vec(),
    ]
    .concat();

    // Try hashes until hash meets target. Before each attmept, check for and
    // handle any incoming transactions or blocks.
    while sha256(&mempool_and_nonce) > target.to_be_bytes() {
      // Handle any incoming transactions.
      match txns_from_network.try_recv() {
        Ok(txn) => {
          // TODO: Verify txn, then push to mempool and send to networking
          // thread for re-broadcast.
          println!("Verifying, pushing, and re-broadcasting txn: {:?}", txn);
          txns_to_network
            .send(txn)
            .await
            .expect("Failed to send transaction to networking thread");
        },
        Err(TryRecvError::Closed) => {
          println!("Channel txns_from_network closed unexpectedly");
          process::exit(1);
        },
        Err(TryRecvError::Empty) => {},
      }

      // TODO: Handle any incoming blocks (reset hash_prev, prune mempool).
      match blks_from_network.try_recv() {
        Ok(blk) => {
          println!("Handling incoming block: {:?}", blk);
          // TODO:
          // - Verify block before proceeding.
          // - Prune mempool against incoming block's transactions.
          _prev_block_hash = blk.hash();
        },
        Err(TryRecvError::Closed) => {
          println!("Channel blks_from_network closed unexpectedly");
          process::exit(1);
        },
        Err(TryRecvError::Empty) => {},
      }

      // Increment nonce, then re-concatenate mempool and nonce.
      nonce = nonce.checked_add(1).expect("Nonce overflowed");
      mempool_and_nonce = [
        bincode::serialize(&mempool).expect("Failed to serialize transaction"),
        nonce.to_le_bytes().to_vec(),
      ]
      .concat();
    }

    // Create block and send to networking thread.
    let block = Block::default();
    blks_to_network
      .send(block)
      .await
      .expect("Failed to send block to networking thread");

    // TODO: Implement method for counting blocks since last target adjustment.

    // Increment block counter.
    block_count += 1;

    // If counter has reached limit, adjust target and reset timer.
    if block_count % CYCLE_BLOCK_LIMIT == 0 {
      // Adjust target and reset cycle timer.
      adjust_target(&mut target, start_time.elapsed());
      start_time = Instant::now();
    }
  }
}

fn adjust_target(target: &mut u256, elapsed: Duration) {
  // Compute factor (elapsed time / target time), clamped to [0.25, 4].
  let elapsed_time = elapsed.as_secs_f64();
  let target_time = (TARGET_BLOCK_TIME * CYCLE_BLOCK_LIMIT).as_secs_f64();
  let factor = (elapsed_time / target_time).clamp(MIN_FACTOR, MAX_FACTOR);

  // Scale factor by 100, and target by 1/100.
  // TODO: Check division implementation.
  let scaled_factor = u256::from((factor * 100.0) as u64);
  let scaled_target = *target / u256::from(100u64);

  // Adjust target to (scaled_factor * scaled_target).
  // TODO: Catch and handle overflow.
  *target = scaled_factor * scaled_target;

  println!(
    "Mined {} blocks in {:.3}s, adjusting target by {:+}%",
    CYCLE_BLOCK_LIMIT,
    elapsed_time,
    scaled_factor.as_i16() - 100
  );
}
