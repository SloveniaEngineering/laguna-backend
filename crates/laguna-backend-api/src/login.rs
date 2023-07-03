use actix_jwt_auth_middleware::{AuthResult, TokenSigner};
use actix_web::post;
use actix_web::{web, HttpResponse};
use jwt_compact::alg::Hs256;
use laguna_backend_model::user::User;

#[post("/login")]
pub async fn login(
    user: web::Json<User>,
    cookie_signer: web::Data<TokenSigner<User, Hs256>>,
) -> AuthResult<HttpResponse> {
    // TODOs:
    // 1. Check if user exists
    // 2. Log user in
    todo!()
}
