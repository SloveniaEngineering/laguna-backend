use actix_jwt_auth_middleware::AuthError;
use actix_web::error::ResponseError;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;

#[derive(Debug, Display)]
pub enum APIError {
    SqlxError(SqlxError),
    AuthError(AuthError),
    UserError(UserError),
    LoginError(LoginError),
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

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum UserError {}

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum LoginError {
    InvalidCredentials,
}

impl From<UserError> for APIError {
    fn from(value: UserError) -> Self {
        Self::UserError(value)
    }
}

impl From<LoginError> for APIError {
    fn from(value: LoginError) -> Self {
        Self::LoginError(value)
    }
}
