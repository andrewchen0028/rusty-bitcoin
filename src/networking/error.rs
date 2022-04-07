use std::fmt::Display;

/// Wrapper error types for the ```networking``` module.
#[derive(Debug)]
pub enum Error {
  /// Wrapper type for ```RecvError```.
  RecvError(async_std::channel::RecvError),
}

impl From<async_std::channel::RecvError> for Error {
  fn from(err: async_std::channel::RecvError) -> Self {
    Self::RecvError(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::RecvError(err) => write!(f, "{}", err),
    }
  }
}

impl std::error::Error for Error {}
