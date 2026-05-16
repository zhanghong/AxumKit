use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::model::role::Role;
use crate::domain::model::user_role::Model as UserRoleModel;
use kit_errors::errors::Errors;

/// Repository interface for user role operations
#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    /// Create a user role
    async fn create_user_role(
        &self,
        user_id: Uuid,
        role: Role,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserRoleModel, Errors>;

    /// Find user roles (excluding expired)
    async fn find_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, Errors>;

    /// Delete a user role
    async fn delete_user_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors>;

    /// Delete expired user role
    async fn delete_expired_user_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors>;
}
