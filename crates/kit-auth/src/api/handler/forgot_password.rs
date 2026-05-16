use crate::application::PasswordApplicationService;
use crate::infrastructure::adapter::WorkerClient;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ForgotPasswordRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Forgot password handler
///
/// Sends a password reset email to the user if the account exists.
#[utoipa::path(
    post,
    path = "/v0/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 204, description = "Password reset email sent if account exists"),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Auth"
)]
pub async fn auth_forgot_password(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(worker): State<WorkerClient>,
    State(password_service): State<PasswordApplicationService>,
    ValidatedJson(payload): ValidatedJson<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    password_service
        .forgot_password(&db, &redis, &worker, &payload.email)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
