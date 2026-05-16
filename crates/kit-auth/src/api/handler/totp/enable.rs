//! TOTP enable handler.

use axum::response::Response;

/// Handler for TOTP enable endpoint.
///
/// POST /auth/totp/enable
///
/// Enables TOTP for the authenticated user after verifying the setup code.
pub async fn enable() -> Response {
    // TODO: Implement TOTP enable logic using TotpApplicationService
    todo!("TOTP enable handler not yet implemented")
}
