use crate::domain::service::search_service::build_user_search_json;
use crate::infrastructure::jobs::index::user::{USERS_INDEX, ensure_index_settings};
use kit_entity::users;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_BATCH_SIZE: u32 = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReindexJobBase {
    pub after_id: Option<Uuid>,
    pub batch_size: u32,
    pub reindex_id: Uuid,
    pub batch_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReindexUsersJob {
    pub base: ReindexJobBase,
}

/// Handle a batch reindex job for users
pub async fn handle_reindex_users(
    job: ReindexUsersJob,
    client: &meilisearch_sdk::client::Client,
    db: &sea_orm::DatabaseConnection,
) -> Result<Option<ReindexUsersJob>, anyhow::Error> {
    tracing::info!(
        reindex_id = %job.base.reindex_id,
        batch_number = job.base.batch_number,
        after_id = ?job.base.after_id,
        batch_size = job.base.batch_size,
        "Processing user reindex batch"
    );

    // First batch: ensure index settings and clear existing data
    if job.base.after_id.is_none() {
        ensure_index_settings(client).await?;

        // Clear all existing users before reindexing
        let index = client.index(USERS_INDEX);
        index.delete_all_documents().await?;

        let total = users::Entity::find().count(db).await?;
        tracing::info!(
            reindex_id = %job.base.reindex_id,
            total_users = total,
            "Starting user reindex"
        );
    }

    // Fetch batch of users
    let users_batch =
        fetch_users_batch(db, job.base.after_id, job.base.batch_size).await?;

    if users_batch.is_empty() {
        tracing::info!(
            reindex_id = %job.base.reindex_id,
            total_batches = job.base.batch_number,
            "User reindex completed"
        );
        return Ok(None);
    }

    // Build search documents
    let search_docs: Vec<_> = users_batch.iter().map(build_user_search_json).collect();

    // Index batch to MeiliSearch
    let index = client.index(USERS_INDEX);
    index.add_documents(&search_docs, Some("id")).await?;

    let processed_count = users_batch.len();
    let last_id = users_batch
        .last()
        .map(|u| u.id)
        .ok_or_else(|| anyhow::anyhow!("users_batch unexpectedly empty"))?;

    tracing::info!(
        reindex_id = %job.base.reindex_id,
        batch_number = job.base.batch_number,
        processed = processed_count,
        last_id = %last_id,
        "Batch processed"
    );

    // Build next batch job
    let next_job = ReindexUsersJob {
        base: ReindexJobBase {
            after_id: Some(last_id),
            batch_size: job.base.batch_size,
            reindex_id: job.base.reindex_id,
            batch_number: job.base.batch_number + 1,
        },
    };

    Ok(Some(next_job))
}

/// Fetch a batch of users using UUID v7 cursor pagination
async fn fetch_users_batch(
    db: &sea_orm::DatabaseConnection,
    after_id: Option<Uuid>,
    batch_size: u32,
) -> Result<Vec<users::Model>, anyhow::Error> {
    let mut query = users::Entity::find().order_by_asc(users::Column::Id);

    if let Some(cursor) = after_id {
        query = query.filter(users::Column::Id.gt(cursor));
    }

    let users = query.limit(batch_size as u64).all(db).await?;

    Ok(users)
}

/// Create a new ReindexUsersJob to start reindexing from the beginning
pub fn create_reindex_users_job(reindex_id: Uuid, batch_size: Option<u32>) -> ReindexUsersJob {
    ReindexUsersJob {
        base: ReindexJobBase {
            after_id: None,
            batch_size: batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
            reindex_id,
            batch_number: 1,
        },
    }
}
