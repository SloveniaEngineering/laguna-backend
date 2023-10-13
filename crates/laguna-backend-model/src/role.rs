use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type, PartialOrd, Ord, Copy, ToSchema,
)]
pub enum Role {
  Normie = 0,
  Verified = 1,
  Mod = 2,
  Admin = 3,
}
