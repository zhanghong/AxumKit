use crate::api::dto::request::ListModerationLogsRequest;
use crate::api::dto::response::{ListModerationLogsResponse, ModerationLogListItem};
use crate::domain::model::{ModerationAction, ModerationResourceType, Model as ModerationLogModel};
use crate::infrastructure::repository::{
    ModerationLogFilter, create_moderation_log, exists_newer_moderation_log,
    exists_older_moderation_log, find_moderation_logs,
};
use kit_dto::pagination::CursorDirection;
use kit_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Creates a new moderation log entry.
///
/// This is typically called by other domains when performing moderation actions.
pub async fn create_moderation_log_service(
    conn: &DatabaseConnection,
    action: ModerationAction,
    actor_id: Option<Uuid>,
    resource_type: ModerationResourceType,
    resource_id: Option<Uuid>,
    reason: String,
    metadata: Option<JsonValue>,
) -> ServiceResult<ModerationLogModel> {
    let log = create_moderation_log(
        conn,
        action,
        actor_id,
        resource_type,
        resource_id,
        reason,
        metadata,
    )
    .await?;

    Ok(log)
}

/// Lists moderation logs with pagination and filtering.
pub async fn list_moderation_logs_service(
    conn: &DatabaseConnection,
    payload: ListModerationLogsRequest,
) -> ServiceResult<ListModerationLogsResponse> {
    let limit = payload.limit;
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let filter = ModerationLogFilter {
        actor_id: payload.actor_id,
        resource_type: payload.resource_type,
        resource_id: payload.resource_id,
        actions: payload.actions,
    };

    let mut logs = find_moderation_logs(
        conn,
        &filter,
        payload.cursor_id,
        payload.cursor_direction,
        limit,
    )
    .await?;

    let (has_newer, has_older) = if logs.is_empty() {
        (false, false)
    } else {
        let first_id = logs.first().unwrap().id;
        let last_id = logs.last().unwrap().id;
        if is_newer {
            let has_newer = exists_newer_moderation_log(conn, &filter, last_id).await?;
            let has_older = exists_older_moderation_log(conn, &filter, first_id).await?;
            (has_newer, has_older)
        } else {
            let has_newer = exists_newer_moderation_log(conn, &filter, first_id).await?;
            let has_older = exists_older_moderation_log(conn, &filter, last_id).await?;
            (has_newer, has_older)
        }
    };

    if is_newer {
        logs.reverse();
    }

    let data: Vec<ModerationLogListItem> =
        logs.into_iter().map(ModerationLogListItem::from).collect();

    Ok(ListModerationLogsResponse {
        data,
        has_newer,
        has_older,
    })
}
