use std::{ffi::OsString, io::stdin};

use clap::StructOpt;

use crate::{
  ui::{
    cli::{Cli, Commands},
    error::Error,
  },
  util::{
    constants::NetworkID,
    types::{addr::Addr, amount::Amount},
  },
};

/// Parse inputs from stdin to CLI.
///
/// TODO: Comminucate with networking thread.
///
/// TODO: TUI integration (https://crates.io/crates/tui) & CLI coloring.
pub fn start_ui() -> Result<(), Error> {
  loop {
    println!();
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let cli = match Cli::try_parse_from(string_to_args(&buf).iter()) {
      Ok(cli) => cli,
      Err(err) => {
        println!("{}", err);
        continue;
      },
    };

    match cli.command {
      Commands::Balance { addr } => {
        println!("Getting balance of wallet {}\n", addr)
      },
      Commands::NewWallet => {
        println!("Created new wallet: {}\n", Addr::new(NetworkID::Mainnet)?)
      },
      Commands::Send { amount, unit, recipient } => {
        println!("Sending {} to {}\n", Amount::new(amount, unit)?, recipient);
      },
      Commands::Shutdown => {
        println!("TODO: Triggering graceful shutdown of this RBTC client\n")
      },
    }
  }
}

/// Convert a string to a series of args.
fn string_to_args(string: &str) -> Vec<OsString> {
  let mut args = vec![OsString::from("program_name")];

  for arg in string.split_whitespace() {
    args.push(arg.into());
  }
  args
}
