use clap::{Parser, Subcommand};

use crate::util::types::{addr::Addr, units::Unit};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  /// Get the RBTC balance of the specified wallet address.
  Balance { addr: Addr },

  /// Create a new wallet and print its address.
  NewWallet,

  /// Send some RBTC to the recipient wallet address.
  Send {
    amount: f64,
    unit: Unit,
    recipient: Addr,
  },

  /// TODO: Trigger a graceful shutdown of this RBTC client.
  Shutdown,
}
