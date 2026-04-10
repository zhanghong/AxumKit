use kit_entity::common::{Permission, Role};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddPermissionRequest {
    pub role: Role,
    pub permission: Permission,
}
