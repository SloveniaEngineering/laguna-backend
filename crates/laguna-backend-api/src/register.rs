use actix_jwt_auth_middleware::TokenSigner;
use actix_web::post;
use actix_web::{web, HttpResponse};
use digest::Digest;
use jwt_compact::alg::Hs256;
use laguna_backend_model::user::User;
use sha2::Sha256;
use sqlx::PgPool;

use crate::error::{APIError, UserError};

#[post("/register")]
pub async fn register(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
    cookie_signer: web::Data<TokenSigner<User, Hs256>>,
) -> Result<HttpResponse, APIError> {
    let fetched_user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE id = $1")
        .bind(user.id)
        .fetch_optional(pool.get_ref())
        .await?;

    match fetched_user {
        Some(registered_user) => Ok(HttpResponse::Ok().json(UserError::AlreadyRegistered {
            user: registered_user,
        })),
        None => {
            // TODO: Verify email
            sqlx::query(
                r#"
                INSERT INTO "User" (username, email, password)
                VALUES ($1, $2, $3);
            "#,
            )
            .bind(&user.username)
            .bind(&user.email)
            .bind(format!("{:x}", Sha256::digest(&user.password)))
            .execute(pool.get_ref())
            .await?;
            Ok(HttpResponse::Ok()
                .cookie(cookie_signer.create_access_cookie(&user)?)
                .cookie(cookie_signer.create_refresh_cookie(&user)?)
                .body(format!("You are now registered as {}", user.username)))
        }
    }
}
