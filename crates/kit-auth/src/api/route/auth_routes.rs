use crate::api::handler::{
    change_email, change_password, complete_signup, confirm_email_change, forgot_password, login,
    logout, resend_verification_email, reset_password, signup, verify_email,
};
use crate::api::handler::totp::{
    disable as totp_disable, enable as totp_enable, regenerate_backup_codes as totp_regenerate_backup_codes,
    setup as totp_setup, status as totp_status, verify as totp_verify,
};
use axum::{Router, routing::get, routing::post};

/// Creates the authentication routes router.
///
/// This function returns a Router configured with all authentication endpoints.
/// The router is designed to be integrated into the main application router.
pub fn auth_routes() -> Router {
    Router::new()
        // Protected routes (authentication via extractors)
        .route("/auth/logout", post(logout))
        // TOTP protected routes (require session)
        .route("/auth/totp/status", get(totp_status))
        .route("/auth/totp/disable", post(totp_disable))
        // OAuth complete signup (pending token + handle)
        .route("/auth/complete-signup", post(complete_signup))
        // Email/password login route
        .route("/auth/login", post(login))
        // Email signup route (public, deferred creation)
        .route("/auth/signup", post(signup))
        // Email verification route (public)
        .route("/auth/verify-email", post(verify_email))
        // Resend verification email (public, email-based)
        .route("/auth/resend-verification-email", post(resend_verification_email))
        // TOTP setup/enable routes (require session)
        .route("/auth/totp/setup", post(totp_setup))
        .route("/auth/totp/enable", post(totp_enable))
        // TOTP verify route (public, for 2FA login)
        .route("/auth/totp/verify", post(totp_verify))
        // TOTP backup codes regeneration (require session)
        .route("/auth/totp/backup-codes/regenerate", post(totp_regenerate_backup_codes))
        // Password reset routes
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        // Password and email change routes (require session)
        .route("/auth/change-password", post(change_password))
        .route("/auth/change-email", post(change_email))
        .route("/auth/confirm-email-change", post(confirm_email_change))
}
