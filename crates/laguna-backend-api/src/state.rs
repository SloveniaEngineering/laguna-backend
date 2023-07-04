use laguna_backend_model::user::UserDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserState {
    AlreadyRegistered { user: UserDTO },
    RegistrationSuccess,
    LoginSuccess { user: UserDTO },
}
