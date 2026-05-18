use std::sync::Arc;

use kit_constants::ActionLogAction;
use kit_entity::common::ActionResourceType;
use kit_errors::errors::Errors;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::model::{ActionLog, ActionLogId};
use crate::domain::repository::{ActionLogFilter, ActionLogRepository};

/// Domain service for action log operations
#[derive(Clone)]
pub struct ActionLogService {
    repository: Arc<dyn ActionLogRepository>,
}

impl ActionLogService {
    /// Create a new action log service instance
    pub fn new(repository: Arc<dyn ActionLogRepository>) -> Self {
        Self { repository }
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
    ) -> Result<ActionLog, Errors> {
        self.repository
            .create_log(action, actor_id, resource_type, resource_id, summary, metadata)
            .await
    }

    /// Find action logs with filtering and pagination
    pub async fn find_logs(
        &self,
        filter: &ActionLogFilter,
        cursor_id: Option<Uuid>,
        cursor_direction: Option<kit_dto::pagination::CursorDirection>,
        limit: u64,
    ) -> Result<Vec<ActionLog>, Errors> {
        self.repository
            .find_logs(filter, cursor_id, cursor_direction, limit)
            .await
    }

    /// Check if newer logs exist
    pub async fn exists_newer(
        &self,
        filter: &ActionLogFilter,
        cursor_id: Uuid,
    ) -> Result<bool, Errors> {
        self.repository.exists_newer(filter, cursor_id).await
    }

    /// Check if older logs exist
    pub async fn exists_older(
        &self,
        filter: &ActionLogFilter,
        cursor_id: Uuid,
    ) -> Result<bool, Errors> {
        self.repository.exists_older(filter, cursor_id).await
    }

    /// Find a single action log by ID
    pub async fn find_by_id(&self, id: ActionLogId) -> Result<Option<ActionLog>, Errors> {
        self.repository.find_by_id(id).await
    }
}
