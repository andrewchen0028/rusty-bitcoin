use std::fmt::Display;

use crate::{ui, util};

/// Wrapper error types for the `ui` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper type for `addr::Error`.
  AddrError(util::types::addr::Error),

  /// Wrapper type for `amount::Error`.
  AmountError(util::types::amount::Error),

  /// Wrapper type for `io::Error`.
  IOError(std::io::Error),

  /// Wrapper type for `SendError<Commands>`.
  SendError(async_std::channel::SendError<ui::cli::Commands>),
}

impl From<util::types::addr::Error> for Error {
  fn from(err: util::types::addr::Error) -> Self {
    Self::AddrError(err)
  }
}

impl From<util::types::amount::Error> for Error {
  fn from(err: util::types::amount::Error) -> Self {
    Self::AmountError(err)
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Self::IOError(err)
  }
}

impl From<async_std::channel::SendError<ui::cli::Commands>> for Error {
  fn from(err: async_std::channel::SendError<ui::cli::Commands>) -> Self {
    Self::SendError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::AddrError(err) => write!(f, "{}", err),
      Error::AmountError(err) => write!(f, "{}", err),
      Error::IOError(err) => write!(f, "{}", err),
      Error::SendError(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Error {}
