use serde::{Deserialize, Serialize};

/// A serializable RBTC transaction.
///
/// TODO: Make this actually contain data.
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
  pub x: String,
}
