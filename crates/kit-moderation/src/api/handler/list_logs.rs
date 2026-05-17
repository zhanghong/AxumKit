use crate::api::dto::{ListModerationLogsRequest, ListModerationLogsResponse};
use crate::application::list_moderation_logs_service;
use axum::extract::State;
use kit_dto::validator::query_validator::ValidatedQuery;
use kit_errors::errors::Errors;
use sea_orm::DatabaseConnection;

#[utoipa::path(
    get,
    path = "/v0/moderation/logs",
    params(ListModerationLogsRequest),
    responses(
        (status = 200, description = "Moderation logs retrieved successfully", body = ListModerationLogsResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error"),
        (status = 500, description = "Internal Server Error - Database error")
    ),
    tag = "Moderation"
)]
pub async fn list_moderation_logs(
    State(db): State<DatabaseConnection>,
    ValidatedQuery(payload): ValidatedQuery<ListModerationLogsRequest>,
) -> Result<ListModerationLogsResponse, Errors> {
    list_moderation_logs_service(&db, payload).await
}
