#![allow(unused)]
use actix_http::header::HeaderValue;
use actix_http::{body::MessageBody, Error, Request};
use actix_jwt_auth_middleware::{use_jwt::UseJWTOnApp, Authority, TokenSigner};
use actix_web::cookie::Cookie;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse},
    http::{header, StatusCode},
    middleware::Logger,
    test::{init_service, TestRequest},
    web, App, HttpRequest, HttpResponse, ResponseError,
};
use chrono::Duration;
use env_logger;
use jwt_compact::{
    alg::{Hs256, Hs256Key},
    TimeOptions,
};
use laguna_backend_api::torrent::{
    get_torrent, get_torrent_download, get_torrent_with_info_hash, get_torrents_with_filter,
    put_torrent,
};
use laguna_backend_api::{
    login::login,
    register::register,
    user::{delete_me, delete_user, get_me, get_user},
};
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
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
                            .service(get_torrent_with_info_hash)
                            .service(get_torrents_with_filter)
                            .service(web::scope("/download").service(get_torrent_download))
                            .service(web::scope("/upload").service(put_torrent)),
                    ),
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

pub(crate) async fn jwt_from_response(res: &ServiceResponse) -> (&HeaderValue, &HeaderValue) {
    let access_token = res.headers().get(ACCESS_TOKEN_HEADER_NAME).unwrap();
    let refresh_token = res.headers().get(REFRESH_TOKEN_HEADER_NAME).unwrap();
    (access_token, refresh_token)
}

pub(crate) async fn request_with_jwt(
    login_res: &ServiceResponse,
    mut req: TestRequest,
    app: &impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> ServiceResponse {
    let (access_token, refresh_token) = jwt_from_response(login_res).await;
    req = req
        .append_header((ACCESS_TOKEN_HEADER_NAME, access_token))
        .append_header((REFRESH_TOKEN_HEADER_NAME, refresh_token));
    app.call(req.to_request()).await.unwrap()
}
