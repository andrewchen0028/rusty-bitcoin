use std::{panic, thread};

use async_std::{channel, task};
use rbtc::{
  mining::thread::start_mining,
  networking::thread::start_networking,
  util::types::{block::Block, transaction::Transaction},
};

/// TODO: Rewrite or heavily scrutinize all files marked with "REWRITE".
fn main() {
  // Initialize inter-thread communication channels.
  let (blks_to_miner, blks_from_network) = channel::unbounded::<Block>();
  let (blks_to_network, blks_from_miner) = channel::unbounded::<Block>();
  let (txns_to_miner, txns_from_network) = channel::unbounded::<Transaction>();
  let (txns_to_network, txns_from_miner) = channel::unbounded::<Transaction>();

  // Spawn mining and networking threads.
  let mining_thread = thread::spawn(|| {
    task::block_on(start_mining(
      blks_from_network,
      txns_from_network,
      blks_to_network,
      txns_to_network,
    ))
  });
  let networking_thread = thread::spawn(|| {
    task::block_on(start_networking(
      blks_from_miner,
      txns_from_miner,
      blks_to_miner,
      txns_to_miner,
    ))
  });

  // TODO: Figure out what "panic::resume_unwind()" thing does.
  match mining_thread.join() {
    Ok(_) => println!("Exited mining thread"),
    Err(err) => panic::resume_unwind(err),
  }
  match networking_thread.join() {
    Ok(_) => println!("Exited networking thread"),
    Err(err) => panic::resume_unwind(err),
  }
}
