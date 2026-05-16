use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::model::user_ban::Model as UserBanModel;
use kit_errors::errors::Errors;

/// Repository interface for user ban operations
#[async_trait]
pub trait UserBanRepository: Send + Sync {
    /// Create a user ban
    async fn create_user_ban(
        &self,
        user_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserBanModel, Errors>;

    /// Find user ban (excluding expired)
    async fn find_user_ban(&self, user_id: Uuid) -> Result<Option<UserBanModel>, Errors>;

    /// Check if user is banned
    async fn is_user_banned(&self, user_id: Uuid) -> Result<bool, Errors>;

    /// Delete user ban
    async fn delete_user_ban(&self, user_id: Uuid) -> Result<u64, Errors>;

    /// Delete expired user ban
    async fn delete_expired_user_ban(&self, user_id: Uuid) -> Result<u64, Errors>;
}
