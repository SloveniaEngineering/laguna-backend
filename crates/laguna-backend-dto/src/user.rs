use actix_jwt_auth_middleware::FromRequest;
use chrono::DateTime;
use chrono::Utc;
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::consts::{USERNAME_MAX_LEN, USERNAME_MIN_LEN};
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;
use laguna_backend_model::user::UserSafe;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// User data transfer object (DTO).
/// This object is serialized and transfered between BE and FE (in API).
/// Unlike [`User`], [`UserDTO`] doesn't expose the following fields:
/// 1. `email`
/// 2. `password`
/// Also, [`UserDTO`]has `last_login` as an [`Option`].
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, Validate)]
pub struct UserDTO {
  /// The user's id
  pub id: Uuid,
  #[validate(
    non_control_character,
    length(min = "USERNAME_MIN_LEN", max = "USERNAME_MAX_LEN")
  )]
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

impl From<UserSafe> for UserDTO {
  fn from(user_safe: UserSafe) -> Self {
    Self {
      id: user_safe.id,
      username: user_safe.username,
      first_login: user_safe.first_login,
      last_login: Some(user_safe.last_login),
      avatar_url: user_safe.avatar_url,
      role: user_safe.role,
      behaviour: user_safe.behaviour,
      is_active: Some(user_safe.is_active),
      has_verified_email: user_safe.has_verified_email,
      is_history_private: user_safe.is_history_private,
      is_profile_private: user_safe.is_profile_private,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest)]
pub struct UserPatchDTO {
  pub avatar_url: Option<String>,
  pub is_history_private: bool,
  pub is_profile_private: bool,
}
