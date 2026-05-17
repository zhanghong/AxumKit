use super::filter::{ModerationLogFilter, apply_moderation_log_filter};
use crate::domain::model::{
    ActiveModel, Column, Entity, Model, ModerationAction, ModerationResourceType,
};
use kit_dto::pagination::CursorDirection;
use kit_errors::errors::Errors;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub async fn create_moderation_log<C>(
    conn: &C,
    action: ModerationAction,
    actor_id: Option<Uuid>,
    resource_type: ModerationResourceType,
    resource_id: Option<Uuid>,
    reason: String,
    metadata: Option<JsonValue>,
) -> Result<Model, Errors>
where
    C: ConnectionTrait,
{
    let log = ActiveModel {
        id: Default::default(),
        action: Set(action.as_str().to_string()),
        actor_id: Set(actor_id),
        resource_type: Set(resource_type),
        resource_id: Set(resource_id),
        reason: Set(reason),
        metadata: Set(metadata),
        created_at: Default::default(),
    };

    let log = log.insert(conn).await?;

    Ok(log)
}

pub async fn find_moderation_logs<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<Model>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = apply_moderation_log_filter(Entity::find(), filter);

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(Column::Id.lt(id))
                .order_by_desc(Column::Id),
            CursorDirection::Newer => query
                .filter(Column::Id.gt(id))
                .order_by_asc(Column::Id),
        };
    } else {
        query = query.order_by_desc(Column::Id);
    }

    let results = query.limit(limit).all(conn).await?;
    Ok(results)
}

pub async fn exists_newer_moderation_log<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_moderation_log_filter(
        Entity::find().filter(Column::Id.gt(cursor_id)),
        filter,
    );

    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}

pub async fn exists_older_moderation_log<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let query = apply_moderation_log_filter(
        Entity::find().filter(Column::Id.lt(cursor_id)),
        filter,
    );

    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}
