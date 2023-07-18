// Uncomment the comment below if it doesn't work.
// #[path = "common/mod.rs"]
mod common;

use actix_web::http::StatusCode;

use fake::Fake;
use fake::Faker;

use laguna_backend_model::{login::LoginDTO, register::RegisterDTO};

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
async fn test_login() {
    let (pool, database_url, app) = common::setup().await;
    let (_, _, _, _) = common::new_user(&app).await;
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_login_with_wrong_username() {
    let (pool, database_url, app) = common::setup().await;
    let (register_dto, user_dto, _, _) = common::new_user(&app).await;
    let login_res = common::login_user(
        LoginDTO {
            username_or_email: user_dto.username[..user_dto.username.len() / 2].to_string(),
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
            username_or_email: register_dto.email[..register_dto.email.len() - 1].to_string() + "x",
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
            password: register_dto.password[..register_dto.password.len() - 1].to_string() + "x",
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
    register_dto.password = String::from("a".repeat(256));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_password_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.password = String::from("a".repeat(2));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_username_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.username = String::from("a".repeat(256));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_username_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.username = String::from("a".repeat(2));
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_too_long() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = register_dto.email + &"a".repeat(256);
    let register_res = common::register_user(register_dto, &app).await;
    assert_eq!(register_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_register_email_too_short() {
    let (pool, database_url, app) = common::setup().await;
    let mut register_dto = Faker.fake::<RegisterDTO>();
    register_dto.email = String::from("a".repeat(1));
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
