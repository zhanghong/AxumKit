//! Application services for authentication
//!
//! Application services coordinate domain services to fulfill use cases.
//! They are the entry point for business operations from the API layer.

pub mod login_application_service;
pub mod signup_application_service;
pub mod password_application_service;
pub mod email_application_service;
pub mod totp_application_service;

// Re-export application services
pub use login_application_service::{LoginApplicationService, LoginResult};
pub use signup_application_service::{SignupApplicationService, PendingEmailSignupData};
pub use password_application_service::{PasswordApplicationService, PasswordResetData};
pub use email_application_service::{EmailApplicationService, EmailChangeData};
pub use totp_application_service::TotpApplicationService;
