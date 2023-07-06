use laguna_backend_model::user::UserDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserState {
    AlreadyRegistered,
    RegistrationSuccess,
    LoginSuccess { user: UserDTO },
}
