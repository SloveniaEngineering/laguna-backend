use actix_jwt_auth_middleware::FromRequest;
use chrono::DateTime;
use chrono::Utc;
use laguna_backend_model::behaviour::Behaviour;
use laguna_backend_model::consts::{USERNAME_MAX_LEN, USERNAME_MIN_LEN};
use laguna_backend_model::role::Role;
use laguna_backend_model::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// User data-transfer object.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, Validate, ToSchema)]
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
  pub is_enabled: bool,
  pub is_donator: bool,
  pub has_verified_email: bool,
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
      is_enabled: user.is_enabled,
      is_donator: user.is_donator,
      has_verified_email: user.has_verified_email,
      is_profile_private: user.is_profile_private,
    }
  }
}

/// Patch non-sensitive user data-transfer object.
/// When patching with this DTO, user does not have to go through additional verification.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, ToSchema)]
pub struct UserPatchDTO {
  /// [`None`] means URL is deleted.
  pub avatar_url: Option<String>,
  /// Set if user's profile is private.
  pub is_profile_private: bool,
}

/// Patch user's password DTO.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, ToSchema)]
pub struct UserPasswordPatchDTO {
  /// User's new password.
  pub password: String,
}

/// Patch user's username DTO.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromRequest, Validate, ToSchema)]
pub struct UserUsernamePatchDTO {
  /// User's new username.
  #[validate(
    non_control_character,
    length(min = "USERNAME_MIN_LEN", max = "USERNAME_MAX_LEN")
  )]
  pub username: String,
}
