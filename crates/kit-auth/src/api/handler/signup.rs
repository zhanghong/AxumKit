use crate::application::SignupApplicationService;
use crate::infrastructure::adapter::WorkerClient;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use kit_dto::user::{CreateUserRequest, CreateUserResponse};
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::{ErrorResponse, Errors};
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Signup handler
///
/// Starts an email and password signup process.
/// Validates the requested email, handle, and password, stores a pending signup in Redis,
/// and queues a verification email. The user account is created only after the token is submitted.
#[utoipa::path(
    post,
    path = "/v0/auth/signup",
    summary = "Start an email and password signup",
    description = "Validates the requested email, handle, and password, stores a pending signup in Redis, and queues a verification email. The user account is created only after the token is submitted to POST /v0/auth/verify-email.",
    request_body = CreateUserRequest,
    responses(
        (status = 202, description = "Verification email queued and pending signup stored", body = CreateUserResponse),
        (status = 400, description = "Malformed JSON payload or validation error", body = ErrorResponse),
        (status = 409, description = "The email or handle is already in use or reserved by another pending signup", body = ErrorResponse),
        (status = 500, description = "Unexpected database or Redis error", body = ErrorResponse),
        (status = 502, description = "Worker service rejected the verification email job or returned an invalid response", body = ErrorResponse),
        (status = 503, description = "Worker service could not be reached", body = ErrorResponse),
    ),
    tag = "Auth"
)]
pub async fn auth_signup(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(worker): State<WorkerClient>,
    State(signup_service): State<SignupApplicationService>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = signup_service
        .signup(&db, &redis, &worker, payload)
        .await?;

    Ok((StatusCode::ACCEPTED, Json(response)))
}
