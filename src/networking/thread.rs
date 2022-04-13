use std::{thread, time::Duration};

use async_std::{
  channel::{Receiver, Sender},
  task,
};

use super::error::Error;
use crate::util::{
  constants::SHA256_HASH_SIZE,
  types::{
    amount::Amount, block::Block, transaction::Transaction, units::Unit,
  },
};

/// # Networking thread
/// Asynchronously handles the following tasks:
/// - Forward incoming transactions and blocks from network to miner
/// - Broadcast outgoing transactions and blocks from miner to network
pub async fn start_networking(
  blks_from_miner: Receiver<Block>,
  txns_from_miner: Receiver<Transaction>,
  blks_to_miner: Sender<Block>,
  txns_to_miner: Sender<Transaction>,
) -> Result<(), Error> {
  // Spawn and await async tasks.
  let broadcast_blks_handle = task::spawn(broadcast_blks(blks_from_miner));
  let broadcast_txns_handle = task::spawn(broadcast_txns(txns_from_miner));
  let forward_blks_handle = task::spawn(forward_blks(blks_to_miner));
  let forward_txns_handle = task::spawn(forward_txns(txns_to_miner));
  broadcast_blks_handle.await;
  broadcast_txns_handle.await;
  forward_blks_handle.await;
  forward_txns_handle.await?;
  Ok(())
}

/// Receive blocks from the mining thread and broadcast to the network.
async fn broadcast_blks(blks_from_miner: Receiver<Block>) {
  loop {
    // Broadcast outgoing blocks from miner to network.
    match blks_from_miner.recv().await {
      Ok(_) => println!("Broadcasting outgoing block from miner"),
      Err(err) => {
        println!("Failed to receive outgoing block from miner: {}", err)
      },
    }
  }
}

/// Receive transactions from the mining thread and broadcast to the network.
async fn broadcast_txns(txns_from_miner: Receiver<Transaction>) {
  loop {
    // Broadcast outgoing transactions to network.
    match txns_from_miner.recv().await {
      Ok(_) => println!("Broadcasting outgoing transaction from miner"),
      Err(err) => {
        println!("Failed to receive outgoing transaction from miner: {}", err)
      },
    }
  }
}

const DELAY_MEDIUM: Duration = Duration::from_secs(4);

/// Receive blocks from the network and forward to the mining thread.
async fn forward_blks(blks_to_miner: Sender<Block>) {
  loop {
    // TODO: Await incoming block from network.
    thread::sleep(DELAY_MEDIUM);
    let blk = Block::new(0, [0u8; SHA256_HASH_SIZE], Vec::new());

    // Forward incoming block to miner.
    match blks_to_miner.send(blk).await {
      Ok(_) => {},
      Err(err) => println!("Failed to send incoming block to miner: {}", err),
    }
  }
}

const DELAY_SHORT: Duration = Duration::from_millis(900);

/// Receive transactions from the network and forward to the mining thread.
async fn forward_txns(txns_to_miner: Sender<Transaction>) -> Result<(), Error> {
  loop {
    // Await transaction from network.
    thread::sleep(DELAY_SHORT);
    let transaction = Transaction::new(Amount::new(0.0, Unit::RBTC)?);

    // Forward transaction to miner.
    match txns_to_miner.send(transaction).await {
      Ok(_) => {},
      Err(err) => {
        println!("Failed to forward incoming transaction to miner: {}", err)
      },
    };
  }
}
