use std::{ffi::OsString, io::stdin};

use async_std::channel::Sender;
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
/// TODO: TUI integration (https://crates.io/crates/tui) & CLI coloring.
pub async fn start_ui(s_commands: Sender<Commands>) -> Result<(), Error> {
  loop {
    logln!("Entered UI loop");
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
        logln!("Getting balance of wallet {}\n", addr);
        s_commands.send(Commands::Balance { addr }).await?;
      },
      Commands::NewWallet => {
        logln!("Created new wallet: {}\n", Addr::new(NetworkID::Mainnet)?);
        s_commands.send(Commands::NewWallet).await?;
      },
      Commands::Send { amount, unit, recipient } => {
        logln!("Sending {} to {}\n", Amount::new(amount, &unit)?, recipient);
        s_commands
          .send(Commands::Send { amount, unit, recipient })
          .await?;
      },
      Commands::Shutdown => {
        logln!("TODO: Triggering graceful shutdown of this RBTC client\n");
        s_commands.send(Commands::Shutdown).await?;
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
