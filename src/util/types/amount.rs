use std::fmt::Display;

use crate::util::{
  constants::{
    NANO_FROM_GIGA, NANO_FROM_KILO, NANO_FROM_MEGA, NANO_FROM_MICRO,
    NANO_FROM_MILLI, NANO_FROM_NANO, NANO_FROM_UNIT,
  },
  types::units::Unit,
};

/// An amount of RBTC.
#[derive(Debug)]
pub struct Amount {
  /// The underlying amount, in indivisible nanoRBTC.
  underlying: u64,
}

impl Amount {
  /// Create a new `Amount` from an `f64` and a `Unit`.
  ///
  /// # Examples
  /// ```
  /// let one = Amount::new(1.0, Unit::RBTC)?;
  /// assert_eq!(one.underlying, 1e9);
  ///
  /// let one_thousand = Amount::new(1.0, Unit::kRBTC)?;
  /// assert_eq!(one_thousand.underlying, 1e9 * 1e3);
  /// ```
  pub fn new(amount: f64, unit: &Unit) -> Result<Self, Error> {
    let factor = match unit {
      Unit::nRBTC => NANO_FROM_NANO,
      Unit::uRBTC => NANO_FROM_MICRO,
      Unit::mRBTC => NANO_FROM_MILLI,
      Unit::RBTC => NANO_FROM_UNIT,
      Unit::kRBTC => NANO_FROM_KILO,
      Unit::MRBTC => NANO_FROM_MEGA,
      Unit::GRBTC => NANO_FROM_GIGA,
    };
    if amount * factor > u64::MAX as f64 {
      Err(Error::IntegerOverflow)
    } else {
      Ok(Self { underlying: (amount * factor) as u64 })
    }
  }
}

impl std::fmt::Display for Amount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.underlying < NANO_FROM_NANO as u64 {
      write!(f, "0 RBTC")
    } else if self.underlying < NANO_FROM_MICRO as u64 {
      write!(f, "{} nRBTC", self.underlying)
    } else if self.underlying < NANO_FROM_MILLI as u64 {
      write!(f, "{} uRBTC", self.underlying as f64 / NANO_FROM_MICRO)
    } else if self.underlying < NANO_FROM_UNIT as u64 {
      write!(f, "{} mRBTC", self.underlying as f64 / NANO_FROM_MILLI)
    } else if self.underlying < NANO_FROM_KILO as u64 {
      write!(f, "{} RBTC", self.underlying as f64 / NANO_FROM_UNIT)
    } else if self.underlying < NANO_FROM_MEGA as u64 {
      write!(f, "{} kRBTC", self.underlying as f64 / NANO_FROM_KILO)
    } else if self.underlying < NANO_FROM_GIGA as u64 {
      write!(f, "{} MRBTC", self.underlying as f64 / NANO_FROM_MEGA)
    } else {
      write!(f, "{} GRBTC", self.underlying as f64 / NANO_FROM_GIGA)
    }
  }
}

/// Error type for `Amount` objects.
#[derive(Debug)]
pub enum Error {
  /// Indicates integer overflow.
  IntegerOverflow,
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IntegerOverflow => write!(f, "IntegerOverflow"),
    }
  }
}

impl std::error::Error for Error {}
