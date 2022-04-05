use crypto::{digest::Digest, ripemd160::Ripemd160, sha2::Sha256};

use crate::util::constants::{RIPEMD160_HASH_SIZE, SHA256_HASH_SIZE};

/// Return the RIPEMD-160 hash of the input bytes.
pub fn ripemd160(input: &[u8]) -> [u8; RIPEMD160_HASH_SIZE] {
  let mut hasher = Ripemd160::new();
  let mut output = [0u8; RIPEMD160_HASH_SIZE];
  hasher.input(input);
  hasher.result(&mut output);
  output
}

/// Return the SHA-256 hash of the input bytes.
pub fn sha256(input: &[u8]) -> [u8; SHA256_HASH_SIZE] {
  let mut hasher = Sha256::new();
  let mut output = [0u8; SHA256_HASH_SIZE];
  hasher.input(input);
  hasher.result(&mut output);
  output
}
