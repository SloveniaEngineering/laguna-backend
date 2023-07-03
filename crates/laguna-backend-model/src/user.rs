use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum Role {
    Normie,
    Verified,
    Mod,
    Admin,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, sqlx::FromRow)]
pub struct User {
    /// Generated using uuid_generate_v4()
    pub id: Uuid,
    pub username: String,
    pub email: String,
    /// Hashed using SHA-256
    pub password: String,
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub first_login: DateTime<Utc>,
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub last_login: DateTime<Utc>,
    pub avatar_url: Option<String>,
    pub role: Role,
    pub is_active: bool,
    pub has_verified_email: bool,
    pub is_history_private: bool,
    pub is_profile_private: bool,
}
