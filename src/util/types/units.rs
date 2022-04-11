use std::{fmt::Display, str::FromStr};

/// Represents the unit of an amount.
/// - `nRBTC` (nano)
/// - `uRBTC` (micro)
/// - `mRBTC` (milli)
/// - `RBTC` (unit)
/// - `kRBTC` (kilo)
/// - `MRBTC` (mega)
/// - `GRBTC` (giga)
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Unit {
  nRBTC,
  uRBTC,
  mRBTC,
  RBTC,
  kRBTC,
  MRBTC,
  GRBTC,
}

impl FromStr for Unit {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "nRBTC" => Ok(Unit::nRBTC),
      "uRBTC" => Ok(Unit::uRBTC),
      "mRBTC" => Ok(Unit::mRBTC),
      "RBTC" => Ok(Unit::RBTC),
      "kRBTC" => Ok(Unit::kRBTC),
      "MRBTC" => Ok(Unit::MRBTC),
      "GRBTC" => Ok(Unit::GRBTC),
      _ => Err(Error::InvalidUnit(s.to_string())),
    }
  }
}

/// The custom error type for a `Unit`.
#[derive(Debug)]
pub enum Error {
  /// Indicates an invalid unit.
  InvalidUnit(String),
}

impl Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::InvalidUnit(s) => write!(f, "Invalid unit: {}", s),
    }
  }
}

impl std::error::Error for Error {}
