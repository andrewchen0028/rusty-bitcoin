/// A generic result type with a boxed error, for use throughout RBTC.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// An error enum covering all RBTC-specific errors.
#[derive(Debug)]
pub enum Error {
  /// Indicates failure to convert a ```Vec<u8>``` into a ```[u8]```.
  IntoArrayError(Vec<u8>),

  /// Indicates an invalid network ID byte.
  InvalidNetworkIDByte(u8),

  /// Indicates failure due to reading from an empty ```str```.
  EmptyStrError,

  /// Indicates an invalid currency unit suffix. Valid suffixes are:
  /// - ```nRBTC``` (nano)
  /// - ```uRBTC``` (micro)
  /// - ```mRBTC``` (milli)
  /// - ```RBTC``` (unit)
  /// - ```kRBTC``` (kilo)
  /// - ```MRBTC``` (mega)
  /// - ```GRBTC``` (giga)
  InvalidUnitSuffix(String),

  /// Indicates integer overflow.
  IntegerOverflow,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IntoArrayError(v) => {
        write!(f, "Failed to convert vector to array: {:?}", v)
      },
      Error::InvalidNetworkIDByte(byte) => {
        write!(f, "Invalid network ID byte: {}", byte)
      },
      Error::EmptyStrError => write!(f, "Attempted read from empty string"),
      Error::InvalidUnitSuffix(suffix) => {
        write!(f, "Invalid currency unit suffix: {}", suffix)
      },
      Error::IntegerOverflow => write!(f, "Integer overflow error"),
    }
  }
}

impl std::error::Error for Error {}
