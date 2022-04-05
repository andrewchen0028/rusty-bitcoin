use std::{io::stdin, str::FromStr, thread::spawn};

use clap::StructOpt;
use rand::{thread_rng, RngCore};
use rbtc::{
  logln,
  mining::miner::start_mining,
  ui::cli::{
    Cli,
    Commands::{Balance, Send},
  },
  util::{constants::NetworkID::Mainnet, errors::Result, types::WalletAddr},
};

/// Parse error-checked arguments and spawn node threads.
/// TODO: Rewrite or heavily scrutinize all files marked with "REWRITE".
/// # REWRITE
fn main() -> Result<()> {
  // Create and print local wallet address.
  let mut x = [0u8; 32];
  let mut y = [0u8; 32];
  thread_rng().try_fill_bytes(&mut x)?;
  thread_rng().try_fill_bytes(&mut y)?;
  let addr = WalletAddr::new(x, y, Mainnet)?;
  println!("{}", &addr);

  // Parse inputs from stdin to CLI.
  // TODO: Seems to be requiring "send send <ADDR> <AMOUNT>", needs fix.
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  let cli = Cli::parse_from(buf.split_whitespace());
  match &cli.command {
    Send { recipient, amount } => {
      let recipient_addr = WalletAddr::from_str(recipient)?;
      println!("Sending {} to {}", amount, recipient_addr);
    },
    Balance { addr } => {
      let addr = WalletAddr::from_str(addr)?;
      println!("Getting balance of {}", addr);
    },
  }

  // Start mining thread.
  let mining_thread = spawn(start_mining);
  match mining_thread.join() {
    Ok(_) => {},
    Err(err) => logln!("Failed to join mining thread, {:?}", err),
  }

  Ok(())
}
