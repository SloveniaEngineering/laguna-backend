use crate::behaviour::Behaviour;

use crate::role::Role;
use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Utc};

use secrecy::Secret;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
/// User DB object.
#[derive(Serialize, Deserialize, Debug, Clone, FromRequest, FromRow)]
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
  pub hnr_count: i32,
  pub behaviour: Behaviour,
  pub is_enabled: bool,
  pub is_donator: bool,
  pub has_verified_email: bool,
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
  pub hnr_count: i32,
  pub behaviour: Behaviour,
  pub is_enabled: bool,
  pub is_donator: bool,
  pub has_verified_email: bool,
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
      hnr_count: user.hnr_count,
      behaviour: user.behaviour,
      is_enabled: user.is_enabled,
      is_donator: user.is_donator,
      has_verified_email: user.has_verified_email,
      is_profile_private: user.is_profile_private,
    }
  }
}
