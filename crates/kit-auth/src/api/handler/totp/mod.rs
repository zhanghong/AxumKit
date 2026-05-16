pub mod disable;
pub mod enable;
pub mod regenerate_backup_codes;
pub mod setup;
pub mod status;
pub mod verify;

// Re-export all TOTP handlers (with short aliases for route convenience)
pub use disable::totp_disable as disable;
pub use enable::enable;
pub use regenerate_backup_codes::totp_regenerate_backup_codes as regenerate_backup_codes;
pub use setup::setup;
pub use status::totp_status as status;
pub use verify::verify;
