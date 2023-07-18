#![allow(unused)]
use actix_http::header::HeaderValue;
use actix_http::{body::MessageBody, Error, Request};
use actix_jwt_auth_middleware::{use_jwt::UseJWTOnApp, Authority, TokenSigner};
use actix_web::cookie::Cookie;
use actix_web::dev::{AppConfig, AppService};
use actix_web::test::read_body_json;
use actix_web::web::ServiceConfig;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse},
    http::{header, StatusCode},
    middleware::Logger,
    test::{init_service, TestRequest},
    web, App, HttpRequest, HttpResponse, ResponseError,
};
use chrono::Duration;
use env_logger;
use fake::{Fake, Faker};
use jwt_compact::{
    alg::{Hs256, Hs256Key},
    TimeOptions,
};
use laguna_backend_api::error::APIError;
use laguna_backend_api::torrent::{get_torrent, patch_torrent, put_torrent};
use laguna_backend_api::{
    login::login,
    register::register,
    user::{delete_me, delete_user, get_me, get_user},
};
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::{login::LoginDTO, register::RegisterDTO, user::UserDTO};
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;
use std::sync::{Arc, Once};
use uuid::Uuid;

use sqlx::{postgres::PgPoolOptions, PgPool};

// Initialize env_logger only once.
static ENV_LOGGER_SETUP: Once = Once::new();

pub(crate) async fn setup() -> (
    PgPool,
    String,
    impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
    ENV_LOGGER_SETUP.call_once(|| {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    });

    let database_uuid = Uuid::new_v4().to_string();
    let database_url = format!(
        "postgres://postgres:postgres@localhost:5432/{}_laguna_test_db",
        database_uuid
    );

    let database_create_command = Command::new("sqlx")
        .args(&[
            "database",
            "reset",
            &format!("--database-url={}", database_url),
            "-y",
            "--source=../../migrations",
        ])
        .status()
        .expect("sqlx database reset command failed");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Unable to connect to test database");

    let key = Hs256Key::new("some random test shit");
    let authority = Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .enable_header_tokens(true)
        .access_token_name(ACCESS_TOKEN_HEADER_NAME)
        .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
        .token_signer(Some(
            TokenSigner::new()
                .signing_key(key.clone())
                .algorithm(Hs256)
                .time_options(TimeOptions::from_leeway(Duration::days(1)))
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
                web::scope("/api")
                    .service(
                        web::scope("/user")
                            .service(get_me)
                            .service(get_user)
                            .service(
                                web::scope("/delete")
                                    .service(delete_me)
                                    .service(delete_user),
                            ),
                    )
                    .service(
                        web::scope("/torrent")
                            .service(get_torrent)
                            .service(patch_torrent)
                            .service(put_torrent),
                    ),
            )
            .default_service(web::to(|| HttpResponse::NotFound())),
    )
    .await;

    (pool, database_url, Box::new(app))
}

pub(crate) async fn teardown(pool: PgPool, database_url: String) {
    pool.close().await;
    let database_drop_command = Command::new("sqlx")
        .args(&[
            "database",
            "drop",
            &format!("--database-url={}", database_url),
            "-y",
        ])
        .status()
        .expect("sqlx database drop command failed");
}

/// It is guaranteed that the user will be registered and logged in successfully or fail the test.
/// Returns the access token and refresh token.
pub(crate) async fn new_user(
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    new_user_with(Faker.fake::<RegisterDTO>(), &app).await
}

pub(crate) async fn new_user_with(
    register_dto: RegisterDTO,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    register_user_safe(register_dto.clone(), &app).await;
    let (user_dto, access_token, refresh_token) =
        login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
    (register_dto, user_dto, access_token, refresh_token)
}

pub(crate) async fn register_user_safe(
    register_dto: RegisterDTO,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
    assert_eq!(
        register_user(register_dto, &app).await.status(),
        StatusCode::OK
    )
}

pub(crate) async fn login_user_safe(
    login_dto: LoginDTO,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (UserDTO, HeaderValue, HeaderValue) {
    let res = login_user(login_dto, &app).await;
    assert_eq!(res.status(), StatusCode::OK);
    let access_token = res
        .headers()
        .get(ACCESS_TOKEN_HEADER_NAME)
        .unwrap()
        .to_owned();
    let refresh_token = res
        .headers()
        .get(REFRESH_TOKEN_HEADER_NAME)
        .unwrap()
        .to_owned();
    let user_dto = read_body_json::<UserDTO, _>(res).await;
    (user_dto, access_token, refresh_token)
}

pub(crate) async fn register_user(
    register_dto: RegisterDTO,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
    app.call(
        TestRequest::post()
            .uri("/api/user/auth/register")
            .set_json(register_dto)
            .to_request(),
    )
    .await
    .unwrap()
}

pub(crate) async fn login_user(
    login_dto: LoginDTO,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
    app.call(
        TestRequest::post()
            .uri("/api/user/auth/login")
            .set_json(login_dto)
            .to_request(),
    )
    .await
    .unwrap()
}

pub(crate) async fn as_logged_in(
    access_token: HeaderValue,
    refresh_token: HeaderValue,
    mut req: TestRequest,
    app: &impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> Result<ServiceResponse, actix_web::Error> {
    req = req
        .append_header((ACCESS_TOKEN_HEADER_NAME, access_token))
        .append_header((REFRESH_TOKEN_HEADER_NAME, refresh_token));
    app.call(req.to_request()).await
}
