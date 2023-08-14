use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum Behaviour {
  Lurker,
  Downloader,
  Freeleecher,
  Leech,
  Seed,
  Choked,
  Uploader,
}
