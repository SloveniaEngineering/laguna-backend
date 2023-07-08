use laguna_backend_model::user::UserDTO;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserState {
    AlreadyRegistered,
    RegisterSuccess,
    DeleteSuccess,
    LoginSuccess { user: UserDTO },
}
