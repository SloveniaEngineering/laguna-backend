use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    /// Plaintext password.
    pub password: String,
}
