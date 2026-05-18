use axum::extract::State;
use kit_dto::action_logs::{ActionLogListResponse, GetActionLogsRequest};
use kit_dto::validator::query_validator::ValidatedQuery;
use kit_errors::errors::Errors;
use std::sync::Arc;

use crate::application::ActionLogApplicationService;

/// Application state for action log handlers
#[derive(Clone)]
pub struct ActionLogState {
    pub action_log_service: ActionLogApplicationService,
}

/// Get action logs with pagination and filtering
#[utoipa::path(
    get,
    path = "/v0/action-logs",
    params(GetActionLogsRequest),
    responses(
        (status = 200, description = "Action logs retrieved successfully", body = ActionLogListResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error"),
        (status = 500, description = "Internal Server Error - Database error")
    ),
    tag = "Action Logs"
)]
pub async fn get_action_logs(
    State(state): State<Arc<ActionLogState>>,
    ValidatedQuery(payload): ValidatedQuery<GetActionLogsRequest>,
) -> Result<ActionLogListResponse, Errors> {
    state.action_log_service.get_logs(payload).await
}
