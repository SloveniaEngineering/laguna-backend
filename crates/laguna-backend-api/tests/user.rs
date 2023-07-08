use actix_http::StatusCode;

use actix_web::test::{read_body_json, TestRequest};

use laguna_backend_api::state::UserState;
use laguna_backend_model::login::LoginDTO;
use laguna_backend_model::register::RegisterDTO;
use laguna_backend_model::user::{User, UserDTO};

mod common;

#[actix_web::test]
async fn test_get_me() {
    let (pool, app) = common::setup().await;

    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_get_me"),
            email: String::from("test_get_me@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_get_me"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_get_me")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt_cookies_set(
        &login_res,
        TestRequest::get().uri("/api/user/me"),
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(read_body_json::<UserDTO, _>(res).await, user.into());

    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_get_one() {
    let (pool, app) = common::setup().await;

    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_get_one"),
            email: String::from("test_get_one@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_get_one"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    // TODO: Can we not hit DB again?
    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_get_one")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt_cookies_set(
        &login_res,
        TestRequest::get().uri(&format!("/api/user/{}", user.id)),
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(read_body_json::<UserDTO, _>(res).await, user.into());
    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_delete_me() {
    let (pool, app) = common::setup().await;

    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_delete_me"),
            email: String::from("test_delete_me@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_delete_me"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;
    assert_eq!(login_res.status(), StatusCode::OK);

    let res = common::request_with_jwt_cookies_set(
        &login_res,
        TestRequest::delete().uri("/api/user/delete/me"),
        &app,
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<UserState, _>(res).await,
        UserState::DeleteSuccess
    );

    common::teardown(pool).await;
}

#[actix_web::test]
async fn test_delete_one() {
    let (pool, app) = common::setup().await;

    let login_res = common::register_and_login_new_user(
        RegisterDTO {
            username: String::from("test_delete_one"),
            email: String::from("test_delete_one@laguna.io"),
            password: String::from("test123"),
        },
        LoginDTO {
            username_or_email: String::from("test_delete_one"),
            password: String::from("test123"),
        },
        &app,
    )
    .await;

    assert_eq!(login_res.status(), StatusCode::OK);

    let user = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE username = $1")
        .bind("test_delete_one")
        .fetch_one(&pool)
        .await
        .unwrap();

    let res = common::request_with_jwt_cookies_set(
        &login_res,
        TestRequest::delete().uri(&format!("/api/user/delete/{}", user.id)),
        &app,
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        read_body_json::<UserState, _>(res).await,
        UserState::DeleteSuccess
    );
    common::teardown(pool).await;
}
