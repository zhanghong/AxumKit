//! kit-moderation - Content moderation domain for AxumKit
//!
//! This crate provides content moderation functionality including:
//! - Moderation log management
//! - Audit trail for administrative actions
//!
//! ## Architecture
//!
//! The crate follows DDD (Domain-Driven Design) principles with the following layers:
//!
//! - **Domain Layer**: Core business logic, entities, and value objects
//! - **Application Layer**: Use cases and application services
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::model::{
    ModerationAction, ModerationResourceType, Model as ModerationLog, Entity as ModerationLogEntity,
};
pub use application::{
    create_moderation_log_service, list_moderation_logs_service,
};
pub use api::dto::{
    ListModerationLogsRequest, ListModerationLogsResponse, ModerationLogListItem,
};
pub use api::route::moderation_routes;
pub use api::openapi::ModerationOpenApi;
pub use infrastructure::repository::{
    ModerationLogFilter, create_moderation_log, find_moderation_logs,
    exists_newer_moderation_log, exists_older_moderation_log,
};
