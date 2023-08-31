use serde::{Deserialize, Serialize};

use serde_with::serde_as;
use std::fmt;

use serde_with::hex::Hex;
use std::fmt::Formatter;

pub const SHA1_LENGTH: usize = 20;
pub const SHA256_LENGTH: usize = 40;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct InfoHash<const N: usize>(#[serde_as(as = "Hex")] pub [u8; N]);

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
