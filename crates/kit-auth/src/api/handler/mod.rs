//! HTTP Handlers for authentication API
//!
//! This module contains all the HTTP request handlers for authentication endpoints.
//! Handlers are organized by functionality:
//!
//! - Core authentication: login, signup, logout
//! - Email verification: verify_email, resend_verification_email
//! - Password management: forgot_password, reset_password, change_password
//! - Email management: change_email, confirm_email_change
//! - OAuth: complete_signup
//! - TOTP (2FA): totp submodule

pub mod change_email;
pub mod change_password;
pub mod complete_signup;
pub mod confirm_email_change;
pub mod forgot_password;
pub mod login;
pub mod logout;
pub mod resend_verification_email;
pub mod reset_password;
pub mod signup;
pub mod totp;
pub mod verify_email;

// Re-export all handlers (with short aliases for route convenience)
pub use change_email::auth_change_email as change_email;
pub use change_password::auth_change_password as change_password;
pub use complete_signup::{auth_complete_signup as complete_signup, AnonymousUserContext};
pub use confirm_email_change::auth_confirm_email_change as confirm_email_change;
pub use forgot_password::auth_forgot_password as forgot_password;
pub use login::auth_login as login;
pub use logout::auth_logout as logout;
pub use resend_verification_email::auth_resend_verification_email as resend_verification_email;
pub use reset_password::auth_reset_password as reset_password;
pub use signup::auth_signup as signup;
pub use verify_email::auth_verify_email as verify_email;
