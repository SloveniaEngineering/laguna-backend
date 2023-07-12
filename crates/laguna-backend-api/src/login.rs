use actix_jwt_auth_middleware::TokenSigner;
use actix_web::post;
use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use chrono::Utc;
use digest::Digest;
use jwt_compact::alg::Hs256;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::login::LoginDTO;
use laguna_backend_model::user::{User, UserDTO};

use sha2::Sha256;
use sqlx::PgPool;

use crate::error::APIError;

/// `POST /api/user/auth/login`
/// # Example
/// ### Request
/// ```sh
/// curl -X POST \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/auth/login' \
///      --data '{
///         "username_or_email": "test",
///         "password": "test123"
///      }'
/// ```
/// ### Response
/// HTTP/1.1 200 OK
///
/// Headers:
/// ```text
/// X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0Njc1OTksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.jAQEpr_tjKc_j-asnoIBEhT8xmhBHXPjYygtwNfb76w; Secure
/// X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4; Secure
/// ```
/// ```json
/// {
///     "LoginSuccess": {
///         "user": {
///             "id": "b33b630d-e098-47d0-bc21-94c6a7467f17"
///             "username": "test",
///             "email": "test@laguna.io",
///             "first_login": "2023-07-04T10:18:17.391698Z",
///             "last_login": "2023-07-04T10:18:17.391698Z",
///             "avatar_url": null,
///             "role": "Normie",
///             "is_active": true,
///             "has_verified_email": false,
///             "is_history_private": true,
///             "is_profile_private": true
///         }
///     }
/// }
/// ```
/// ### Response
/// 1. On successful login: HTTP/1.1 200 OK
/// 2. On invalid password/email/username: HTTP/1.1 401 Unauthorized
/// 3. On invalid format (ie. too long, too short, not email, etc.): HTTP/1.1 400 Bad Request
#[post("/login")]
pub async fn login(
    login_dto: Json<LoginDTO>,
    pool: web::Data<PgPool>,
    signer: web::Data<TokenSigner<UserDTO, Hs256>>,
) -> Result<HttpResponse, APIError> {
    let fetched_user =
        sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1 OR email = $1")
            .bind(&login_dto.username_or_email)
            .fetch_optional(pool.get_ref())
            .await?;

    if let Some(logged_user) = fetched_user {
        if logged_user.password == format!("{:x}", Sha256::digest(&login_dto.password)) {
            // Logged user has been updated, we need to return the updated user.
            let user = sqlx::query_as::<_, User>(
                "UPDATE \"User\" SET last_login = $1 WHERE id = $2 RETURNING *",
            )
            .bind(Utc::now())
            .bind(logged_user.id)
            .fetch_one(pool.get_ref())
            .await?;
            return Ok(HttpResponse::Ok()
                // TODO: get rid of clones
                .append_header((
                    ACCESS_TOKEN_HEADER_NAME,
                    signer.create_access_header_value(&user.clone().into())?,
                ))
                .append_header((
                    REFRESH_TOKEN_HEADER_NAME,
                    signer.create_refresh_header_value(&user.clone().into())?,
                ))
                .json(UserDTO::from(user)));
        }
    }

    // SECURITY: Don't report "Password" or "Username" invalid to avoid brute-force attacks.
    Ok(HttpResponse::Unauthorized().finish())
}
