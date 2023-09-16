use crate::common::setup_test;
use sqlx::PgPool;

mod common;

#[sqlx::test(migrations = "../../migrations")]
#[ignore]
async fn test_peer_stats(pool: PgPool) -> sqlx::Result<()> {
  let app = setup_test(&pool).await;
  let (_, _, access_token, refresh_token) = common::new_user(&app).await;
  // Upload some torrents

  Ok(())
}
