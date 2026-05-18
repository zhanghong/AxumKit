use kit_constants::ActionLogAction;
use kit_dto::action_logs::{ActionLogListResponse, ActionLogResponse, GetActionLogsRequest};
use kit_dto::pagination::CursorDirection;
use kit_entity::common::ActionResourceType;
use kit_errors::errors::ServiceResult;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::model::ActionLog;
use crate::domain::repository::ActionLogFilter;
use crate::domain::service::ActionLogService;

/// Application service for action log operations
/// Coordinates domain operations and handles DTO conversions
#[derive(Clone)]
pub struct ActionLogApplicationService {
    domain_service: ActionLogService,
}

impl ActionLogApplicationService {
    /// Create a new action log application service instance
    pub fn new(domain_service: ActionLogService) -> Self {
        Self { domain_service }
    }

    /// Get action logs with pagination and filtering
    pub async fn get_logs(&self, request: GetActionLogsRequest) -> ServiceResult<ActionLogListResponse> {
        let limit = request.limit;
        let is_newer = request.cursor_direction == Some(CursorDirection::Newer);

        let filter = ActionLogFilter {
            actor_id: request.user_id,
            resource_id: request.resource_id,
            resource_type: request.resource_type,
            actions: request.actions,
        };

        let mut logs = self
            .domain_service
            .find_logs(&filter, request.cursor_id, request.cursor_direction, limit)
            .await?;

        // Calculate has_newer / has_older
        let (has_newer, has_older) = if logs.is_empty() {
            (false, false)
        } else {
            let first_id = logs.first().unwrap().id;
            let last_id = logs.last().unwrap().id;
            if is_newer {
                let has_newer = self.domain_service.exists_newer(&filter, last_id).await?;
                let has_older = self.domain_service.exists_older(&filter, first_id).await?;
                (has_newer, has_older)
            } else {
                let has_newer = self.domain_service.exists_newer(&filter, first_id).await?;
                let has_older = self.domain_service.exists_older(&filter, last_id).await?;
                (has_newer, has_older)
            }
        };

        // Reverse if Newer direction
        if is_newer {
            logs.reverse();
        }

        let data: Vec<ActionLogResponse> = logs.into_iter().map(ActionLogResponse::from).collect();

        Ok(ActionLogListResponse {
            data,
            has_newer,
            has_older,
        })
    }

    /// Create a new action log entry
    pub async fn create_log(
        &self,
        action: ActionLogAction,
        actor_id: Option<Uuid>,
        resource_type: ActionResourceType,
        resource_id: Option<Uuid>,
        summary: String,
        metadata: Option<JsonValue>,
    ) -> ServiceResult<ActionLogResponse> {
        let log = self
            .domain_service
            .create_log(action, actor_id, resource_type, resource_id, summary, metadata)
            .await?;
        Ok(ActionLogResponse::from(log))
    }
}

impl From<ActionLog> for ActionLogResponse {
    fn from(log: ActionLog) -> Self {
        Self {
            id: log.id,
            action: log.action,
            actor_id: log.actor_id,
            resource_type: log.resource_type,
            resource_id: log.resource_id,
            summary: log.summary,
            metadata: log.metadata,
            created_at: log.created_at,
        }
    }
}
