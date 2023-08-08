use crate::behaviour::Behaviour;

use crate::role::Role;
use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Utc};

use secrecy::Secret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// User DB object.
/// Not to be confused with [`UserDTO`] used for API.
#[derive(Serialize, Deserialize, Debug, Clone, FromRequest, sqlx::FromRow)]
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
    pub salt: String,
    pub role: Role,
    pub behaviour: Behaviour,
    pub is_active: bool,
    pub has_verified_email: bool,
    pub is_history_private: bool,
    pub is_profile_private: bool,
}

pub struct UserSafe {
    pub id: Uuid,
    pub username: String,
    pub email: Secret<String>,
    pub password: Secret<String>,
    pub first_login: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub avatar_url: Option<String>,
    pub salt: Secret<String>,
    pub role: Role,
    pub behaviour: Behaviour,
    pub is_active: bool,
    pub has_verified_email: bool,
    pub is_history_private: bool,
    pub is_profile_private: bool,
}

impl From<User> for UserSafe {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: Secret::new(user.email),
            password: Secret::new(user.password),
            first_login: user.first_login,
            last_login: user.last_login,
            avatar_url: user.avatar_url,
            salt: Secret::new(user.salt),
            role: user.role,
            behaviour: user.behaviour,
            is_active: user.is_active,
            has_verified_email: user.has_verified_email,
            is_history_private: user.is_history_private,
            is_profile_private: user.is_profile_private,
        }
    }
}
