use crate::api::dto::request::{
    ChangeEmailRequest, ChangePasswordRequest, CompleteSignupRequest, ConfirmEmailChangeRequest,
    ForgotPasswordRequest, LoginRequest, ResendVerificationEmailRequest, ResetPasswordRequest,
    TotpDisableRequest, TotpEnableRequest, TotpRegenerateBackupCodesRequest, TotpVerifyRequest,
    VerifyEmailRequest,
};
use crate::api::dto::response::{
    LoginResponse, LogoutResponse, TotpBackupCodesResponse, TotpEnableResponse, TotpRequiredResponse,
    TotpSetupResponse, TotpStatusResponse,
};
use utoipa::OpenApi;

/// OpenAPI documentation for authentication endpoints.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handler::signup::signup,
        crate::api::handler::login::login,
        crate::api::handler::logout::logout,
        crate::api::handler::forgot_password::forgot_password,
        crate::api::handler::reset_password::reset_password,
        crate::api::handler::complete_signup::complete_signup,
        crate::api::handler::totp::setup::setup,
        crate::api::handler::totp::verify::verify,
        crate::api::handler::totp::enable::enable,
        crate::api::handler::totp::disable::disable,
        crate::api::handler::totp::status::status,
        crate::api::handler::totp::regenerate_backup_codes::regenerate_backup_codes,
        crate::api::handler::verify_email::verify_email,
        crate::api::handler::resend_verification_email::resend_verification_email,
        crate::api::handler::change_password::change_password,
        crate::api::handler::change_email::change_email,
        crate::api::handler::confirm_email_change::confirm_email_change,
    ),
    components(
        schemas(
            LoginRequest,
            LoginResponse,
            VerifyEmailRequest,
            ResendVerificationEmailRequest,
            ForgotPasswordRequest,
            ResetPasswordRequest,
            CompleteSignupRequest,
            TotpVerifyRequest,
            TotpEnableRequest,
            TotpDisableRequest,
            TotpRegenerateBackupCodesRequest,
            TotpSetupResponse,
            TotpStatusResponse,
            TotpEnableResponse,
            TotpBackupCodesResponse,
            TotpRequiredResponse,
            ChangePasswordRequest,
            ChangeEmailRequest,
            ConfirmEmailChangeRequest,
            LogoutResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints")
    )
)]
pub struct AuthApiDoc;
