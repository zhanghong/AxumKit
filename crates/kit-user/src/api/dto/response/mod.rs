use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::model::role::Role;

/// Full user profile response (with email, for "my profile")
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub handle: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Public user profile response (without email)
#[derive(Debug, Serialize, ToSchema)]
pub struct PublicUserProfile {
    pub id: Uuid,
    pub handle: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl IntoResponse for PublicUserProfile {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Ban user response
#[derive(Debug, Serialize, ToSchema)]
pub struct BanUserResponse {
    pub user_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
}

impl IntoResponse for BanUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Unban user response
#[derive(Debug, Serialize, ToSchema)]
pub struct UnbanUserResponse {
    pub user_id: Uuid,
}

impl IntoResponse for UnbanUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Grant role response
#[derive(Debug, Serialize, ToSchema)]
pub struct GrantRoleResponse {
    pub user_id: Uuid,
    pub role: Role,
    pub expires_at: Option<DateTime<Utc>>,
}

impl IntoResponse for GrantRoleResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Revoke role response
#[derive(Debug, Serialize, ToSchema)]
pub struct RevokeRoleResponse {
    pub user_id: Uuid,
    pub role: Role,
}

impl IntoResponse for RevokeRoleResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Check handle available response
#[derive(Debug, Serialize, ToSchema)]
pub struct CheckHandleAvailableResponse {
    pub available: bool,
}

impl IntoResponse for CheckHandleAvailableResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Upload user image response
#[derive(Debug, Serialize, ToSchema)]
pub struct UploadUserImageResponse {
    pub image_url: String,
}

impl IntoResponse for UploadUserImageResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}

impl From<crate::domain::model::user::Model> for UserResponse {
    fn from(user: crate::domain::model::user::Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            handle: user.handle,
            display_name: user.display_name,
            bio: user.bio,
            profile_image: user.profile_image,
            banner_image: user.banner_image,
            is_verified: user.verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}

impl From<crate::domain::model::user::Model> for PublicUserProfile {
    fn from(user: crate::domain::model::user::Model) -> Self {
        Self {
            id: user.id,
            handle: user.handle,
            display_name: user.display_name,
            bio: user.bio,
            profile_image: user.profile_image,
            banner_image: user.banner_image,
            is_verified: user.verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}
