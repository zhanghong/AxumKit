use axum::http::StatusCode;
use axum::response::IntoResponse;
use kit_errors::errors::Errors;

use crate::application::HealthApplicationService;

/// Health check handler
#[utoipa::path(
    get,
    path = "/health-check",
    responses(
        (status = 204, description = "Service is healthy and running"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Health"
)]
pub async fn health_check() -> Result<impl IntoResponse, Errors> {
    Ok(StatusCode::NO_CONTENT)
}

/// Health check handler with service
pub async fn health_check_with_service(
    service: HealthApplicationService,
) -> Result<impl IntoResponse, Errors> {
    if service.check_health().await {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(Errors::SysInternalError("Service is unhealthy".to_string()))
    }
}
