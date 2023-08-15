use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt;
use std::fmt::Formatter;

pub const SHA256_LENGTH: usize = 40;
pub const SHA1_LENGTH: usize = 20;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct InfoHash(#[serde_as(as = "[_; SHA1_LENGTH]")] pub [u8; SHA1_LENGTH]);

impl From<[u8; SHA1_LENGTH]> for InfoHash {
  fn from(value: [u8; SHA1_LENGTH]) -> Self {
    Self(value)
  }
}

impl From<Vec<u8>> for InfoHash {
  fn from(value: Vec<u8>) -> Self {
    Self(<[u8; SHA1_LENGTH]>::try_from(value.as_slice()).unwrap())
  }
}

impl fmt::Display for InfoHash {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("{:x?}", self.0))
  }
}
