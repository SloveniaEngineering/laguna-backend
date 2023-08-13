use actix_http::StatusCode;
use actix_web::test::{read_body_json, TestRequest};

use laguna_backend_dto::user::{UserDTO, UserPatchDTO};
use sqlx::PgPool;
use uuid::Uuid;

mod common;

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_me(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_user_by_normie(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_user_by_verified_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_user_by_mod(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_user_by_admin(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_delete_inexistant_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_get_inexistant_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
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
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_user(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: Some(String::from("https://example.com")),
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let mut user_dto_expected = user_dto;
  user_dto_expected.avatar_url = Some(String::from("https://example.com"));
  user_dto_expected.is_history_private = false;
  user_dto_expected.is_profile_private = true;
  assert_eq!(
    read_body_json::<UserDTO, _>(get_me_res).await,
    user_dto_expected
  );
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_patch_user_remove_avatar_url(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let get_me_res = common::as_logged_in(
    access_token.clone(),
    refresh_token.clone(),
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: Some(String::from("https://example.com")),
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::patch()
      .uri(&format!("/api/user/{}", user_dto.id))
      .set_json(UserPatchDTO {
        avatar_url: None,
        is_history_private: false,
        is_profile_private: true,
      }),
    &app,
  )
  .await
  .unwrap();
  assert_eq!(get_me_res.status(), StatusCode::OK);
  let mut user_dto_expected = user_dto;
  user_dto_expected.avatar_url = None;
  user_dto_expected.is_history_private = false;
  user_dto_expected.is_profile_private = true;
  assert_eq!(
    read_body_json::<UserDTO, _>(get_me_res).await,
    user_dto_expected
  );
  Ok(())
}
