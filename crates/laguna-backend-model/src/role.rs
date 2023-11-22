use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// [`Role`] of [`User`].
/// Higher the role more power.
/// [`Role::Verified`] can upload torrents, [`Role::Normie`] can only download and leech/seed.
/// [`Role::Mod`] manages [`Role::Normie`], [`Role::Verified`].
/// [`Role::Admin`] manages everything [`Role::Mod`] manages + [`Role::Mod`].
#[allow(missing_docs)]
#[derive(
  Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type, PartialOrd, Ord, Copy, ToSchema,
)]
pub enum Role {
  Normie = 0,
  Verified = 1,
  Mod = 2,
  Admin = 3,
}
