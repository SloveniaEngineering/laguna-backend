#[cfg(feature = "testx")]
use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type, ToSchema)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub enum SpeedLevel {
  Lowspeed,
  Mediumspeed,
  Highspeed,
}
