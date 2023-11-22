use crate::behaviour::Behaviour;

use crate::role::Role;
use actix_jwt_auth_middleware::FromRequest;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User DB object.
#[derive(Serialize, Deserialize, Debug, Clone, FromRequest, FromRow)]
pub struct User {
  /// Unique ID for user.
  /// UUID generated using uuid_generate_v4() on DB.
  pub id: Uuid,
  /// User's username.
  pub username: String,
  /// User's email.
  pub email: String,
  /// User's password.
  /// Hashed using SHA-256.
  pub password: String,
  /// First login timestamp (time at registration).
  /// In DB: `DEFAULT TIMESTAMP WITH TIME ZONE`
  pub first_login: DateTime<Utc>,
  /// Last login timestamp.
  /// In DB: `DEFAULT TIMESTAMP WITH TIME ZONE`.
  pub last_login: DateTime<Utc>,
  /// URL to avatar, must be image (size limit will be determined).
  /// In DB `DEFAULT NULL`.
  pub avatar_url: Option<String>,
  /// Random number use in combination with pepper inside Argon2 context to hash the password.
  pub salt: String,
  /// Role of user.
  pub role: Role,
  /// Hit & Run count.
  pub hnr_count: i32,
  /// Overall Behaviour of user, this is computed based on all of his peers.
  /// For example, if user only downloads but never seeds, the overall behaviour is [`Behaviour::Leech`].
  /// This is different from specific behaviour in [`crate::peer::Peer`] structure.
  pub behaviour: Behaviour,
  /// Is user enabled? If not enabled, user has been banned permanently.
  pub is_enabled: bool,
  /// Is user donator?
  pub is_donator: bool,
  /// Has user verified email?
  pub has_verified_email: bool,
  /// Does user wish his profile to be private (visible only to [`Role::Mod`] and [`Role::Admin`] users)?
  pub is_profile_private: bool,
}
