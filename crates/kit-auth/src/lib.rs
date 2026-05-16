//! Kit Auth - Authentication domain module for AxumKit
//!
//! This crate provides authentication functionality following DDD architecture.
//!
//! # Architecture Layers
//!
//! - **Domain Layer**: Core business logic (models, services, repository interfaces)
//! - **Application Layer**: Application services coordinating domain operations
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters

pub mod domain;
pub mod application;
pub mod api;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::model::{Session, SessionContext};
pub use domain::service::{SessionService, TotpService, TotpTempToken, verify_totp_code, generate_backup_codes};
pub use domain::repository::{UserRepository, SessionRepository};

// Re-export application services
pub use application::{
    LoginApplicationService,
    SignupApplicationService,
    PasswordApplicationService,
    TotpApplicationService,
    EmailApplicationService,
};

// Re-export handlers
pub use api::handler::{
    auth_login as login,
    auth_signup as signup,
    auth_logout as logout,
    auth_verify_email as verify_email,
    auth_resend_verification_email as resend_verification_email,
    auth_forgot_password as forgot_password,
    auth_reset_password as reset_password,
    auth_change_password as change_password,
    auth_change_email as change_email,
    auth_confirm_email_change as confirm_email_change,
    auth_complete_signup as complete_signup,
};

pub use api::handler::totp::{
    setup as totp_setup,
    enable as totp_enable,
    verify as totp_verify,
    totp_disable as totp_disable,
    totp_status as totp_status,
    totp_regenerate_backup_codes as totp_regenerate_backup_codes,
};

pub use api::route::auth_routes;
