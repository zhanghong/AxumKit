use async_trait::async_trait;
use sea_orm::prelude::DateTimeUtc;
use uuid::Uuid;

use crate::domain::model::user::Model as UserModel;
use kit_errors::errors::Errors;

/// User update parameters
/// - `Option<T>`: None = no change, Some(value) = change to value
/// - `Option<Option<T>>`: None = no change, Some(None) = set to NULL, Some(Some(value)) = set to value
#[derive(Default)]
pub struct UserUpdateParams {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<Option<String>>,
    pub password: Option<Option<String>>,
    pub verified_at: Option<Option<DateTimeUtc>>,
    pub profile_image: Option<Option<String>>,
    pub banner_image: Option<Option<String>>,
    pub totp_secret: Option<Option<String>>,
    pub totp_enabled_at: Option<Option<DateTimeUtc>>,
    pub totp_backup_codes: Option<Option<Vec<String>>>,
}

/// Repository interface for user operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user with password
    async fn create_user(
        &self,
        email: String,
        handle: String,
        display_name: String,
        password: String,
    ) -> Result<UserModel, Errors>;

    /// Create a user with pre-hashed password (used by email verification flow)
    async fn create_user_with_password_hash(
        &self,
        email: String,
        handle: String,
        display_name: String,
        password_hash: String,
    ) -> Result<UserModel, Errors>;

    /// Find user by id (returns Option)
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Errors>;

    /// Find user by email (returns Option)
    async fn find_user_by_email(&self, email: String) -> Result<Option<UserModel>, Errors>;

    /// Find user by handle (returns Option)
    async fn find_user_by_handle(&self, handle: String) -> Result<Option<UserModel>, Errors>;

    /// Find users by ids
    async fn find_users_by_ids(&self, ids: &[Uuid]) -> Result<Vec<UserModel>, Errors>;

    /// Get user by id (returns UserNotFound error if not found)
    async fn get_user_by_id(&self, id: Uuid) -> Result<UserModel, Errors>;

    /// Get user by id with row-level lock (SELECT ... FOR UPDATE)
    async fn get_user_by_id_for_update(&self, id: Uuid) -> Result<UserModel, Errors>;

    /// Get user by email (returns UserNotFound error if not found)
    async fn get_user_by_email(&self, email: String) -> Result<UserModel, Errors>;

    /// Get user by handle (returns UserNotFound error if not found)
    async fn get_user_by_handle(&self, handle: String) -> Result<UserModel, Errors>;

    /// Update user information
    async fn update_user(&self, user_id: Uuid, params: UserUpdateParams) -> Result<UserModel, Errors>;
}
