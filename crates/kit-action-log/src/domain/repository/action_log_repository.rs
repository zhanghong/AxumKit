use async_trait::async_trait;
use kit_constants::ActionLogAction;
use kit_entity::common::ActionResourceType;
use kit_errors::errors::Errors;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::model::{ActionLog, ActionLogId};

/// Filter for querying action logs
#[derive(Debug, Default, Clone)]
pub struct ActionLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_id: Option<Uuid>,
    pub resource_type: Option<ActionResourceType>,
    pub actions: Option<Vec<ActionLogAction>>,
}

/// Repository interface for action log operations
#[async_trait]
pub trait ActionLogRepository: Send + Sync {
    /// Find action logs with optional filtering and pagination
    async fn find_logs(
        &self,
        filter: &ActionLogFilter,
        cursor_id: Option<Uuid>,
        cursor_direction: Option<kit_dto::pagination::CursorDirection>,
        limit: u64,
    ) -> Result<Vec<ActionLog>, Errors>;

    /// Create a new action log
    async fn create_log(
        &self,
        action: ActionLogAction,
        actor_id: Option<Uuid>,
        resource_type: ActionResourceType,
        resource_id: Option<Uuid>,
        summary: String,
        metadata: Option<JsonValue>,
    ) -> Result<ActionLog, Errors>;

    /// Check if there exists a newer action log than the given cursor
    async fn exists_newer(&self, filter: &ActionLogFilter, cursor_id: Uuid) -> Result<bool, Errors>;

    /// Check if there exists an older action log than the given cursor
    async fn exists_older(&self, filter: &ActionLogFilter, cursor_id: Uuid) -> Result<bool, Errors>;

    /// Find a single action log by ID
    async fn find_by_id(&self, id: ActionLogId) -> Result<Option<ActionLog>, Errors>;
}
