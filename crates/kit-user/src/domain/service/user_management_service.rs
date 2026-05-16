use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::model::role::Role;
use crate::domain::model::user_ban::Model as UserBanModel;
use crate::domain::model::user_role::Model as UserRoleModel;
use crate::domain::repository::user_ban_repository::UserBanRepository;
use crate::domain::repository::user_role_repository::UserRoleRepository;
use kit_errors::errors::Errors;

/// Domain service for user management operations (ban, role)
#[derive(Clone)]
pub struct UserManagementService {
    user_ban_repository: Arc<dyn UserBanRepository>,
    user_role_repository: Arc<dyn UserRoleRepository>,
}

impl UserManagementService {
    /// Create a new user management service instance
    pub fn new(
        user_ban_repository: Arc<dyn UserBanRepository>,
        user_role_repository: Arc<dyn UserRoleRepository>,
    ) -> Self {
        Self {
            user_ban_repository,
            user_role_repository,
        }
    }

    /// Ban a user
    pub async fn ban_user(
        &self,
        user_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserBanModel, Errors> {
        // Delete expired ban first to avoid UNIQUE constraint violation
        let _ = self
            .user_ban_repository
            .delete_expired_user_ban(user_id)
            .await?;
        self.user_ban_repository
            .create_user_ban(user_id, expires_at)
            .await
    }

    /// Unban a user
    pub async fn unban_user(&self, user_id: Uuid) -> Result<u64, Errors> {
        self.user_ban_repository.delete_user_ban(user_id).await
    }

    /// Check if user is banned
    pub async fn is_user_banned(&self, user_id: Uuid) -> Result<bool, Errors> {
        self.user_ban_repository.is_user_banned(user_id).await
    }

    /// Grant role to user
    pub async fn grant_role(
        &self,
        user_id: Uuid,
        role: Role,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserRoleModel, Errors> {
        // Delete expired role first to avoid UNIQUE constraint violation
        let _ = self
            .user_role_repository
            .delete_expired_user_role(user_id, role)
            .await?;
        self.user_role_repository
            .create_user_role(user_id, role, expires_at)
            .await
    }

    /// Revoke role from user
    pub async fn revoke_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors> {
        self.user_role_repository.delete_user_role(user_id, role).await
    }

    /// Get user roles
    pub async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, Errors> {
        self.user_role_repository.find_user_roles(user_id).await
    }
}
