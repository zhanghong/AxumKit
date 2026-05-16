use crate::application::SignupApplicationService;
use crate::infrastructure::adapter::WorkerClient;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ResendVerificationEmailRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::{ErrorResponse, Errors};
use redis::aio::ConnectionManager as RedisClient;

/// Resend verification email handler
///
/// Looks up an existing email and password signup that is still pending and resends the same verification token.
#[utoipa::path(
    post,
    path = "/v0/auth/resend-verification-email",
    summary = "Resend the pending signup verification email",
    description = "Looks up an existing email and password signup that is still pending and resends the same verification token with its remaining validity window. Returns 204 No Content even when no pending signup exists to avoid email enumeration.",
    request_body = ResendVerificationEmailRequest,
    responses(
        (status = 204, description = "Verification email was resent when a pending signup existed"),
        (status = 400, description = "Malformed JSON payload or validation error", body = ErrorResponse),
        (status = 500, description = "Unexpected Redis error", body = ErrorResponse),
        (status = 502, description = "Worker service rejected the verification email job or returned an invalid response", body = ErrorResponse),
        (status = 503, description = "Worker service could not be reached", body = ErrorResponse)
    ),
    tag = "Auth"
)]
pub async fn auth_resend_verification_email(
    State(redis): State<RedisClient>,
    State(worker): State<WorkerClient>,
    State(signup_service): State<SignupApplicationService>,
    ValidatedJson(payload): ValidatedJson<ResendVerificationEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    signup_service
        .resend_verification_email(&redis, &worker, &payload.email)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
