use crate::application::PasswordApplicationService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ResetPasswordRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Reset password handler
///
/// Resets the user's password using a valid reset token.
#[utoipa::path(
    post,
    path = "/v0/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 204, description = "Password reset successfully"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or invalid/expired token"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Auth"
)]
pub async fn auth_reset_password(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(password_service): State<PasswordApplicationService>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    password_service
        .reset_password(&db, &redis, &payload.token, &payload.new_password)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
