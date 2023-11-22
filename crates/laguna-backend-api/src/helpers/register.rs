use actix_web::web;
use laguna_backend_model::user::User;
use rand::Rng;

use crate::error::APIError;

pub(crate) async fn generate_username_recommendations(
  user: User,
  pool: &web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<Vec<String>, APIError> {
  let recommendations = vec![
    format!(
      "{}{}",
      user.username,
      rand::thread_rng().gen_range(0..10000)
    ),
    format!(
      "{}{}",
      user.username,
      rand::thread_rng().gen_range(0..10000)
    ),
    format!(
      "{}{}",
      user.username,
      rand::thread_rng().gen_range(0..10000)
    ),
  ];
  let mut recommendations_filtered = Vec::with_capacity(recommendations.capacity());
  for recomm in recommendations.into_iter() {
    if sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM "User" WHERE username = $1"#)
      .bind(&recomm)
      .fetch_one(pool.get_ref())
      .await?
      == 0
    {
      recommendations_filtered.push(recomm)
    }
  }
  Ok(recommendations_filtered)
}
