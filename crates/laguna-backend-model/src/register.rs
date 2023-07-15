use crate::consts::{EMAIL_MAX_LEN, EMAIL_MIN_LEN};
use crate::consts::{PASSWORD_MAX_LEN, PASSWORD_MIN_LEN};
use crate::consts::{USERNAME_MAX_LEN, USERNAME_MIN_LEN};
use fake::faker::internet::en::FreeEmail;
use fake::faker::internet::en::Password;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Data transfer object (DTO) used for registering.
/// This object is serialized and transfered from the frontend to the backend to register a new user.
#[derive(Serialize, Deserialize, Debug, Dummy, Clone, Validate)]
pub struct RegisterDTO {
    #[validate(
        non_control_character,
        length(min = "USERNAME_MIN_LEN", max = "USERNAME_MAX_LEN")
    )]
    #[dummy(faker = "USERNAME_MIN_LEN..USERNAME_MAX_LEN")]
    pub username: String,
    #[validate(
        non_control_character,
        email,
        length(min = "EMAIL_MIN_LEN", max = "EMAIL_MAX_LEN")
    )]
    // TODO: This is not guaranteed to be in limits.
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    #[validate(
        non_control_character,
        length(min = "PASSWORD_MIN_LEN", max = "PASSWORD_MAX_LEN")
    )]
    #[dummy(faker = "Password(PASSWORD_MIN_LEN..PASSWORD_MAX_LEN)")]
    pub password: String,
}
