use laguna_backend_model::role::Role;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RoleChangeDTO {
  pub to: Role,
}
