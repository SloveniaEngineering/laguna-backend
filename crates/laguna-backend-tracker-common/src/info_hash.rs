use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt;

use serde_with::Bytes;
use std::fmt::Formatter;

pub const SHA256_LENGTH: usize = 40;
pub const SHA1_LENGTH: usize = 20;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct InfoHash<const N: usize>(#[serde_as(as = "Bytes")] pub [u8; N]);

impl From<[u8; SHA1_LENGTH]> for InfoHash<SHA1_LENGTH> {
  fn from(value: [u8; SHA1_LENGTH]) -> Self {
    Self(value)
  }
}

impl<const N: usize> From<Vec<u8>> for InfoHash<N> {
  fn from(value: Vec<u8>) -> Self {
    Self(<[u8; N]>::try_from(value.as_slice()).unwrap())
  }
}

impl<const N: usize> fmt::Display for InfoHash<N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(
      self
        .0
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
        .as_str(),
    )
  }
}
