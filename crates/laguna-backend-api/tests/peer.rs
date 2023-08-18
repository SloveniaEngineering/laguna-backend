use actix_web::test::TestRequest;

use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
#[ignore]
pub async fn test_announce_peer_started(pool: PgPool) -> sqlx::Result<()> {
  let app = common::setup_test(&pool).await;
  let (_, _user_dto, access_token, refresh_token) = common::new_user(&app).await;
  let _get_me_res = common::as_logged_in(
    access_token,
    refresh_token,
    TestRequest::with_uri("/api/peer/announce"),
    &app,
  )
  .await
  .unwrap();
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
#[ignore]
async fn test_announce_peer_stopped(_pool: PgPool) -> sqlx::Result<()> {
  Ok(())
}

#[sqlx::test(migrations = "../../migrations")]
#[ignore]
async fn test_patch_peer(_pool: PgPool) -> sqlx::Result<()> {
  Ok(())
}
