use crate::api::handler::search_users;
use crate::infrastructure::adapter::MeilisearchClient;
use axum::{Router, routing::get};

/// Creates the search routes router.
///
/// This function returns a Router configured with all search endpoints.
/// The router is designed to be integrated into the main application router.
pub fn search_routes() -> Router<MeilisearchClient> {
    // Public routes (no authentication required)
    Router::new().route("/search/users", get(search_users))
}
