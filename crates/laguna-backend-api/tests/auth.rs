// Uncomment the comment below if it doesn't work.
// #[path = "common/mod.rs"]
mod common;

use actix_web::{
    http::StatusCode,
    test::{read_body_json, TestRequest},
};

use laguna_backend_api::error::LoginError;
use laguna_backend_model::{login::LoginDTO, register::RegisterDTO};

#[actix_web::test]
async fn test_register() {
    let (pool, app) = common::setup().await;
    let res = common::register_new_user(
        RegisterDTO {
            username: String::from("test_register"),
            email: String::from("test_register@laguna.io"),
            password: String::from("test123"),
        },
        &app,
    )
    .await
    .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_register_twice() {
    let (pool, app) = common::setup().await;
    let res = common::register_new_user(
        RegisterDTO {
            username: String::from("test_register_twice"),
            email: String::from("test_register_twice@laguna.io"),
            password: String::from("test123"),
        },
        &app,
    )
    .await
    .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = common::register_new_user(
        RegisterDTO {
            username: String::from("test_register_twice"),
            email: String::from("test_register_twice@laguna.io"),
            password: String::from("test123"),
        },
        &app,
    )
    .await
    .unwrap();
    assert_eq!(res.status(), StatusCode::ALREADY_REPORTED);
    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_login() {
    let (pool, app) = common::setup().await;
    let res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_login"),
            email: String::from("test_login@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_login"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_login_with_wrong_username_or_email() {
    let (pool, app) = common::setup().await;
    let res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_login_wrong"),
            email: String::from("test_login_wrong@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_login_wrong"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);

    // Wrong username
    let res = common::login_new_user(
        LoginDTO {
            username_or_email: String::from("test_login_2938"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;

    assert_eq!(res.unwrap().status(), StatusCode::UNAUTHORIZED);

    // Wrong email
    let res = common::login_new_user(
        LoginDTO {
            username_or_email: String::from("tnwiefn@laguna.com"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;

    assert_eq!(res.unwrap().status(), StatusCode::UNAUTHORIZED);
    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_login_with_wrong_password() {
    let (pool, app) = common::setup().await;
    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_login_wrong_pwd"),
            email: String::from("test_login_wrong_pwd@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_login_wrong_pwd"),
            password: String::from("lololool"),
        },
        &app,
    )
    .await;

    assert_eq!(login_res.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        read_body_json::<LoginError, _>(login_res).await,
        LoginError::InvalidCredentials
    );
    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_access_token() {
    let (pool, app) = common::setup().await;

    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_access_token"),
            email: String::from("test_access_token@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_access_token"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let res = common::request_with_jwt_cookies_set(
        &login_res,
        TestRequest::get().uri("/api/user/me"),
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);

    common::teardown(pool).await;
}

#[actix_web::test]
#[ignore = "Not implemented"]
async fn test_refresh_token() {}
