use axum::extract::{Json, State};
use axum::response::IntoResponse;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::api::dto::request::{BanUserRequest, GrantRoleRequest, RevokeRoleRequest, UnbanUserRequest};
use crate::api::dto::response::{BanUserResponse, GrantRoleResponse, RevokeRoleResponse, UnbanUserResponse};
use crate::application::user_management_application_service::UserManagementApplicationService;
use crate::infrastructure::repository::user_ban_repository_impl::UserBanRepositoryImpl;
use crate::infrastructure::repository::user_role_repository_impl::UserRoleRepositoryImpl;
use kit_errors::errors::Errors;

/// Helper to create user management application service from database connection
fn create_management_service(db: &DatabaseConnection) -> UserManagementApplicationService {
    let user_ban_repo = UserBanRepositoryImpl::new(Arc::new(db.clone()));
    let user_role_repo = UserRoleRepositoryImpl::new(Arc::new(db.clone()));

    let user_management_service = crate::domain::service::user_management_service::UserManagementService::new(
        Arc::new(user_ban_repo),
        Arc::new(user_role_repo),
    );
    UserManagementApplicationService::new(user_management_service)
}

/// Ban user
#[utoipa::path(
    post,
    path = "/users/ban",
    request_body = BanUserRequest,
    responses(
        (status = 200, description = "User banned successfully", body = BanUserResponse),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "User Management"
)]
pub async fn ban_user(
    State(db): State<DatabaseConnection>,
    Json(request): Json<BanUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_management_service(&db);

    let ban = app_service.ban_user(request.user_id, request.expires_at).await?;
    Ok(BanUserResponse {
        user_id: ban.user_id,
        expires_at: ban.expires_at,
    })
}

/// Unban user
#[utoipa::path(
    post,
    path = "/users/unban",
    request_body = UnbanUserRequest,
    responses(
        (status = 200, description = "User unbanned successfully", body = UnbanUserResponse),
        (status = 403, description = "Forbidden"),
    ),
    tag = "User Management"
)]
pub async fn unban_user(
    State(db): State<DatabaseConnection>,
    Json(request): Json<UnbanUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_management_service(&db);

    let _ = app_service.unban_user(request.user_id).await?;
    Ok(UnbanUserResponse {
        user_id: request.user_id,
    })
}

/// Grant role to user
#[utoipa::path(
    post,
    path = "/users/roles/grant",
    request_body = GrantRoleRequest,
    responses(
        (status = 200, description = "Role granted successfully", body = GrantRoleResponse),
        (status = 400, description = "Invalid request"),
        (status = 403, description = "Forbidden"),
    ),
    tag = "User Management"
)]
pub async fn grant_role(
    State(db): State<DatabaseConnection>,
    Json(request): Json<GrantRoleRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_management_service(&db);

    let role = app_service
        .grant_role(request.user_id, request.role, request.expires_at)
        .await?;
    Ok(GrantRoleResponse {
        user_id: role.user_id,
        role: role.role,
        expires_at: role.expires_at,
    })
}

/// Revoke role from user
#[utoipa::path(
    post,
    path = "/users/roles/revoke",
    request_body = RevokeRoleRequest,
    responses(
        (status = 200, description = "Role revoked successfully", body = RevokeRoleResponse),
        (status = 403, description = "Forbidden"),
    ),
    tag = "User Management"
)]
pub async fn revoke_role(
    State(db): State<DatabaseConnection>,
    Json(request): Json<RevokeRoleRequest>,
) -> Result<impl IntoResponse, Errors> {
    let app_service = create_management_service(&db);

    let _ = app_service.revoke_role(request.user_id, request.role).await?;
    Ok(RevokeRoleResponse {
        user_id: request.user_id,
        role: request.role,
    })
}
