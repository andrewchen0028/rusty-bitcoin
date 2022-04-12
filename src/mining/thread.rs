use std::{thread, time::Duration};

use async_std::{
  channel::{Receiver, Sender},
  task,
};

use super::mine::mine_block;
use crate::{
  mining::error::Error,
  util::types::{block::Block, transaction::Transaction},
};

/// # Mining thread
/// Asynchronously handles the following tasks:
/// - Receive incoming transactions and blocks
/// - Send transactions and blocks to networker for broadcast
/// TODO: Finish implementation.
pub async fn start_mining(
  blks_from_network: Receiver<Block>,
  txns_from_network: Receiver<Transaction>,
  blks_to_network: Sender<Block>,
  txns_to_network: Sender<Transaction>,
) -> Result<(), Error> {
  // Spawn and await async tasks.
  let receive_blks_handle = task::spawn(receive_blks(blks_from_network));
  let receive_txns_handle = task::spawn(receive_txns(txns_from_network));
  let send_blks_handle = task::spawn(send_blks(blks_to_network));
  let send_txns_handle = task::spawn(send_txns(txns_to_network));
  receive_blks_handle.await;
  receive_txns_handle.await;
  send_blks_handle.await;
  send_txns_handle.await;
  Ok(())
}

/// Receive blocks from the networking thread, forwarded from the network.
async fn receive_blks(blks_from_network: Receiver<Block>) {
  loop {
    // Receive blocks from network.
    match blks_from_network.recv().await {
      Ok(_) => println!("Received block from networker"),
      Err(err) => {
        println!("Failed to receive block from networker: {}", err)
      },
    }
  }
}

/// Receive transactions from the networking thread, forwarded from the network.
async fn receive_txns(txns_from_network: Receiver<Transaction>) {
  loop {
    // Receive transactions from network.
    match txns_from_network.recv().await {
      Ok(_) => println!("Received transaction from networker"),
      Err(err) => {
        println!("Failed to receive transaction from networker: {}", err)
      },
    }
  }
}

const DELAY_LONG: Duration = Duration::from_secs(17);

/// Send blocks to the networking thread for broadcasting to the network.
async fn send_blks(blks_to_network: Sender<Block>) {
  loop {
    // Await outgoing block.
    thread::sleep(DELAY_LONG);
    let blk = mine_block();

    // Send outgoing block to networker.
    match blks_to_network.send(blk).await {
      Ok(_) => {},
      Err(err) => {
        println!("Failed to send outgoing block to networker: {}", err)
      },
    }
  }
}

/// Send transactions to the networking thread for broadcasting to the network.
async fn send_txns(txns_to_network: Sender<Transaction>) {
  loop {
    // Await outgoing transaction.
    thread::sleep(DELAY_LONG);
    let transaction = Transaction { x: String::from("txn") };

    // Send transaction to networker.
    match txns_to_network.send(transaction).await {
      Ok(_) => {},
      Err(err) => {
        println!("Failed to send outgoing transaction to networker: {}", err)
      },
    };
  }
}
