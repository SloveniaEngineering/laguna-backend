use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash, sqlx::Type, ToSchema)]
pub enum Behaviour {
  Lurker,
  Downloader,
  Freeleecher,
  Leech,
  Seed,
  Choked,
  Uploader,
  Stopped,
}
