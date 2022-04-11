use std::fmt::Display;

/// Wrapper error types for the `mining` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper type for `io::Error`.
  IOError(std::io::Error),

  /// Wrapper type for `rand::Error`.
  RandError(rand::Error),
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    Self::IOError(err)
  }
}

impl From<rand::Error> for Error {
  fn from(err: rand::Error) -> Self {
    Self::RandError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IOError(err) => write!(f, "{}", err),
      Error::RandError(err) => write!(f, "{}", err),
    }
  }
}
