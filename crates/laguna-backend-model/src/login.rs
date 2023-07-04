use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    /// Plaintext password.
    pub password: String,
    /// Timestamp created by client (on click)
    /// Timestamp cannot be created on server because of latency.
    pub login_timestamp: DateTime<Utc>,
}
