use super::types;

/// Wrapper error types for the ```util``` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper for ```type``` module errors.
  TypeError(types::error::Error),
}

impl From<types::error::Error> for Error {
  fn from(err: types::error::Error) -> Self {
    Error::TypeError(err)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::TypeError(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Error {}
