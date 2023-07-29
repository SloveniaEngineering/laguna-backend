use crate::behaviour::Behaviour;

use crate::role::Role;
use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
