use crate::application::TotpApplicationService;
use crate::domain::model::SessionContext;
use axum::extract::State;
use axum::http::StatusCode;
use kit_dto::auth::request::TotpDisableRequest;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use sea_orm::DatabaseConnection;

/// TOTP disable handler
///
/// Disables TOTP for the authenticated user after verifying a valid code.
#[utoipa::path(
    post,
    path = "/v0/auth/totp/disable",
    request_body = TotpDisableRequest,
    responses(
        (status = 204, description = "TOTP disabled"),
        (status = 400, description = "Invalid TOTP code or TOTP not enabled"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_disable(
    State(db): State<DatabaseConnection>,
    State(totp_service): State<TotpApplicationService>,
    session: SessionContext,
    ValidatedJson(payload): ValidatedJson<TotpDisableRequest>,
) -> Result<StatusCode, Errors> {
    totp_service.disable(&db, session.user_id, &payload.code).await?;
    Ok(StatusCode::NO_CONTENT)
}
