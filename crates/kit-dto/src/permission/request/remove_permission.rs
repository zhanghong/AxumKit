use kit_entity::common::{Permission, Role};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RemovePermissionRequest {
    pub role: Role,
    pub permission: Permission,
}
