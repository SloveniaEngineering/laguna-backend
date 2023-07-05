use serde::{Deserialize, Serialize};

/// Data transfer object (DTO) used for registering.
/// This object is serialized and transfered from the frontend to the backend to register a new user.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
