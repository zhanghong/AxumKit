//! Domain services for authentication.
//!
//! Domain services encapsulate business logic that doesn't naturally belong to any specific entity.

pub mod session_service;
pub mod totp_service;

pub use session_service::SessionService;
pub use totp_service::{TotpService, TotpTempToken, verify_totp_code, generate_backup_codes};
