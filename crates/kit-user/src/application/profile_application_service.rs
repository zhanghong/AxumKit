use crate::domain::model::user::Model as UserModel;
use crate::domain::service::profile_service::ProfileService;
use kit_errors::errors::Errors;
use uuid::Uuid;

/// Application service for user profile operations
/// Coordinates domain services to fulfill use cases
#[derive(Clone)]
pub struct ProfileApplicationService {
    profile_service: ProfileService,
}

impl ProfileApplicationService {
    /// Create a new profile application service instance
    pub fn new(profile_service: ProfileService) -> Self {
        Self { profile_service }
    }

    /// Get my profile (full profile with email)
    pub async fn get_my_profile(&self, user_id: Uuid) -> Result<UserModel, Errors> {
        self.profile_service.get_profile_by_id(user_id).await
    }

    /// Get user profile by handle (public profile)
    pub async fn get_user_profile_by_handle(&self, handle: String) -> Result<UserModel, Errors> {
        self.profile_service.get_profile_by_handle(handle).await
    }

    /// Get user profile by id (public profile)
    pub async fn get_user_profile_by_id(&self, user_id: Uuid) -> Result<UserModel, Errors> {
        self.profile_service.get_profile_by_id(user_id).await
    }

    /// Update my profile
    pub async fn update_my_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        bio: Option<Option<String>>,
    ) -> Result<UserModel, Errors> {
        self.profile_service
            .update_profile(user_id, display_name, bio)
            .await
    }

    /// Upload profile image
    pub async fn upload_profile_image(
        &self,
        user_id: Uuid,
        image_url: String,
    ) -> Result<UserModel, Errors> {
        self.profile_service
            .update_profile_image(user_id, Some(image_url))
            .await
    }

    /// Delete profile image
    pub async fn delete_profile_image(&self, user_id: Uuid) -> Result<UserModel, Errors> {
        self.profile_service.update_profile_image(user_id, None).await
    }

    /// Upload banner image
    pub async fn upload_banner_image(
        &self,
        user_id: Uuid,
        image_url: String,
    ) -> Result<UserModel, Errors> {
        self.profile_service
            .update_banner_image(user_id, Some(image_url))
            .await
    }

    /// Delete banner image
    pub async fn delete_banner_image(&self, user_id: Uuid) -> Result<UserModel, Errors> {
        self.profile_service.update_banner_image(user_id, None).await
    }

    /// Check if handle is available
    pub async fn check_handle_available(&self, handle: String) -> Result<bool, Errors> {
        self.profile_service.is_handle_available(handle).await
    }
}
