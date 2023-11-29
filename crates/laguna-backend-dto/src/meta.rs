use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AppInfoDTO {
  pub version: String,
  pub authors: Vec<String>,
  pub license: String,
  pub description: String,
  pub repository: String,
}
