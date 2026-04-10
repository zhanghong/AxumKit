use axum::{extract::Extension, middleware::Next, response::Response, http::Request, body::Body};
use kit_entity::common::{Permission, Role};
use kit_errors::errors::Errors;
use sea_orm::DatabaseConnection;

use crate::permission::PermissionService;
use crate::service::auth::session_types::SessionContext;

/// Middleware to require a specific role
pub async fn require_role(
    Extension(session): Extension<Option<SessionContext>>,
    Extension(conn): Extension<DatabaseConnection>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Errors> {
    let role = req
        .extensions()
        .get::<Role>()
        .ok_or(Errors::UserPermissionInsufficient)?;

    PermissionService::require_role(&conn, session.as_ref(), *role).await?;

    Ok(next.run(req).await)
}

/// Middleware to require a specific permission
pub async fn require_permission(
    Extension(session): Extension<Option<SessionContext>>,
    Extension(conn): Extension<DatabaseConnection>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Errors> {
    let permission = req
        .extensions()
        .get::<Permission>()
        .ok_or(Errors::UserPermissionInsufficient)?;

    PermissionService::require_permission(&conn, session.as_ref(), *permission).await?;

    Ok(next.run(req).await)
}

/// Helper function to add required role to request extensions
pub fn with_required_role<B>(mut req: Request<B>, role: Role) -> Request<B> {
    req.extensions_mut().insert(role);
    req
}

/// Helper function to add required permission to request extensions
pub fn with_required_permission<B>(mut req: Request<B>, permission: Permission) -> Request<B> {
    req.extensions_mut().insert(permission);
    req
}
