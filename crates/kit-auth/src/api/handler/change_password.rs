use crate::application::PasswordApplicationService;
use crate::domain::model::SessionContext;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ChangePasswordRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Change password handler
///
/// Changes the password for the currently authenticated user.
/// Invalidates all other sessions after password change.
#[utoipa::path(
    post,
    path = "/v0/auth/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 204, description = "Password changed successfully"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or incorrect password"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_change_password(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(password_service): State<PasswordApplicationService>,
    session: SessionContext,
    ValidatedJson(payload): ValidatedJson<ChangePasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    password_service
        .change_password(&db, &redis, session.user_id, &session.session_id, payload)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
