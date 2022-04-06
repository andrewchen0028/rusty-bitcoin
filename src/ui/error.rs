use std::fmt::Display;

use crate::util::{self, types::addr};

/// Wrapper error types for the ```ui``` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper error type for ```addr::Error```.
  AddrError(addr::Error),
}

impl From<util::types::addr::Error> for Error {
  fn from(err: util::types::addr::Error) -> Self {
    Self::AddrError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::AddrError(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Error {}
