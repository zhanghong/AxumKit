use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    /// Service status
    pub status: String,
    /// Timestamp of the health check
    pub timestamp: String,
}

impl HealthCheckResponse {
    /// Create a new healthy response
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
