/// Prints to the standard error, prefixed by the path to the invoking line and
/// column in the source code.
#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    eprint!(
      "{}:{}:{}: {}",
      file!(),
      line!(),
      column!(),
      format_args!($($arg)*))
  }
}

/// Prints to the standard error with a newline, prefixed by the path to the
/// invoking line and column in the source code.
#[macro_export]
macro_rules! logln {
  ($($arg:tt)*) => {
    eprintln!(
      "{}:{}:{}: {}",
      file!(),
      line!(),
      column!(),
      format_args!($($arg)*))
  }
}
