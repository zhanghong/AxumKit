use axum::{extract::Extension, Json}; 
use kit_dto::permission::request::AddPermissionRequest;
use kit_entity::common::Role;
use kit_errors::errors::Errors;

use crate::permission::PermissionService;
use crate::service::auth::session_types::SessionContext;
use crate::state::AppState;

#[utoipa::path(
    post, 
    path = "/v0/permission/add",
    summary = "Add permission to role",
    description = "Add a permission to a role",
    request_body = AddPermissionRequest,
    responses(
        (status = 200, description = "Permission added successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("session" = [])
    )
)]
pub async fn add_permission(
    Extension(session): Extension<Option<SessionContext>>,
    Extension(state): Extension<AppState>,
    Json(req): Json<AddPermissionRequest>,
) -> Result<(), Errors> {
    // Require admin role
    let _ctx = PermissionService::require_role(&state.db, session.as_ref(), Role::Admin).await?;
    
    // Add permission to role
    PermissionService::add_permission_to_role(&state.db, req.role, req.permission).await?;
    
    Ok(())
}
