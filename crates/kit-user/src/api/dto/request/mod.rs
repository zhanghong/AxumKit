use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::model::role::Role;

/// Request payload for update my profile request.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UpdateMyProfileRequest {
    /// Display name shown in the UI.
    #[schema(min_length = 1, max_length = 50, example = "John Doe")]
    #[validate(length(min = 1, max = 50))]
    pub display_name: Option<String>,
    #[validate(length(max = 500))]
    pub bio: Option<String>,
}

/// Request payload for get user profile by handle.
#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct GetUserProfileRequest {
    #[validate(length(min = 3, max = 20))]
    pub handle: String,
}

/// Request payload for get user profile by id.
#[derive(Debug, Deserialize, ToSchema, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct GetUserProfileByIdRequest {
    pub user_id: Uuid,
}

/// Request payload for ban user request.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct BanUserRequest {
    pub user_id: Uuid,
    /// Ban expiration time (None = permanent ban)
    pub expires_at: Option<DateTime<Utc>>,
    #[validate(length(min = 1, max = 1000))]
    pub reason: String,
}

/// Request payload for unban user request.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UnbanUserRequest {
    pub user_id: Uuid,
}

/// Request payload for grant role request.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct GrantRoleRequest {
    pub user_id: Uuid,
    pub role: Role,
    /// Role expiration time (None = permanent)
    pub expires_at: Option<DateTime<Utc>>,
    #[validate(length(min = 1, max = 1000))]
    pub reason: String,
}

/// Request payload for revoke role request.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RevokeRoleRequest {
    pub user_id: Uuid,
    pub role: Role,
}

/// Request payload for check handle available.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CheckHandleAvailableRequest {
    #[validate(length(min = 3, max = 20))]
    pub handle: String,
}
