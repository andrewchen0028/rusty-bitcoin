use std::time::{Duration, Instant};

use ethnum::u256;
use rand::{thread_rng, RngCore};

use super::constants::CYCLE_BLOCK_LIMIT;
use crate::{
  mining::constants::{MAX_FACTOR, MIN_FACTOR, TARGET_BLOCK_TIME},
  util::{constants::SHA256_HASH_SIZE, hashes::sha256, types::block::Block},
};

/// Mine a block.
pub fn mine_block() -> Block {
  // Initialize block counter to zero, and hash target to largest value
  // representable by a 256-bit unsigned integer.
  let mut cycle_block_count = 0;
  let mut target = u256::MAX;

  // TODO: Asynchronously append serialized transactions.
  let mut block_tx_data = [0u8; SHA256_HASH_SIZE];

  // TODO: Asynchronously update previous block.

  // Measure start time of first block counter cycle.
  let mut start_time = Instant::now();

  // Mine RBTC.
  loop {
    match thread_rng().try_fill_bytes(&mut block_tx_data) {
      Ok(_) => {},
      Err(err) => println!("Failed to fill random bytes: {}", err),
    }

    // Initialize nonce, and concatenate with block transaction data.
    let mut nonce: u32 = 0;
    let mut data_and_nonce = Vec::from_iter(block_tx_data);
    data_and_nonce.extend(nonce.to_le_bytes());

    // Try hashes until hash meets target.
    // Check for new transactions/blocks before each attempt, and update
    // block_tx_data and hash_prev_block as necessary.
    while sha256(&data_and_nonce) > target.to_be_bytes() {
      // Increment nonce, then re-concatenate with data.
      // TODO: Handle nonce overflow.
      nonce += 1;
      data_and_nonce = Vec::from_iter(block_tx_data);
      data_and_nonce.extend(nonce.to_le_bytes());
    }

    // TODO: Create block, serialize, and send to networking thread.
    let block = Block::new(block_tx_data, nonce);

    // Increment block counter.
    cycle_block_count += 1;

    // If counter has reached limit, adjust target and reset counter.
    if cycle_block_count == CYCLE_BLOCK_LIMIT {
      // Adjust target, then reset block counter and cycle timer.
      adjust_target(&mut target, start_time.elapsed());
      cycle_block_count = 0;
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
