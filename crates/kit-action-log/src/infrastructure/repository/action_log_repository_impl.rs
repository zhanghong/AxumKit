use async_trait::async_trait;
use kit_constants::ActionLogAction;
use kit_entity::action_logs::{
    ActiveModel as ActionLogActiveModel, Column as ActionLogColumn, Entity as ActionLogEntity,
    Model as ActionLogModel,
};
use kit_entity::common::ActionResourceType;
use kit_errors::errors::Errors;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::model::{ActionLog, ActionLogId};
use crate::domain::repository::{ActionLogFilter, ActionLogRepository};

/// Repository implementation for action logs using SeaORM
#[derive(Clone)]
pub struct ActionLogRepositoryImpl<C: ConnectionTrait + Clone + Send + Sync> {
    db: C,
}

impl<C: ConnectionTrait + Clone + Send + Sync> ActionLogRepositoryImpl<C> {
    /// Create a new action log repository instance
    pub fn new(db: C) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<C: ConnectionTrait + Clone + Send + Sync> ActionLogRepository for ActionLogRepositoryImpl<C> {
    async fn find_logs(
        &self,
        filter: &ActionLogFilter,
        cursor_id: Option<Uuid>,
        cursor_direction: Option<kit_dto::pagination::CursorDirection>,
        limit: u64,
    ) -> Result<Vec<ActionLog>, Errors> {
        let mut query = apply_action_log_filter(ActionLogEntity::find(), filter);

        // Apply cursor-based filtering (UUIDv7 is time-sortable)
        if let Some(id) = cursor_id {
            let direction = cursor_direction.unwrap_or(kit_dto::pagination::CursorDirection::Older);
            query = match direction {
                kit_dto::pagination::CursorDirection::Older => query
                    .filter(ActionLogColumn::Id.lt(id))
                    .order_by_desc(ActionLogColumn::Id),
                kit_dto::pagination::CursorDirection::Newer => query
                    .filter(ActionLogColumn::Id.gt(id))
                    .order_by_asc(ActionLogColumn::Id),
            };
        } else {
            query = query.order_by_desc(ActionLogColumn::Id);
        }

        let models = query.limit(limit).all(&self.db).await?;
        let logs = models.into_iter().map(ActionLog::from).collect();

        Ok(logs)
    }

    async fn create_log(
        &self,
        action: ActionLogAction,
        actor_id: Option<Uuid>,
        resource_type: ActionResourceType,
        resource_id: Option<Uuid>,
        summary: String,
        metadata: Option<JsonValue>,
    ) -> Result<ActionLog, Errors> {
        let log = ActionLogActiveModel {
            id: Default::default(),
            action: Set(action.as_str().to_string()),
            actor_id: Set(actor_id),
            resource_type: Set(resource_type),
            resource_id: Set(resource_id),
            summary: Set(summary),
            metadata: Set(metadata),
            created_at: Default::default(), // Uses database default now()
        };

        let model = log.insert(&self.db).await?;
        Ok(ActionLog::from(model))
    }

    async fn exists_newer(&self, filter: &ActionLogFilter, cursor_id: Uuid) -> Result<bool, Errors> {
        let query = apply_action_log_filter(
            ActionLogEntity::find().filter(ActionLogColumn::Id.gt(cursor_id)),
            filter,
        );

        let count = query.limit(1).count(&self.db).await?;
        Ok(count > 0)
    }

    async fn exists_older(&self, filter: &ActionLogFilter, cursor_id: Uuid) -> Result<bool, Errors> {
        let query = apply_action_log_filter(
            ActionLogEntity::find().filter(ActionLogColumn::Id.lt(cursor_id)),
            filter,
        );

        let count = query.limit(1).count(&self.db).await?;
        Ok(count > 0)
    }

    async fn find_by_id(&self, id: ActionLogId) -> Result<Option<ActionLog>, Errors> {
        let result = ActionLogEntity::find_by_id(id).one(&self.db).await?;
        Ok(result.map(ActionLog::from))
    }
}

/// Apply filter conditions to a query
fn apply_action_log_filter(
    mut query: sea_orm::Select<ActionLogEntity>,
    filter: &ActionLogFilter,
) -> sea_orm::Select<ActionLogEntity> {
    if let Some(actor_id) = filter.actor_id {
        query = query.filter(ActionLogColumn::ActorId.eq(actor_id));
    }

    if let Some(resource_id) = filter.resource_id {
        query = query.filter(ActionLogColumn::ResourceId.eq(resource_id));
    }

    if let Some(resource_type) = filter.resource_type {
        query = query.filter(ActionLogColumn::ResourceType.eq(resource_type));
    }

    if let Some(actions) = &filter.actions
        && !actions.is_empty()
    {
        let action_strs: Vec<&str> = actions.iter().map(|a| a.as_str()).collect();
        query = query.filter(ActionLogColumn::Action.is_in(action_strs));
    }

    query
}
