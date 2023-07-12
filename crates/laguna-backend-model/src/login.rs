use serde::{Deserialize, Serialize};

use crate::consts::{PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};
use crate::register::RegisterDTO;
use crate::validators::login::validate_username_or_email;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
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
