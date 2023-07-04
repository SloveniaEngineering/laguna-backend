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
pub enum  Behaviour {
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
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub first_login: DateTime<Utc>,
    /// UTC DateTime aka TIMESTAMP WITH TIME ZONE
    pub last_login: DateTime<Utc>,
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    /// If None, DEFAULT is CURRENT_TIMESTAMP
    pub first_login: Option<DateTime<Utc>>,
    /// If None, DEFAULT is CURRENT_TIMESTAMP
    pub last_login: Option<DateTime<Utc>>,
    pub avatar_url: Option<String>,
    pub role: Role,
    pub behaviour: Behaviour,
    /// If None, DEFAULT is true
    pub is_active: Option<bool>,
    /// If None, DEFAULT is false
    pub has_verified_email: Option<bool>,
    /// If None, DEFAULT is true
    pub is_history_private: Option<bool>,
    /// If None, DEFAULT is true
    pub is_profile_private: Option<bool>,
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            email: user.email,
            password: user.password,
            first_login: Some(user.first_login),
            last_login: Some(user.last_login),
            avatar_url: user.avatar_url,
            role: user.role,
            behaviour: user.behaviour,
            is_active: Some(user.is_active),
            has_verified_email: Some(user.has_verified_email),
            is_history_private: Some(user.is_history_private),
            is_profile_private: Some(user.is_profile_private),
        }
    }
}
