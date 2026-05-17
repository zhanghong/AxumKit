use crate::domain::model::{ModerationAction, ModerationResourceType};
use crate::domain::model::{Column, Entity};
use sea_orm::{ColumnTrait, QueryFilter, Select};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct ModerationLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<ModerationResourceType>,
    pub resource_id: Option<Uuid>,
    pub actions: Option<Vec<ModerationAction>>,
}

pub(crate) fn apply_moderation_log_filter(
    mut query: Select<Entity>,
    filter: &ModerationLogFilter,
) -> Select<Entity> {
    if let Some(actor_id) = filter.actor_id {
        query = query.filter(Column::ActorId.eq(actor_id));
    }

    if let Some(resource_type) = filter.resource_type.clone() {
        query = query.filter(Column::ResourceType.eq(resource_type));
    }

    if let Some(resource_id) = filter.resource_id {
        query = query.filter(Column::ResourceId.eq(resource_id));
    }

    if let Some(actions) = &filter.actions
        && !actions.is_empty()
    {
        let action_strs: Vec<&str> = actions.iter().map(|action| action.as_str()).collect();
        query = query.filter(Column::Action.is_in(action_strs));
    }

    query
}
