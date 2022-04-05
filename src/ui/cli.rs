use clap::{Parser, Subcommand};

/// # REWRITE
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Send some RBTC to the recipient wallet address.
  Send { recipient: String, amount: u64 },

  /// Get the RBTC balance of the specified wallet address.
  Balance { addr: String },
}
