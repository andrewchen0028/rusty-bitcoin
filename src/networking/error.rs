use std::fmt::Display;

use crate::util::types::amount;

/// Wrapper error types for the `networking` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper type for `RecvError`.
  RecvError(async_std::channel::RecvError),

  /// Wrapper type for `amount::Error`.
  AmountError(amount::Error),
}

impl From<async_std::channel::RecvError> for Error {
  fn from(err: async_std::channel::RecvError) -> Self {
    Self::RecvError(err)
  }
}

impl From<amount::Error> for Error {
  fn from(err: amount::Error) -> Self {
    Self::AmountError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RecvError(err) => write!(f, "{}", err),
      Error::AmountError(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Error {}
