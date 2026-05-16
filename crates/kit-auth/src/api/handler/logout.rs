use crate::application::LoginApplicationService;
use crate::api::dto::response::create_logout_response;
use crate::domain::model::SessionContext;
use axum::{extract::State, response::Response};
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;

/// Logout handler
///
/// Logs out the current user by invalidating their session.
#[utoipa::path(
    post,
    path = "/v0/auth/logout",
    responses(
        (status = 204, description = "Logout successful"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error - Redis error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_logout(
    State(redis): State<RedisClient>,
    State(_login_service): State<LoginApplicationService>,
    session_context: SessionContext,
) -> Result<Response, Errors> {
    // Delete session from Redis
    crate::domain::service::SessionService::delete_session(&redis, &session_context.session_id)
        .await?;

    create_logout_response()
}
