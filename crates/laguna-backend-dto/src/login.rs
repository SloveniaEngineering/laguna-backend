use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::register::RegisterDTO;
use crate::validators::login::validate_username_or_email;
use laguna_backend_model::consts::{PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};
use validator::Validate;

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginDTO {
  #[validate(non_control_character, custom = "validate_username_or_email")]
  pub username_or_email: String,
  /// Plaintext password.
  #[validate(
    non_control_character,
    length(min = "PASSWORD_MIN_LEN", max = "PASSWORD_MAX_LEN")
  )]
  pub password: String,
}

impl From<RegisterDTO> for LoginDTO {
  fn from(register_dto: RegisterDTO) -> Self {
    Self {
      username_or_email: register_dto.email,
      password: register_dto.password,
    }
  }
}
