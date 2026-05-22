use axum::extract::{Extension, Json, Path, Query, State};
use axum::response::IntoResponse;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::dto::request::{
    GetUserProfileByIdRequest, GetUserProfileRequest, UpdateMyProfileRequest,
};
use crate::api::dto::response::{
    CheckHandleAvailableResponse, PublicUserProfile, UserResponse,
};
use crate::application::profile_application_service::ProfileApplicationService;
use crate::infrastructure::repository::user_repository_impl::UserRepositoryImpl;
use kit_errors::errors::Errors;

/// Extension type used by the caller (kit-server) to inject the authenticated user_id.
/// The caller's session middleware sets this before the request reaches the handler.
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

/// Helper to create application service from database connection
fn create_profile_service(db: &DatabaseConnection) -> ProfileApplicationService {
    let user_repo = UserRepositoryImpl::new(Arc::new(db.clone()));
    let profile_service = crate::domain::service::profile_service::ProfileService::new(
        Arc::new(user_repo),
    );
    ProfileApplicationService::new(profile_service)
}

/// Get my profile (requires authentication)
#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "User profile retrieved successfully", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    tag = "User"
)]
pub async fn get_my_profile(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_profile_service(&db);
    let user = app_service.get_my_profile(user.user_id).await?;
    Ok(UserResponse::from(user))
}

/// Update my profile
#[utoipa::path(
    patch,
    path = "/user/me",
    request_body = UpdateMyProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "User"
)]
pub async fn update_my_profile(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<UpdateMyProfileRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_profile_service(&db);

    let bio_option = request.bio.map(Some);
    let user = app_service
        .update_my_profile(user.user_id, request.display_name, bio_option)
        .await?;
    Ok(UserResponse::from(user))
}

/// Get user profile by handle (public)
#[utoipa::path(
    get,
    path = "/users/profile",
    params(GetUserProfileRequest),
    responses(
        (status = 200, description = "User profile retrieved successfully", body = PublicUserProfile),
        (status = 404, description = "User not found"),
    ),
    tag = "User"
)]
pub async fn get_user_profile_by_handle(
    State(db): State<DatabaseConnection>,
    Query(request): Query<GetUserProfileRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_profile_service(&db);
    let user = app_service.get_user_profile_by_handle(request.handle).await?;
    Ok(PublicUserProfile::from(user))
}

/// Get user profile by id (public)
#[utoipa::path(
    get,
    path = "/users/profile/id",
    params(GetUserProfileByIdRequest),
    responses(
        (status = 200, description = "User profile retrieved successfully", body = PublicUserProfile),
        (status = 404, description = "User not found"),
    ),
    tag = "User"
)]
pub async fn get_user_profile_by_id(
    State(db): State<DatabaseConnection>,
    Query(request): Query<GetUserProfileByIdRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_profile_service(&db);
    let user = app_service.get_user_profile_by_id(request.user_id).await?;
    Ok(PublicUserProfile::from(user))
}

/// Check handle available
#[utoipa::path(
    get,
    path = "/users/handle/{handle}/available",
    params(
        ("handle" = String, Path, description = "User handle to check")
    ),
    responses(
        (status = 200, description = "Handle availability checked", body = CheckHandleAvailableResponse),
    ),
    tag = "User"
)]
pub async fn check_handle_available(
    State(db): State<DatabaseConnection>,
    Path(handle): Path<String>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_profile_service(&db);
    let available = app_service.check_handle_available(handle).await?;
    Ok(CheckHandleAvailableResponse { available })
}
