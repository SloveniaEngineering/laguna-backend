use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum Role {
    Normie,
    Verified,
    Mod,
    Admin,
}
