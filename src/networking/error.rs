use std::fmt::Display;

/// Wrapper error types for the ```networking``` module.
#[derive(Debug)]
pub enum Error {}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "TODO")
  }
}

impl std::error::Error for Error {}
