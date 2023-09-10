#[cfg(feature = "testx")]
use fake::{
  faker::internet::en::{FreeEmail, Password},
  Dummy,
};
use laguna_backend_model::consts::{EMAIL_MAX_LEN, EMAIL_MIN_LEN};
use laguna_backend_model::consts::{PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};
use laguna_backend_model::consts::{USERNAME_MAX_LEN, USERNAME_MIN_LEN};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Data transfer object (DTO) used for registering.
/// This object is serialized and transfered from the frontend to the backend to register a new user.
#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[cfg_attr(feature = "testx", derive(Dummy))]
pub struct RegisterDTO {
  #[validate(
    non_control_character,
    length(min = "USERNAME_MIN_LEN", max = "USERNAME_MAX_LEN")
  )]
  #[cfg_attr(feature = "testx", dummy(faker = "USERNAME_MIN_LEN..USERNAME_MAX_LEN"))]
  pub username: String,
  #[validate(
    non_control_character,
    email,
    length(min = "EMAIL_MIN_LEN", max = "EMAIL_MAX_LEN")
  )]
  // TODO: This is not guaranteed to be in limits.
  #[cfg_attr(feature = "testx", dummy(faker = "FreeEmail()"))]
  pub email: String,
  #[validate(
    non_control_character,
    length(min = "PASSWORD_MIN_LEN", max = "PASSWORD_MAX_LEN")
  )]
  #[cfg_attr(
    feature = "testx",
    dummy(faker = "Password(PASSWORD_MIN_LEN..PASSWORD_MAX_LEN)")
  )]
  pub password: String,
}
