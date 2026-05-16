use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::model::role::Role;
use crate::domain::model::user_ban::Model as UserBanModel;
use crate::domain::model::user_role::Model as UserRoleModel;
use crate::domain::service::user_management_service::UserManagementService;
use kit_errors::errors::Errors;

/// Application service for user management operations
/// Coordinates domain services to fulfill use cases
#[derive(Clone)]
pub struct UserManagementApplicationService {
    user_management_service: UserManagementService,
}

impl UserManagementApplicationService {
    /// Create a new user management application service instance
    pub fn new(user_management_service: UserManagementService) -> Self {
        Self {
            user_management_service,
        }
    }

    /// Ban a user
    pub async fn ban_user(
        &self,
        user_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserBanModel, Errors> {
        self.user_management_service.ban_user(user_id, expires_at).await
    }

    /// Unban a user
    pub async fn unban_user(&self, user_id: Uuid) -> Result<u64, Errors> {
        self.user_management_service.unban_user(user_id).await
    }

    /// Check if user is banned
    pub async fn is_user_banned(&self, user_id: Uuid) -> Result<bool, Errors> {
        self.user_management_service.is_user_banned(user_id).await
    }

    /// Grant role to user
    pub async fn grant_role(
        &self,
        user_id: Uuid,
        role: Role,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserRoleModel, Errors> {
        self.user_management_service
            .grant_role(user_id, role, expires_at)
            .await
    }

    /// Revoke role from user
    pub async fn revoke_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors> {
        self.user_management_service.revoke_role(user_id, role).await
    }

    /// Get user roles
    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, Errors> {
        self.user_management_service.get_user_roles(user_id).await
    }
}
