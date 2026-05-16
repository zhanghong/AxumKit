use crate::domain::repository::health_repository::HealthRepository;
use std::sync::Arc;

/// Domain service for health check operations
#[derive(Clone)]
pub struct HealthService {
    repository: Arc<dyn HealthRepository>,
}

impl HealthService {
    /// Create a new health service instance
    pub fn new(repository: Arc<dyn HealthRepository>) -> Self {
        Self { repository }
    }

    /// Check if the service is healthy
    pub async fn is_healthy(&self) -> bool {
        self.repository.check_health().await
    }
}
