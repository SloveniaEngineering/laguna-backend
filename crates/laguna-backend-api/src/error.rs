use actix_jwt_auth_middleware::AuthError;
use actix_web::error::ResponseError;
use derive_more::Display;
use laguna_backend_model::user::User;
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;

#[derive(Debug, Display)]
pub enum APIError {
    SqlxError(SqlxError),
    AuthError(AuthError),
}

impl From<SqlxError> for APIError {
    fn from(value: SqlxError) -> Self {
        Self::SqlxError(value)
    }
}

impl From<AuthError> for APIError {
    fn from(value: AuthError) -> Self {
        Self::AuthError(value)
    }
}

impl ResponseError for APIError {}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserError {
    AlreadyRegistered { user: User },
}
