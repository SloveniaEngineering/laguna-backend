#[cfg(feature = "testx")]
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::Type)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub enum SpeedLevel {
  Lowspeed,
  Mediumspeed,
  Highspeed,
}
