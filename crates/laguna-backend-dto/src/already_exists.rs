use serde::Serialize;
use utoipa::ToSchema;

#[allow(missing_docs)]
#[derive(Debug, Serialize, ToSchema)]
pub struct AlreadyExistsDTO {
  pub message: String,
  pub recommended_usernames: Vec<String>,
}
