use std::fmt::Display;

use super::{addr, amount, units};

/// Wrapper error types for the ```util::types``` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper error type for the ```util::types::addr``` module.
  AddrError(addr::Error),

  /// Wrapper error type for the ```util::types::amount``` module.
  AmountError(amount::Error),

  /// Wrapper error type for the ```util::types::units``` module.
  UnitError(units::Error),
}

impl From<addr::Error> for Error {
  fn from(err: addr::Error) -> Self {
    Error::AddrError(err)
  }
}

impl From<amount::Error> for Error {
  fn from(err: amount::Error) -> Self {
    Error::AmountError(err)
  }
}

impl From<units::Error> for Error {
  fn from(err: units::Error) -> Self {
    Error::UnitError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::AddrError(err) => write!(f, "{}", err),
      Error::AmountError(err) => write!(f, "{}", err),
      Error::UnitError(err) => write!(f, "{}", err),
    }
  }
}
