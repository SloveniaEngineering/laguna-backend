use actix_web::{get, HttpResponse};
use laguna_backend_model::user::UserDTO;

use crate::error::APIError;

/// `GET /api/user/me`
/// # Example
/// ## Request
///
/// ## Response
///
#[get("/me")]
pub async fn me(user: UserDTO) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().json(user))
}
