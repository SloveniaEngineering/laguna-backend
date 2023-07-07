#![allow(unused)]
use actix_http::{body::MessageBody, Error, Request};
use actix_jwt_auth_middleware::{use_jwt::UseJWTOnApp, Authority, TokenSigner};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse},
    http::{header, StatusCode},
    middleware::Logger,
    test::{init_service, TestRequest},
    web, App, HttpRequest, HttpResponse, ResponseError,
};
use cookie::{Cookie, CookieJar};
use env_logger;
use jwt_compact::{
    alg::{Hs256, Hs256Key},
    TimeOptions,
};
use laguna_backend_api::{
    login::login,
    register::register,
    user::{get_me, get_one},
};
use laguna_backend_model::{login::LoginDTO, register::RegisterDTO, user::UserDTO};
use std::env;
use std::sync::Once;

use sqlx::{postgres::PgPoolOptions, PgPool};

// Initialize env_logger only once.
static ENV_LOGGER_SETUP: Once = Once::new();

pub(crate) async fn setup() -> (
    PgPool,
    impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
    ENV_LOGGER_SETUP.call_once(|| {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    });

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("Unable to connect to test database");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Couldn't run migrations");

    let key = Hs256Key::new("some random test shit");
    let authority = Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .token_signer(Some(
            TokenSigner::new()
                .signing_key(key.clone())
                .algorithm(Hs256)
                .build()
                .expect("Cannot create token signer"),
        ))
        .verifying_key(key.clone())
        .build()
        .expect("Cannot create key authority");
    let app = init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/user/auth")
                    .service(register)
                    .service(login),
            )
            .use_jwt(
                authority,
                web::scope("/api").service(web::scope("/user").service(get_me).service(get_one)),
            )
            .default_service(web::to(|| HttpResponse::NotFound())),
    )
    .await;

    (pool, app)
}

pub(crate) async fn teardown(pool: PgPool) {
    sqlx::query("DELETE FROM \"User\"")
        .execute(&pool)
        .await
        .expect("Failed to cleanup \"User\" table");
    pool.close().await;
}

pub(crate) async fn register_new_user(
    register_dto: RegisterDTO,
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> Result<ServiceResponse, actix_web::Error> {
    let req = TestRequest::post()
        .set_json(register_dto)
        .uri("/api/user/auth/register");
    app.call(req.to_request()).await
}

pub(crate) async fn login_new_user(
    login_dto: LoginDTO,
    app: &impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> Result<ServiceResponse, actix_web::Error> {
    let req = TestRequest::post()
        .set_json(login_dto)
        .uri("/api/user/auth/login");
    app.call(req.to_request()).await
}

pub(crate) async fn register_and_login_new_user(
    register_dto: RegisterDTO,
    login_dto: LoginDTO,
    app: &impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
    let _ = register_new_user(register_dto, app).await.unwrap();
    let res = login_new_user(login_dto, app).await.unwrap();
    res
}

pub(crate) async fn request_with_jwt_cookies_set(
    login_res: &ServiceResponse,
    mut req: TestRequest,
    app: &impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
    for cookie in extract_cookies(login_res).await {
        req = req.cookie(cookie);
    }
    app.call(req.to_request()).await.unwrap()
}

pub(crate) async fn extract_cookies<'a>(res: &'a ServiceResponse) -> Vec<Cookie<'a>> {
    let mut cookies = res.headers().get_all(header::SET_COOKIE);
    let access_token = cookies.next().unwrap().to_str().unwrap();
    let refresh_token = cookies.next().unwrap().to_str().unwrap();

    vec![
        Cookie::parse(access_token).unwrap(),
        Cookie::parse(refresh_token).unwrap(),
    ]
}
