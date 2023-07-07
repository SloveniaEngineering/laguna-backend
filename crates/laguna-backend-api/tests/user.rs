use actix_http::StatusCode;

use actix_web::dev::Service;
use actix_web::test::{read_body_json, TestRequest};

use laguna_backend_model::login::LoginDTO;
use laguna_backend_model::register::RegisterDTO;
use laguna_backend_model::user::UserDTO;

mod common;

#[actix_web::test]
#[ignore = "Doesn't work"]
async fn test_get_me() {
    let (pool, app) = common::setup().await;

    let res = common::register_and_login_new_user(
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
    assert_eq!(res.status(), StatusCode::OK);

    let res =
        common::request_with_jwt_cookies_set(&res, TestRequest::with_uri("/api/user/me"), &app)
            .await;
    assert_eq!(res.status(), StatusCode::OK);
    common::teardown(pool).await;
}

#[actix_web::test]
#[ignore = "Doesn't work"]
async fn test_get_one() {
    let (pool, app) = common::setup().await;

    let res = common::register_and_login_new_user(
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
    assert_eq!(res.status(), StatusCode::OK);

    let mut req = common::test_request_with_cookies_from_response(&res).await;
    let user = read_body_json::<UserDTO, _>(res).await;
    req = req.uri(&format!("/api/user/{}", user.id));

    let res = app.call(req.to_request()).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    common::teardown(pool).await;
}
