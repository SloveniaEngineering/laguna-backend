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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum Behaviour {
    Lurker,
    Downloader,
    Freeleecher,
    Leech,
    Seed,
    Choked,
}

/// User DB object.
/// Not to be confused with [`UserDTO`] used for API.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, sqlx::FromRow)]
pub struct User {
    /// UUID generated using uuid_generate_v4() on DB
    pub id: Uuid,
    pub username: String,
    pub email: String,
    /// Hashed using SHA-256
    pub password: String,
    /// DEFAULT TIMESTAMP WITH TIME ZONE
    pub first_login: DateTime<Utc>,
    /// DEFAULT TIMESTAMP WITH TIME ZONE
    pub last_login: DateTime<Utc>,
    /// DEFAULT NULL
    pub avatar_url: Option<String>,
    pub role: Role,
    pub behaviour: Behaviour,
    pub is_active: bool,
    pub has_verified_email: bool,
    pub is_history_private: bool,
    pub is_profile_private: bool,
}

/// User data transfer object (DTO).
/// This object is serialized and transfered between BE and FE (in API).
/// Unlike [`User`], [`UserDTO`] doesn't expose the following fields:
/// 1. `email`
/// 2. `password`
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, sqlx::FromRow)]
pub struct UserDTO {
    /// The user's id
    pub id: Uuid,
    pub username: String,
    pub first_login: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub avatar_url: Option<String>,
    pub role: Role,
    pub behaviour: Behaviour,
    pub is_active: Option<bool>,
    pub has_verified_email: bool,
    pub is_history_private: bool,
    pub is_profile_private: bool,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_login: user.first_login,
            last_login: Some(user.last_login),
            avatar_url: user.avatar_url,
            role: user.role,
            behaviour: user.behaviour,
            is_active: Some(user.is_active),
            has_verified_email: user.has_verified_email,
            is_history_private: user.is_history_private,
            is_profile_private: user.is_profile_private,
        }
    }
}
