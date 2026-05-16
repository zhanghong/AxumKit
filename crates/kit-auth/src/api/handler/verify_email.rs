use crate::application::SignupApplicationService;
use crate::infrastructure::adapter::WorkerClient;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::VerifyEmailRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::{ErrorResponse, Errors};
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Verify email handler
///
/// Completes an email signup with a verification token.
/// Consumes the pending email verification token, creates the user account if the email and handle are still available.
#[utoipa::path(
    post,
    path = "/v0/auth/verify-email",
    summary = "Complete an email signup with a verification token",
    description = "Consumes the pending email verification token, creates the user account if the email and handle are still available, and then schedules background indexing. The token is only cleaned up after the database commit succeeds.",
    request_body = VerifyEmailRequest,
    responses(
        (status = 204, description = "Verification token accepted and the account was created"),
        (status = 400, description = "Malformed JSON payload, validation error, or invalid verification token", body = ErrorResponse),
        (status = 409, description = "The email or handle became unavailable before the account was created", body = ErrorResponse),
        (status = 500, description = "Unexpected database or Redis error", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_verify_email(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(signup_service): State<SignupApplicationService>,
    ValidatedJson(payload): ValidatedJson<VerifyEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    let _user_id = signup_service
        .verify_email(&db, &redis, &payload.token)
        .await?;

    // TODO: Trigger user indexing via worker client
    // worker_client::index_user(&worker, user_id).await.ok();

    Ok(StatusCode::NO_CONTENT)
}
