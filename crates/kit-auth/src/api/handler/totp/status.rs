use crate::application::TotpApplicationService;
use crate::domain::model::SessionContext;
use axum::extract::State;
use kit_dto::auth::response::TotpStatusResponse;
use kit_errors::errors::Errors;
use sea_orm::DatabaseConnection;

/// TOTP status handler
///
/// Returns the TOTP status for the authenticated user.
#[utoipa::path(
    get,
    path = "/v0/auth/totp/status",
    responses(
        (status = 200, description = "TOTP status", body = TotpStatusResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_status(
    State(db): State<DatabaseConnection>,
    State(totp_service): State<TotpApplicationService>,
    session: SessionContext,
) -> Result<TotpStatusResponse, Errors> {
    totp_service.status(&db, session.user_id).await
}
