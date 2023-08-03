// Uncomment the comment below if it doesn't work.
// #[path = "common/mod.rs"]
mod common;

use std::thread;

use actix_web::http::StatusCode;

use common::get_dev_settings;

use fake::Fake;
use fake::Faker;

use laguna_config::Settings;
use std::time::Duration as StdDuration;

use laguna_backend_dto::{login::LoginDTO, register::RegisterDTO};
use laguna_backend_model::consts::EMAIL_MAX_LEN;
use laguna_backend_model::consts::EMAIL_MIN_LEN;
use laguna_backend_model::consts::PASSWORD_MAX_LEN;
use laguna_backend_model::consts::PASSWORD_MIN_LEN;
use laguna_backend_model::consts::USERNAME_MAX_LEN;
use laguna_backend_model::consts::USERNAME_MIN_LEN;

use crate::common::different_string;
use crate::common::setup_with_settings;

#[actix_web::test]
async fn test_register() {
    let (pool, database_url, app) = common::setup().await;
    common::register_user_safe(Faker.fake::<RegisterDTO>(), &app).await;
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_twice() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::ALREADY_REPORTED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_with_existing_username() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::ALREADY_REPORTED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_with_existing_email() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::ALREADY_REPORTED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login() {
    let (pool, database_url, app) = common::setup().await;
    let (_, _, _, _) = common::new_user(&app).await;
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_password_control_char() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: register_dto.username,
            password: String::from("a\nb\r\t"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_username_or_email_control_char() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: String::from("a\nb\r\t@x.y"),
            password: register_dto.password,
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_username_or_email_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: String::from("a".repeat(USERNAME_MAX_LEN + 1)),
            password: register_dto.password,
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_password_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: register_dto.username,
            password: String::from("a".repeat(PASSWORD_MAX_LEN + 1)),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_password_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: register_dto.username,
            password: String::from("a".repeat(PASSWORD_MIN_LEN - 1)),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_username_or_email_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: String::from("a".repeat(USERNAME_MIN_LEN - 1)),
            password: register_dto.password,
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_wrong_username() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: different_string(register_dto.username),
            password: register_dto.password,
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::UNAUTHORIZED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_wrong_email() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, _, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: different_string(register_dto.email),
            password: register_dto.password,
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::UNAUTHORIZED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_wrong_password() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, user_dto, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: user_dto.username,
            password: different_string(register_dto.password),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::UNAUTHORIZED);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_password_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.password = String::from("a".repeat(PASSWORD_MAX_LEN + 1));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_password_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.password = String::from("a".repeat(PASSWORD_MIN_LEN - 1));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_username_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.username = String::from("a".repeat(USERNAME_MAX_LEN + 1));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_username_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.username = String::from("a".repeat(USERNAME_MIN_LEN - 1));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = register_dto.email + &"a".repeat(EMAIL_MAX_LEN + 1);
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = String::from("a".repeat(EMAIL_MIN_LEN - 1));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_invalid() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = String::from("invalid.email");
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_username_with_control_characters() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.username = String::from("a\nb");
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_with_control_characters() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = String::from("a\nb\t\r@x.y");
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_password_with_control_characters() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.password = String::from("a\nb\r\t");
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_out_of_date_access_token() {
    let mut settings = get_dev_settings();
    Settings::override_field(
        &mut settings.application.auth.access_token_lifetime_seconds,
        "0",
    )
    .expect("Failed to override field");

    let (pool, database_url, app) = setup_with_settings(settings).await;

    let (register_dto, _, access_token_old, _) = common::new_user(&app).await;

    thread::sleep(StdDuration::from_secs(3));

    let (_, access_token, _) = common::login_user_safe(register_dto.into(), &app).await;

    // Old and new access tokens should be different.
    assert_ne!(access_token_old, access_token);

    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_out_of_date_refresh_token() {
    let mut settings = get_dev_settings();
    Settings::override_field(
        &mut settings.application.auth.access_token_lifetime_seconds,
        "0",
    )
    .expect("Failed to override field");
    Settings::override_field(
        &mut settings.application.auth.refresh_token_lifetime_seconds,
        "0",
    )
    .expect("Failed to override field");

    let (pool, database_url, app) = setup_with_settings(settings).await;

    let (register_dto, _, _, refresh_token_old) = common::new_user(&app).await;

    // Just to be sure that refresh token is out of date.
    thread::sleep(StdDuration::from_secs(3));

    let (_, _, refresh_token) = common::login_user_safe(register_dto.into(), &app).await;

    // Old and new refresh tokens should be different.
    assert_ne!(refresh_token_old, refresh_token);

    common::teardown(pool, database_url).await;
}
