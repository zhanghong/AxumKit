use kit_entity::common::Permission;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RolePermissionsResponse {
    pub permissions: Vec<Permission>,
}
