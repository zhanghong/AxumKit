use axum::{Router, routing::get};

use crate::api::handler::health_check::health_check;

/// Create health check routes
pub fn health_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/health-check", get(health_check))
}
