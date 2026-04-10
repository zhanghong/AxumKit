use axum::{extract::Extension, Json}; 
use kit_dto::permission::request::RemovePermissionRequest;
use kit_entity::common::Role;
use kit_errors::errors::Errors;

use crate::permission::PermissionService;
use crate::service::auth::session_types::SessionContext;
use crate::state::AppState;

#[utoipa::path(
    post, 
    path = "/v0/permission/remove",
    summary = "Remove permission from role",
    description = "Remove a permission from a role",
    request_body = RemovePermissionRequest,
    responses(
        (status = 200, description = "Permission removed successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Permission not found"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("session" = [])
    )
)]
pub async fn remove_permission(
    Extension(session): Extension<Option<SessionContext>>,
    Extension(state): Extension<AppState>,
    Json(req): Json<RemovePermissionRequest>,
) -> Result<(), Errors> {
    // Require admin role
    let _ctx = PermissionService::require_role(&state.db, session.as_ref(), Role::Admin).await?;
    
    // Remove permission from role
    PermissionService::remove_permission_from_role(&state.db, req.role, req.permission).await?;
    
    Ok(())
}
