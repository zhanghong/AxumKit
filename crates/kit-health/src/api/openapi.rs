use utoipa::OpenApi;

/// Health API documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handler::health_check::health_check,
    ),
    tags(
        (name = "Health", description = "Health check endpoints")
    )
)]
pub struct HealthApiDoc;
