use actix_http::header::HeaderValue;
use actix_http::Request;
use actix_jwt_auth_middleware::AuthenticationService;
use actix_jwt_auth_middleware::{Authority, TokenSigner};

use actix_web::test::read_body_json;
use actix_web::web::ServiceConfig;
use actix_web::{
    dev::{self, Service, ServiceResponse},
    http::StatusCode,
    test::{init_service, TestRequest},
    web, App, HttpResponse,
};
use chrono::Duration;
use env_logger;
use fake::{Fake, Faker};
use jwt_compact::alg::{Hs256, Hs256Key};
use jwt_compact::TimeOptions;

use laguna_backend_api::misc::get_app_info;
use laguna_backend_api::torrent::{torrent_get, torrent_patch, torrent_put};
use laguna_backend_api::user::{user_patch, user_peers_get};
use laguna_backend_api::{
    login::login,
    register::register,
    user::{user_delete, user_get, user_me_delete, user_me_get},
};

use laguna_backend_dto::user::UserDTO;
use laguna_backend_dto::{login::LoginDTO, register::RegisterDTO};
use laguna_backend_middleware::auth::AuthorizationMiddlewareFactory;
use laguna_backend_middleware::consts::{ACCESS_TOKEN_HEADER_NAME, REFRESH_TOKEN_HEADER_NAME};
use laguna_backend_model::role::Role;

use laguna_config::CONFIG_DEV;
use laguna_config::{Settings, MIGRATIONS_DIR};

use std::process::Command;
use std::sync::Once;
use uuid::Uuid;

use sqlx::{postgres::PgPoolOptions, PgPool};

static ENV_LOGGER_INIT: Once = Once::new();

pub(crate) fn get_dev_settings() -> Settings {
    Settings::parse_toml(CONFIG_DEV).expect("Failed to parse settings")
}

pub(crate) async fn setup() -> (
    PgPool,
    String,
    impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
    setup_with_settings(get_dev_settings()).await
}

pub async fn setup_with_settings(
    mut settings: Settings,
) -> (
    PgPool,
    String,
    impl Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) {
    setup_logging(&settings);

    let (pool, database_url) = setup_db(&mut settings).await;

    let secret_key = Hs256Key::new(settings.application.auth.secret_key.as_str());

    let (token_signer, authority) = crate::setup_authority!(secret_key);
    let pool_clone = pool.clone();

    let app = setup_with_config(move |service_config| {
        service_config
            .app_data(web::Data::new(pool_clone))
            // AuthenticationService by default doesnt include token_signer into app_data, hence we get it from setup_authority!() which is kinda hacky.
            .app_data(web::Data::new(token_signer.clone()))
            .service(
                web::scope("/api/user/auth")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login)),
            )
            .service(web::scope("/misc").route("/", web::get().to(get_app_info)))
            .service(
                web::scope("/api")
                    .wrap(AuthenticationService::new(authority))
                    .service(
                        web::scope("/user")
                            .route("/", web::patch().to(user_patch))
                            .route("/me", web::get().to(user_me_get))
                            .route("/{id}", web::get().to(user_get))
                            .route("/me", web::delete().to(user_me_delete))
                            .route(
                                "/{id}",
                                web::delete().to(user_delete).wrap(
                                    AuthorizationMiddlewareFactory::new(
                                        secret_key.clone(),
                                        Role::Admin,
                                    ),
                                ),
                            )
                            .route("/{id}/peers", web::get().to(user_peers_get)),
                    )
                    .service(
                        web::scope("/torrent")
                            .route("/", web::get().to(torrent_get))
                            .route("/", web::put().to(torrent_put))
                            .route("/", web::patch().to(torrent_patch)),
                    ),
            )
            .default_service(web::to(|| HttpResponse::NotFound()));
    })
    .await;

    (pool, database_url, app)
}

pub async fn setup_with_config<F: FnOnce(&mut ServiceConfig) -> ()>(
    config_fn: F,
) -> impl Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    init_service(App::new().configure(config_fn)).await
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

    assert!(database_drop_command.success());
}

pub(crate) fn setup_logging(settings: &Settings) {
    if settings.actix.enable_log {
        ENV_LOGGER_INIT.call_once(|| {
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
        });
    }
}

pub(crate) async fn setup_db(settings: &mut Settings) -> (PgPool, String) {
    Settings::override_field(&mut settings.application.database.name, "laguna_test_db")
        .expect("Cannot set database name to laguna_test_db");

    let database_url_with_uuid =
        format!("{}{}", settings.application.database.url(), Uuid::new_v4());
    let database_create_status = Command::new("sqlx")
        .args(&[
            "database",
            "reset",
            &format!("--database-url={}", database_url_with_uuid),
            &format!("--source={}", MIGRATIONS_DIR),
            "-y",
        ])
        .status()
        .expect("sqlx database create command failed");

    assert!(database_create_status.success());

    // Database connection setup.
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url_with_uuid.as_str())
        .await
        .expect("Failed to connect to database");

    // Run database migrations.
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    (pool, database_url_with_uuid)
}

// Waiting for this to resolve: https://github.com/rust-lang/rust/pull/93582.
// Use macro in the meantime.
#[rustversion::nightly]
#[feature(impl_trait_in_fn_trait_return)]
pub(crate) fn setup_authority(
    settings: &Settings,
) -> Authority<UserDTO, Hs256, impl Fn() -> impl Future<Output = Result<(), actix_web::Error>>, ()>
{
    let secret_key = Hs256Key::new(settings.application.auth.secret_key.as_str());

    let authority = Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .enable_header_tokens(true)
        .access_token_name(ACCESS_TOKEN_HEADER_NAME)
        .algorithm(Hs256)
        .time_options(TimeOptions::from_leeway(Duration::seconds(5)))
        .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
        .token_signer(Some(
            TokenSigner::new()
                .signing_key(secret_key.clone())
                .algorithm(Hs256)
                .access_token_name(ACCESS_TOKEN_HEADER_NAME)
                .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
                .access_token_lifetime(Duration::seconds(ACCESS_TOKEN_LIFETIME_SECONDS))
                .refresh_token_lifetime(Duration::seconds(REFRESH_TOKEN_LIFETIME_SECONDS))
                .build()
                .expect("Cannot create token signer"),
        ))
        .verifying_key(secret_key.clone())
        .build()
        .expect("Cannot create key authority");

    authority
}

#[macro_export]
macro_rules! setup_authority {
    ($secret_key:ident) => {{
        use ::laguna_backend_middleware::consts::ACCESS_TOKEN_HEADER_NAME;
        use ::laguna_backend_middleware::consts::ACCESS_TOKEN_LIFETIME_SECONDS;
        use ::laguna_backend_middleware::consts::REFRESH_TOKEN_HEADER_NAME;
        use ::laguna_backend_middleware::consts::REFRESH_TOKEN_LIFETIME_SECONDS;
        (
            TokenSigner::<UserDTO, Hs256>::new()
                .signing_key($secret_key.clone())
                .algorithm(Hs256)
                .access_token_name(ACCESS_TOKEN_HEADER_NAME)
                .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
                .access_token_lifetime(Duration::seconds(ACCESS_TOKEN_LIFETIME_SECONDS))
                .refresh_token_lifetime(Duration::seconds(REFRESH_TOKEN_LIFETIME_SECONDS))
                .build()
                .expect("Cannot create token signer"),
            Authority::<UserDTO, Hs256, _, _>::new()
                .refresh_authorizer(|| async move { Ok(()) })
                .enable_header_tokens(true)
                .time_options(TimeOptions::from_leeway(Duration::seconds(5)))
                .algorithm(Hs256)
                .access_token_name(ACCESS_TOKEN_HEADER_NAME)
                .refresh_token_name(REFRESH_TOKEN_HEADER_NAME)
                .verifying_key($secret_key.clone())
                .build()
                .expect("Cannot create key authority"),
        )
    }};
}

/// Registers and logs in a default user (Normie) with fake data.
pub(crate) async fn new_user(
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    new_user_with(Faker.fake::<RegisterDTO>(), &app).await
}

/// Registers and logs in a Verified user with fake data.
#[allow(dead_code)]
pub(crate) async fn new_verified_user(
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
    sqlx::query("UPDATE \"User\" SET role = 'Verified' WHERE id = $1")
        .bind(user_dto.id)
        .execute(pool)
        .await
        .expect("Unable to set user to 'Verified'");
    // Get the updated tokens for the updated user.
    let (user_dto, access_token, refresh_token) =
        login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
    (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in a Moderator user with fake data.
#[allow(dead_code)]
pub(crate) async fn new_mod_user(
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
    sqlx::query("UPDATE \"User\" SET role = 'Mod' WHERE id = $1")
        .bind(user_dto.id)
        .execute(pool)
        .await
        .expect("Unable to set user to 'Mod'");
    let (user_dto, access_token, refresh_token) =
        login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
    (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in an Admin user with fake data.
#[allow(dead_code)]
pub(crate) async fn new_admin_user(
    app: &impl dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error>,
    pool: &PgPool,
) -> (RegisterDTO, UserDTO, HeaderValue, HeaderValue) {
    let (register_dto, user_dto, _, _) = new_user_with(Faker.fake::<RegisterDTO>(), &app).await;
    sqlx::query("UPDATE \"User\" SET role = 'Admin' WHERE id = $1")
        .bind(user_dto.id)
        .execute(pool)
        .await
        .expect("Unable to set user to 'Admin'");
    let (user_dto, access_token, refresh_token) =
        login_user_safe(LoginDTO::from(register_dto.clone()), &app).await;
    (register_dto, user_dto, access_token, refresh_token)
}

/// Registers and logs in a Normie user given a RegisterDTO.
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

#[allow(dead_code)]
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

// Replaces the last character of the string with its successor, guaranteeing that the new string is different from the original.
// We need this because we have tests (using fake data) that require different strings than original which Faker cannot guarantee.
// TODO: Find a better way
#[allow(dead_code)]
pub(crate) fn different_string(string: String) -> String {
    string[..string.len() - 1].to_owned()
        + char::from_u32(string.chars().last().unwrap() as u32 + 1)
            .unwrap()
            .to_string()
            .as_str()
}
