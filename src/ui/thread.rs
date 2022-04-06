use std::{ffi::OsString, io::stdin};

use clap::StructOpt;

use crate::{
  logln,
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
/// TODO: TUI integration (https://crates.io/crates/tui).
pub fn start_ui() -> Result<(), Error> {
  loop {
    logln!("--------------------");
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
      Ok(_) => {},
      Err(err) => {
        println!("Failed to read line: {}", err);
        continue;
      },
    }

    let cli = match Cli::try_parse_from(string_to_args(&buf).iter()) {
      Ok(cli) => cli,
      Err(err) => {
        println!("Failed to parse input to CLI: {}", err);
        continue;
      },
    };

    match cli.command {
      Commands::Balance { addr } => {
        println!("Getting balance of {}", addr);
      },
      Commands::NewWallet {} => {
        let addr = Addr::new(NetworkID::Mainnet)?;
        println!("Created new wallet with address {}", addr);
      },
      Commands::Send { amount, unit, recipient } => {
        let amount = match Amount::new(amount, unit) {
          Ok(amount) => amount,
          Err(err) => {
            println!("{}", err);
            continue;
          },
        };
        println!("Sending {} to {}", amount, recipient);
      },
      Commands::Shutdown => {
        todo!()
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
