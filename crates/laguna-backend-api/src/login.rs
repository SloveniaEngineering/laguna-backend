use actix_jwt_auth_middleware::TokenSigner;

use actix_web::{web, HttpResponse};
use actix_web_validator::Json;
use argon2::{Algorithm, Argon2, ParamsBuilder, PasswordHash, PasswordVerifier, Version};
use chrono::Utc;
use jwt_compact::alg::Hs256;
use laguna_backend_dto::login::LoginDTO;
use laguna_backend_dto::user::UserDTO;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::user::{User, UserSafe};

use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::error::{user::UserError, APIError};

/// `POST /api/user/auth/login`
/// Signs in existing user.
/// For registering see [`register`](crate::login::login).
/// # Example
/// ### Request
/// ```sh
/// curl -X POST \
///      -H 'Content-Type: application/json' \
///      -i 'http://127.0.0.1:6969/api/user/auth/login' \
///      --data '{
///         "username_or_email": "test123",
///         "password": "test123"
///      }'
/// ```
/// ### Response
/// #### Headers
/// ```text
/// X-Access-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0Njc1OTksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.jAQEpr_tjKc_j-asnoIBEhT8xmhBHXPjYygtwNfb76w; Secure
/// X-Refresh-Token: eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODg0NjkzMzksImlhdCI6MTY4ODQ2NzUzOSwidXNlcm5hbWUiOiJ0ZXN0IiwiZW1haWwiOiJ0ZXN0QGxhZ3VuYS5pbyIsInBhc3N3b3JkIjoiZWNkNzE4NzBkMTk2MzMxNmE5N2UzYWMzNDA4Yzk4MzVhZDhjZjBmM2MxYmM3MDM1MjdjMzAyNjU1MzRmNzVhZSIsImZpcnN0X2xvZ2luIjoiMjAyMy0wNy0wNFQxMDoxODoxNy4zOTE2OThaIiwibGFzdF9sb2dpbiI6IjIwMjMtMDctMDRUMTA6MTg6MTcuMzkxNjk4WiIsImF2YXRhcl91cmwiOm51bGwsInJvbGUiOiJOb3JtaWUiLCJpc19hY3RpdmUiOnRydWUsImhhc192ZXJpZmllZF9lbWFpbCI6ZmFsc2UsImlzX2hpc3RvcnlfcHJpdmF0ZSI6dHJ1ZSwiaXNfcHJvZmlsZV9wcml2YXRlIjp0cnVlfQ.5fdMnIj0WqV0lszANlJD_x5-Oyq2h8bhqDkllz1CGg4; Secure
/// ```
/// #### Body
/// ```json
/// {
///   "id": "b33b630d-e098-47d0-bc21-94c6a7467f17"
///   "username": "test123",
///   "email": "test123@laguna.io",
///   "first_login": "2023-07-04T10:18:17.391698Z",
///   "last_login": "2023-07-04T10:18:17.391698Z",
///   "avatar_url": null,
///   "role": "Normie",
///   "is_active": true,
///   "has_verified_email": false,
///   "is_history_private": true,
///   "is_profile_private": true
/// }
/// ```
/// #### Status Code
/// |Code|Description|
/// |---|---|
/// |200 OK|Successful login. Returns [`UserDTO`] + tokens|
/// |400 Bad Request|Last login didnt due to invalid input data|
/// |401 Unauthorized|Invalid password/email/username|
pub async fn login(
    login_dto: Json<LoginDTO>,
    pool: web::Data<PgPool>,
    signer: web::Data<TokenSigner<UserDTO, Hs256>>,
) -> Result<HttpResponse, APIError> {
    let fetched_user = sqlx::query_as!(
        User,
        r#"
        SELECT id, 
               username, 
               email, 
               password, 
               first_login, 
               last_login, 
               avatar_url,
               salt,
               role AS "role: _",
               behaviour AS "behaviour: _",
               is_active,
               has_verified_email,
               is_history_private,
               is_profile_private
        FROM "User" WHERE username = $1 OR email = $1
        "#,
        &login_dto.username_or_email
    )
    .fetch_optional(pool.get_ref())
    .await?
    .map(UserSafe::from);

    if let Some(logged_user) = fetched_user {
        let argon_context = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            ParamsBuilder::new()
                .p_cost(1)
                .m_cost(12288)
                .t_cost(3)
                .build()
                .unwrap(),
        );
        let password_hash = PasswordHash::new(logged_user.password.expose_secret()).unwrap();
        if let Ok(_) = argon_context.verify_password(login_dto.password.as_bytes(), &password_hash)
        {
            // Logged user has been updated, we need to return the updated user.
            let user = sqlx::query_as!(
                User,
                r#"
                UPDATE "User"
                SET last_login = $1
                WHERE id = $2
                RETURNING id, 
                          username, 
                          email, 
                          password, 
                          first_login, 
                          last_login, 
                          avatar_url,
                          salt,
                          role AS "role: _",
                          behaviour AS "behaviour: _",
                          is_active,
                          has_verified_email,
                          is_history_private,
                          is_profile_private
            "#,
                Utc::now(),
                logged_user.id
            )
            .fetch_optional(pool.get_ref())
            .await?
            .map(UserSafe::from)
            .map(UserDTO::from)
            .ok_or_else(|| UserError::DidntUpdate)?;
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
                .json(user));
        }
    }

    // SECURITY: Don't report "Password" or "Username" invalid to avoid brute-force attacks.
    Ok(HttpResponse::Unauthorized().finish())
}
