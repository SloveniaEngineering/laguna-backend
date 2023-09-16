use laguna_backend_model::role::Role;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RoleChangeDTO {
  pub to: Role,
}
