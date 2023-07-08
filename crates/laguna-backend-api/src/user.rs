use actix_web::{delete, get, web, HttpResponse};
use laguna_backend_model::user::UserDTO;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{APIError, UserError},
    state::UserState,
};

/// `GET /api/user/me`
/// # Example
/// ## Request
/// ## Response
#[get("/me")]
pub async fn get_me(user: UserDTO) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().json(user))
}

/// `GET /api/user/{id}`
/// # Example
/// ## Request
/// ## Response
#[get("/{id}")]
pub async fn get_one(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().json(
        sqlx::query_as::<_, UserDTO>("SELECT * FROM \"User\" WHERE id = $1")
            .bind(*id)
            .fetch_optional(pool.get_ref())
            .await?
            .ok_or_else(|| UserError::DoesNotExist)?,
    ))
}

/// `DELETE /api/user/delete/me`
#[delete("/me")]
pub async fn delete_me(user: UserDTO, pool: web::Data<PgPool>) -> Result<HttpResponse, APIError> {
    sqlx::query("DELETE FROM \"User\" WHERE id = $1")
        .bind(user.id)
        .execute(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(UserState::DeleteSuccess))
}

/// `DELETE /api/user/delete/{id}`
#[delete("/{id}")]
pub async fn delete_one(
    id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, APIError> {
    sqlx::query("DELETE FROM \"User\" WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(UserState::DeleteSuccess))
}
