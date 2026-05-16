use std::sync::Arc;

use crate::domain::model::user::Model as UserModel;
use crate::domain::repository::user_repository::{UserRepository, UserUpdateParams};
use kit_errors::errors::Errors;
use uuid::Uuid;

/// Domain service for user profile operations
#[derive(Clone)]
pub struct ProfileService {
    user_repository: Arc<dyn UserRepository>,
}

impl ProfileService {
    /// Create a new profile service instance
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Get user profile by id
    pub async fn get_profile_by_id(&self, user_id: Uuid) -> Result<UserModel, Errors> {
        self.user_repository.get_user_by_id(user_id).await
    }

    /// Get user profile by handle
    pub async fn get_profile_by_handle(&self, handle: String) -> Result<UserModel, Errors> {
        self.user_repository.get_user_by_handle(handle).await
    }

    /// Find user profile by handle (returns Option)
    pub async fn find_profile_by_handle(&self, handle: String) -> Result<Option<UserModel>, Errors> {
        self.user_repository.find_user_by_handle(handle).await
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        bio: Option<Option<String>>,
    ) -> Result<UserModel, Errors> {
        let params = UserUpdateParams {
            display_name,
            bio,
            ..Default::default()
        };
        self.user_repository.update_user(user_id, params).await
    }

    /// Update user profile image
    pub async fn update_profile_image(
        &self,
        user_id: Uuid,
        image_url: Option<String>,
    ) -> Result<UserModel, Errors> {
        let params = UserUpdateParams {
            profile_image: Some(image_url),
            ..Default::default()
        };
        self.user_repository.update_user(user_id, params).await
    }

    /// Update user banner image
    pub async fn update_banner_image(
        &self,
        user_id: Uuid,
        image_url: Option<String>,
    ) -> Result<UserModel, Errors> {
        let params = UserUpdateParams {
            banner_image: Some(image_url),
            ..Default::default()
        };
        self.user_repository.update_user(user_id, params).await
    }

    /// Check if handle is available
    pub async fn is_handle_available(&self, handle: String) -> Result<bool, Errors> {
        let user = self.user_repository.find_user_by_handle(handle).await?;
        Ok(user.is_none())
    }
}
