use kit_entity::common::Role;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetRolePermissionsRequest {
    pub role: Role,
}
