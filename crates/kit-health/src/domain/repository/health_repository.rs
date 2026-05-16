use async_trait::async_trait;

/// Repository interface for health check operations
#[async_trait]
pub trait HealthRepository: Send + Sync {
    /// Check the health status of the service
    async fn check_health(&self) -> bool;
}
