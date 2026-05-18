use axum::{Router, routing::get};
use std::sync::Arc;

use crate::api::handler::get_action_logs;
use crate::api::handler::action_log::ActionLogState;

/// Create action log routes
pub fn action_log_routes(state: Arc<ActionLogState>) -> Router {
    Router::new()
        .route("/action-logs", get(get_action_logs))
        .with_state(state)
}
