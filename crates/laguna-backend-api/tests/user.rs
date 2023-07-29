use actix_http::StatusCode;
use actix_web::test::{read_body_json, TestRequest};

use laguna_backend_dto::user::UserDTO;
use uuid::Uuid;

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
        TestRequest::delete().uri("/api/user/me"),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(delete_me_res.status(), StatusCode::OK);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_user_by_normie() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/{}", user_dto.id)),
        &app,
    )
    .await;
    assert_eq!(
        delete_me_res.unwrap_err().as_response_error().status_code(),
        StatusCode::UNAUTHORIZED
    );
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_user_by_verified_user() {
    let (pool, _database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_verified_user(&app, &pool).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/{}", user_dto.id)),
        &app,
    )
    .await;
    assert_eq!(
        delete_me_res.unwrap_err().as_response_error().status_code(),
        StatusCode::UNAUTHORIZED
    );
}

#[actix_web::test]
async fn test_delete_user_by_mod() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_mod_user(&app, &pool).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/{}", user_dto.id)),
        &app,
    )
    .await;
    assert_eq!(
        delete_me_res.unwrap_err().as_response_error().status_code(),
        StatusCode::UNAUTHORIZED
    );
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_user_by_admin() {
    let (pool, database_url, app) = common::setup().await;
    let (_, user_dto, access_token, refresh_token) = common::new_admin_user(&app, &pool).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/{}", user_dto.id)),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(delete_me_res.status(), StatusCode::OK);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_delete_inexistant_user() {
    let (pool, database_url, app) = common::setup().await;
    let (_, _, access_token, refresh_token) = common::new_admin_user(&app, &pool).await;
    let delete_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::delete().uri(&format!("/api/user/{}", Uuid::new_v4())),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(delete_me_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}

#[actix_web::test]
async fn test_get_inexistant_user() {
    let (pool, database_url, app) = common::setup().await;
    let (_, _, access_token, refresh_token) = common::new_user(&app).await;
    let get_me_res = common::as_logged_in(
        access_token,
        refresh_token,
        TestRequest::get().uri(&format!("/api/user/{}", Uuid::new_v4())),
        &app,
    )
    .await
    .unwrap();
    assert_eq!(get_me_res.status(), StatusCode::BAD_REQUEST);
    common::teardown(pool, database_url).await;
}
