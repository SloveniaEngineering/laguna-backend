use serde::{Deserialize, Serialize};

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type, PartialOrd, Ord, Copy,
)]
pub enum Role {
  Normie = 0,
  Verified = 1,
  Mod = 2,
  Admin = 3,
}
