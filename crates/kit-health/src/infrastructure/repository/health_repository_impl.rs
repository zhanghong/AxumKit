use async_trait::async_trait;

use crate::domain::repository::HealthRepository;

/// Implementation of health repository
#[derive(Clone, Default)]
pub struct HealthRepositoryImpl;

impl HealthRepositoryImpl {
    /// Create a new health repository instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl HealthRepository for HealthRepositoryImpl {
    async fn check_health(&self) -> bool {
        // Basic health check - always returns true for simple implementation
        // Can be extended to check database, cache, etc.
        true
    }
}
