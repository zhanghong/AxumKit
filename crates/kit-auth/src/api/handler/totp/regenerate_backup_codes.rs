use crate::application::TotpApplicationService;
use crate::domain::model::SessionContext;
use axum::extract::State;
use kit_dto::auth::request::TotpRegenerateBackupCodesRequest;
use kit_dto::auth::response::TotpBackupCodesResponse;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use sea_orm::DatabaseConnection;

/// TOTP regenerate backup codes handler
///
/// Generates new backup codes for the authenticated user.
/// Requires TOTP verification to prevent unauthorized access.
#[utoipa::path(
    post,
    path = "/v0/auth/totp/backup-codes/regenerate",
    request_body = TotpRegenerateBackupCodesRequest,
    responses(
        (status = 200, description = "Backup codes regenerated", body = TotpBackupCodesResponse),
        (status = 400, description = "Invalid TOTP code or TOTP not enabled"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Auth - TOTP"
)]
pub async fn totp_regenerate_backup_codes(
    State(db): State<DatabaseConnection>,
    State(totp_service): State<TotpApplicationService>,
    session: SessionContext,
    ValidatedJson(payload): ValidatedJson<TotpRegenerateBackupCodesRequest>,
) -> Result<TotpBackupCodesResponse, Errors> {
    totp_service
        .regenerate_backup_codes(&db, session.user_id, &payload.code)
        .await
}
