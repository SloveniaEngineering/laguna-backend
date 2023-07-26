use validator::{validate_email, validate_length, validate_non_control_character, ValidationError};

use crate::consts::{EMAIL_MAX_LEN, EMAIL_MIN_LEN, USERNAME_MAX_LEN, USERNAME_MIN_LEN};

pub fn validate_username_or_email(username_or_email: &str) -> Result<(), ValidationError> {
    if username_or_email.contains('@') {
        if !validate_length(
            username_or_email,
            Some(EMAIL_MIN_LEN as u64),
            Some(EMAIL_MAX_LEN as u64),
            None,
        ) {
            return Err(ValidationError::new("email"));
        }
        if !validate_non_control_character(username_or_email) {
            return Err(ValidationError::new("email"));
        }
        if !validate_email(username_or_email) {
            return Err(ValidationError::new("email"));
        }
        Ok(())
    } else {
        if !validate_length(
            username_or_email,
            Some(USERNAME_MIN_LEN as u64),
            Some(USERNAME_MAX_LEN as u64),
            None,
        ) {
            return Err(ValidationError::new("username"));
        }
        if !validate_non_control_character(username_or_email) {
            return Err(ValidationError::new("username"));
        }
        Ok(())
    }
}
