use actix_http::StatusCode;

use actix_web::test::{read_body_json, TestRequest};

use laguna_backend_model::user::UserDTO;

mod common;

#[actix_web::test]
async fn test_get_me() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
    let get_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::with_uri("/api/user/me"),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(get_me_res.status(), StatusCode::OK);
    assert_eq!(read_body_json::<UserDTO, _>(get_me_res).await, user_dto);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_get_user() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
    let get_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::with_uri(&format!("/api/user/{}", user_dto.id)),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(get_me_res.status(), StatusCode::OK);
    assert_eq!(read_body_json::<UserDTO, _>(get_me_res).await, user_dto);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_me() {
    let (pool, database_url, app) = common::setup().await;
    let (_, _, access_token, refresh_token) = common::new_user(&app).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri("/api/user/delete/me"),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(delete_me_res.status(), StatusCode::OK);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_user() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/delete/{}", user_dto.id)),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(delete_me_res.status(), StatusCode::OK);
    common::teardown(pool, database_url).await;
}
