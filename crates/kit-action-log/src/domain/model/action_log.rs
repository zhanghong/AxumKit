use chrono::{DateTime, Utc};
use kit_constants::ActionLogAction;
use kit_entity::common::ActionResourceType;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Action Log ID type
pub type ActionLogId = Uuid;

/// Action Log domain model
/// This is the domain representation of an action log entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionLog {
    pub id: ActionLogId,
    pub action: String,
    pub actor_id: Option<Uuid>,
    pub resource_type: ActionResourceType,
    pub resource_id: Option<Uuid>,
    pub summary: String,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

impl ActionLog {
    /// Create a new action log
    pub fn new(
        action: ActionLogAction,
        actor_id: Option<Uuid>,
        resource_type: ActionResourceType,
        resource_id: Option<Uuid>,
        summary: String,
        metadata: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            action: action.as_str().to_string(),
            actor_id,
            resource_type,
            resource_id,
            summary,
            metadata,
            created_at: Utc::now(),
        }
    }

    /// Get the action as ActionLogAction enum
    pub fn action_enum(&self) -> Option<ActionLogAction> {
        self.action.parse().ok()
    }
}

impl From<kit_entity::action_logs::Model> for ActionLog {
    fn from(model: kit_entity::action_logs::Model) -> Self {
        Self {
            id: model.id,
            action: model.action,
            actor_id: model.actor_id,
            resource_type: model.resource_type,
            resource_id: model.resource_id,
            summary: model.summary,
            metadata: model.metadata,
            created_at: model.created_at,
        }
    }
}
