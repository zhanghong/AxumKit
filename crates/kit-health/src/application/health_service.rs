use crate::domain::service::HealthService;

/// Application service for health check operations
#[derive(Clone)]
pub struct HealthApplicationService {
    domain_service: HealthService,
}

impl HealthApplicationService {
    /// Create a new health application service instance
    pub fn new(domain_service: HealthService) -> Self {
        Self { domain_service }
    }

    /// Perform health check
    pub async fn check_health(&self) -> bool {
        self.domain_service.is_healthy().await
    }
}
