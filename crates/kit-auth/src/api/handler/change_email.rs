use crate::application::EmailApplicationService;
use crate::infrastructure::adapter::WorkerClient;
use crate::domain::model::SessionContext;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ChangeEmailRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Change email handler
///
/// Initiates an email change for the currently authenticated user.
/// Sends a verification email to the new address.
#[utoipa::path(
    post,
    path = "/v0/auth/change-email",
    request_body = ChangeEmailRequest,
    responses(
        (status = 204, description = "Verification email sent to new address"),
        (status = 400, description = "Bad request - Invalid JSON, validation error, or incorrect password"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn auth_change_email(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(worker): State<WorkerClient>,
    State(email_service): State<EmailApplicationService>,
    session: SessionContext,
    ValidatedJson(payload): ValidatedJson<ChangeEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    email_service
        .change_email(&db, &redis, &worker, session.user_id, payload)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
