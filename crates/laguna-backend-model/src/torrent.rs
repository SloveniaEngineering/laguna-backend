use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::User;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Torrent {
    pub id: Uuid,
    pub name: String,
    pub file_name: String,
    pub nfo: Option<String>,
    pub path: PathBuf,
    pub info_hash: String,
    pub uploaded_by: User,
    pub modded_by: Option<User>,
}
