use crate::application::EmailApplicationService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_dto::auth::request::ConfirmEmailChangeRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;

/// Confirm email change handler
///
/// Confirms an email change using the token sent to the new email address.
#[utoipa::path(
    post,
    path = "/v0/auth/confirm-email-change",
    request_body = ConfirmEmailChangeRequest,
    responses(
        (status = 204, description = "Email changed successfully"),
        (status = 400, description = "Bad request - Invalid or expired token"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Auth"
)]
pub async fn auth_confirm_email_change(
    State(db): State<DatabaseConnection>,
    State(redis): State<RedisClient>,
    State(email_service): State<EmailApplicationService>,
    ValidatedJson(payload): ValidatedJson<ConfirmEmailChangeRequest>,
) -> Result<impl IntoResponse, Errors> {
    email_service
        .confirm_email_change(&db, &redis, &payload.token)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
