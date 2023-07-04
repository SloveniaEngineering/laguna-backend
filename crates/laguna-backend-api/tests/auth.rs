use actix_jwt_auth_middleware::{use_jwt::UseJWTOnApp, Authority, TokenSigner};
use actix_web::{
    dev::{Service, ServiceResponse},
    http::{header, StatusCode},
    test::{init_service, TestRequest},
    web, App,
};

use chrono::Utc;
use env_logger;

use jwt_compact::alg::{Hs256, Hs256Key};
use laguna_backend_api::{login::login, register::register};
use laguna_backend_model::{
    login::LoginDTO,
    user::{Role, UserDTO},
};

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{env, sync::Once};

// Initialize env_logger only once.
static ENV_LOGGER_SETUP: Once = Once::new();

async fn setup() -> PgPool {
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

    pool
}

async fn teardown(pool: PgPool) {
    sqlx::query("DELETE FROM \"User\"")
        .execute(&pool)
        .await
        .expect("Failed to cleanup \"User\" table");
    pool.close().await;
}

#[actix_web::test]
async fn test_register() {
    let pool = setup().await;
    let app = init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(register),
    )
    .await;
    let req = TestRequest::post()
        .set_json(UserDTO {
            username: String::from("test"),
            email: String::from("test@laguna.io"),
            password: String::from("test123"),
            avatar_url: None,
            role: Role::Admin,
            is_active: None,
            is_history_private: None,
            first_login: None,
            last_login: None,
            has_verified_email: None,
            is_profile_private: None,
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Test already exists
    let req = TestRequest::post()
        .set_json(UserDTO {
            username: String::from("test"),
            email: String::from("test@laguna.io"),
            password: String::from("test123"),
            avatar_url: None,
            role: Role::Admin,
            is_active: None,
            is_history_private: None,
            first_login: None,
            last_login: None,
            has_verified_email: None,
            is_profile_private: None,
        })
        .uri("/register");

    let res = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::ALREADY_REPORTED);

    teardown(pool).await;
}

#[actix_web::test]
async fn test_login() {
    let pool = setup().await;
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
            .service(register)
            .service(login)
            .use_jwt(authority, web::scope("/api")),
    )
    .await;

    let req = TestRequest::post()
        .set_json(UserDTO {
            username: String::from("test_login"),
            email: String::from("test_login@laguna.io"),
            password: String::from("test123"),
            avatar_url: None,
            role: Role::Admin,
            is_active: None,
            is_history_private: None,
            first_login: None,
            last_login: None,
            has_verified_email: None,
            is_profile_private: None,
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Test login with username
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_login"),
            password: String::from("test123"),
            login_timestamp: Utc::now(),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key(header::SET_COOKIE));

    // Test login with email
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_login@laguna.io"),
            password: String::from("test123"),
            login_timestamp: Utc::now(),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key(header::SET_COOKIE));

    // Test login with wrong password
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_login"),
            password: String::from("seiufhoifhjqow"),
            login_timestamp: Utc::now(),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // Test login with wrong username
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_loginx"),
            password: String::from("test123"),
            login_timestamp: Utc::now(),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    teardown(pool).await;
}

#[actix_web::test]
async fn test_access_with_bearer() {
    let pool = setup().await;
    let key = Hs256Key::new("some random test shit");
    let authority = Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .enable_header_tokens(true)
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

    async fn me(user: UserDTO) -> String {
        format!("Hello {:?}", user)
    }

    let app = init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(register)
            .service(login)
            .use_jwt(
                authority,
                web::scope("/api").service(web::resource("/me").to(me)),
            ),
    )
    .await;

    let req = TestRequest::post()
        .set_json(UserDTO {
            username: String::from("test_access_with_bearer"),
            email: String::from("test_access_with_bearer@laguna.io"),
            password: String::from("test123"),
            avatar_url: None,
            role: Role::Admin,
            is_active: None,
            is_history_private: None,
            first_login: None,
            last_login: None,
            has_verified_email: None,
            is_profile_private: None,
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_access_with_bearer"),
            password: String::from("test123"),
            login_timestamp: Utc::now(),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    let mut cookies = res.headers().get_all(header::SET_COOKIE);
    let access_token = cookies.next().unwrap();
    let _refresh_token = cookies.next().unwrap();
    assert_eq!(cookies.next(), None);
    // Guess what? HttpRequest has ::cookies(), but TestRequest doesn't have ::cookies() => we must use headers in order to use TestRequest.
    // This is why the below parsing is required.
    let unprocessed_token = access_token
        .to_str()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    let unprocessed_token = unprocessed_token
        .chars()
        .take(unprocessed_token.len() - 1)
        .collect::<Vec<char>>();
    let (_, token) =
        unprocessed_token.split_at(unprocessed_token.iter().position(|c| c == &'=').unwrap() + 1);
    let token = token.to_vec().into_iter().collect::<String>();
    let req = TestRequest::get()
        .uri("/api/me")
        .append_header(("access_token", token));
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), 200);
    teardown(pool).await;
}
