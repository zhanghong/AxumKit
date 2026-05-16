//! Kit Health - Health check domain module for AxumKit
//!
//! This crate provides health check functionality following DDD architecture.
//!
//! # Architecture Layers
//!
//! - **Domain Layer**: Core business logic (services, repository interfaces)
//! - **Application Layer**: Application services coordinating domain operations
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters

pub mod domain;
pub mod application;
pub mod api;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::service::HealthService;
pub use domain::repository::HealthRepository;
pub use application::HealthApplicationService;
pub use api::handler::health_check;
pub use api::route::health_routes;
pub use infrastructure::repository::HealthRepositoryImpl;

/// Create a complete health service stack with default implementation
pub fn create_health_service() -> HealthApplicationService {
    let repository = std::sync::Arc::new(HealthRepositoryImpl::new());
    let domain_service = HealthService::new(repository);
    HealthApplicationService::new(domain_service)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let service = create_health_service();
        assert!(service.check_health().await);
    }
}
