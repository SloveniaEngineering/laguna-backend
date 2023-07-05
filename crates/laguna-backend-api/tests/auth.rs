use actix_jwt_auth_middleware::{use_jwt::UseJWTOnApp, Authority, TokenSigner};
use actix_web::{
    dev::{Service, ServiceResponse},
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    test::{init_service, TestRequest},
    web, App,
};

use chrono::{Duration, Utc};
use env_logger;

use jwt_compact::{
    alg::{Hs256, Hs256Key},
    TimeOptions,
};
use laguna_backend_api::{login::login, register::register, user::me};
use laguna_backend_model::{
    login::LoginDTO,
    register::RegisterDTO,
    user::{Behaviour, Role, UserDTO},
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
        .set_json(RegisterDTO {
            username: String::from("test"),
            email: String::from("test@laguna.io"),
            password: String::from("test123"),
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Test already exists
    let req = TestRequest::post()
        .set_json(RegisterDTO {
            username: String::from("test"),
            email: String::from("test@laguna.io"),
            password: String::from("test123"),
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
        .set_json(RegisterDTO {
            username: String::from("test_login"),
            email: String::from("test_login@laguna.io"),
            password: String::from("test123"),
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // Test login with username
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_login"),
            password: String::from("test123"),
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
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // Test login with wrong username
    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_loginx"),
            password: String::from("test123"),
        })
        .uri("/login");

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    teardown(pool).await;
}

#[actix_web::test]
async fn test_access_and_refresh_token() {
    let pool = setup().await;
    let key = Hs256Key::new("some random test shit");
    let authority = Authority::<UserDTO, Hs256, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .enable_header_tokens(true) // see comment below
        .token_signer(Some(
            TokenSigner::new()
                .signing_key(key.clone())
                .algorithm(Hs256)
                .time_options(TimeOptions::from_leeway(Duration::nanoseconds(5))) // to make sure refresh is triggered. TODO: this is kind of best-effort like, can we explicitly test this?
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
            .use_jwt(
                authority,
                web::scope("/api").service(web::scope("/user").service(me)),
            ),
    )
    .await;

    let req = TestRequest::post()
        .set_json(RegisterDTO {
            username: String::from("test_access_refresh"),
            email: String::from("test_access_refresh@laguna.io"),
            password: String::from("test123"),
        })
        .uri("/register");
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let req = TestRequest::post()
        .set_json(LoginDTO {
            username_or_email: String::from("test_access_refresh"),
            password: String::from("test123"),
        })
        .uri("/login");

    // TODO: Find better way.
    // Guess what? HttpRequest has ::cookies(), but TestRequest doesn't have ::cookies() => we must use headers in order to use TestRequest.
    // Why do we need cookie? Because actix-jwt-auth-middleware deals with cookies (when sending, when receiving, however, it is customizable).
    // So we enable headers with [`Authority::enable_header_tokens(true)`].
    // This is why the below parsing is required.
    fn cookie_in_header_to_token(cookie: &HeaderValue) -> String {
        // INPUT FORMAT: {access,refresh}_token=<TOKEN>; Secure
        // OUTPUT FORMAT: <TOKEN>
        // First get access_token=<TOKEN>; by splitting whitespace
        let unprocessed_token = cookie.to_str().unwrap().split_whitespace().next().unwrap();
        // Second get rid of ;
        let unprocessed_token = unprocessed_token
            .chars()
            .take(unprocessed_token.len() - 1)
            .collect::<Vec<char>>();
        // Split on = (position of(=) + 1 because we don't want =) to get <TOKEN> into RHS token value
        let (_, token) = unprocessed_token
            .split_at(unprocessed_token.iter().position(|c| c == &'=').unwrap() + 1);
        token.to_vec().into_iter().collect::<String>()
    }

    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    let mut cookies = res.headers().get_all(header::SET_COOKIE);
    let access_token = cookie_in_header_to_token(cookies.next().unwrap());
    let refresh_token = cookie_in_header_to_token(cookies.next().unwrap());
    assert_eq!(cookies.next(), None);

    let req = TestRequest::get()
        .uri("/api/user/me")
        .append_header(("access_token", access_token))
        .append_header(("refresh_token", refresh_token));
    let res: ServiceResponse = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), 200);

    teardown(pool).await;
}
