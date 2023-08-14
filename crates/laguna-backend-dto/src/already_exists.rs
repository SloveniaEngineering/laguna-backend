use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AlreadyExistsDTO {
  pub message: String,
  pub recommended_usernames: Vec<String>,
}
