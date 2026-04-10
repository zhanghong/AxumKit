use axum::{extract::Extension, Json}; 
use kit_dto::permission::request::GetRolePermissionsRequest;
use kit_dto::permission::response::RolePermissionsResponse;
use kit_entity::common::Role;
use kit_errors::errors::Errors;

use crate::permission::PermissionService;
use crate::service::auth::session_types::SessionContext;
use crate::state::AppState;

#[utoipa::path(
    post, 
    path = "/v0/permission/role",
    summary = "Get role permissions",
    description = "Get permissions for a role",
    request_body = GetRolePermissionsRequest,
    responses(
        (status = 200, description = "Role permissions retrieved successfully", body = RolePermissionsResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("session" = [])
    )
)]
pub async fn get_role_permissions(
    Extension(session): Extension<Option<SessionContext>>,
    Extension(state): Extension<AppState>,
    Json(req): Json<GetRolePermissionsRequest>,
) -> Result<Json<RolePermissionsResponse>, Errors> {
    // Require admin role
    let _ctx = PermissionService::require_role(&state.db, session.as_ref(), Role::Admin).await?;
    
    // Get role permissions
    let permissions = PermissionService::get_role_permissions(&state.db, req.role).await?;
    
    Ok(Json(RolePermissionsResponse {
        permissions,
    }))
}
