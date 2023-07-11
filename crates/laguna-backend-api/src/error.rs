use std::io;

use actix_jwt_auth_middleware::AuthError;
use actix_web::error::ResponseError;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SqlxError;

#[derive(Debug, Display)]
pub enum APIError {
    SqlxError(SqlxError),
    IOError(io::Error),
    AuthError(AuthError),
    UserError(UserError),
    TorrentError(TorrentError),
    LoginError(LoginError),
}

impl From<io::Error> for APIError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
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

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum UserError {
    DoesNotExist,
}

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum TorrentError {
    DoesNotExist,
    MissingFileName,
}

impl From<TorrentError> for APIError {
    fn from(value: TorrentError) -> Self {
        Self::TorrentError(value)
    }
}
