//! Kit Action Log - Action log domain module for AxumKit
//!
//! This crate provides action log functionality following DDD architecture.
//!
//! # Architecture Layers
//!
//! - **Domain Layer**: Core business logic (models, services, repository interfaces)
//! - **Application Layer**: Application services coordinating domain operations
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters
//!
//! # Example Usage
//!
//! ```rust
//! use kit_action_log::{
//!     create_action_log_service,
//!     ActionLogApplicationService,
//!     ActionLogState,
//!     action_log_routes,
//! };
//! use sea_orm::DatabaseConnection;
//! use std::sync::Arc;
//!
//! async fn setup(db: DatabaseConnection) {
//!     // Create the service stack
//!     let service = create_action_log_service(db);
//!     
//!     // Create state for handlers
//!     let state = Arc::new(ActionLogState { action_log_service: service });
//!     
//!     // Create routes
//!     let routes = action_log_routes(state);
//! }
//! ```

pub mod domain;
pub mod application;
pub mod api;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::model::{ActionLog, ActionLogId};
pub use domain::repository::{ActionLogFilter, ActionLogRepository};
pub use domain::service::ActionLogService;
pub use application::ActionLogApplicationService;
pub use api::handler::action_log::ActionLogState;
pub use api::handler::get_action_logs;
pub use api::route::action_log_routes;
pub use infrastructure::repository::ActionLogRepositoryImpl;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Create a complete action log service stack with default implementation
/// 
/// # Arguments
/// 
/// * `db` - Database connection for repository implementation
/// 
/// # Returns
/// 
/// A fully configured `ActionLogApplicationService` ready for use
pub fn create_action_log_service(db: DatabaseConnection) -> ActionLogApplicationService {
    let repository = Arc::new(ActionLogRepositoryImpl::new(db));
    let domain_service = ActionLogService::new(repository);
    ActionLogApplicationService::new(domain_service)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_log_creation() {
        use kit_constants::ActionLogAction;
        use kit_entity::common::ActionResourceType;

        let log = ActionLog::new(
            ActionLogAction::UserCreate,
            None,
            ActionResourceType::User,
            None,
            "Test user created".to_string(),
            None,
        );

        assert_eq!(log.action, "user:create");
        assert_eq!(log.resource_type, ActionResourceType::User);
    }
}
