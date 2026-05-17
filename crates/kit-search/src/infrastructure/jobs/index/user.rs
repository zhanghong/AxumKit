use crate::domain::service::search_service::build_user_search_json;
use kit_entity::users;
use meilisearch_sdk::client::Client;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexUserJob {
    pub user_id: Uuid,
    pub action: UserIndexAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIndexAction {
    Index,
    Delete,
}

pub const USERS_INDEX: &str = "users";

/// MeiliSearch index settings for users
pub fn user_index_settings() -> meilisearch_sdk::settings::Settings {
    meilisearch_sdk::settings::Settings::new()
        .with_searchable_attributes(["handle", "display_name", "bio"])
        .with_displayed_attributes(["id", "handle", "display_name", "bio", "profile_image"])
        .with_ranking_rules(["words", "typo", "proximity", "attribute", "exactness"])
}

/// Ensure index exists with proper settings
pub async fn ensure_index_settings(
    client: &Client,
) -> Result<(), anyhow::Error> {
    let index = client.index(USERS_INDEX);

    // Check if index exists by trying to get stats
    match index.get_stats().await {
        Ok(_) => {
            // Index exists, settings should already be applied
            Ok(())
        }
        Err(meilisearch_sdk::errors::Error::Meilisearch(ref e))
            if e.error_code == meilisearch_sdk::errors::ErrorCode::IndexNotFound =>
        {
            // Index doesn't exist, create it
            tracing::info!("Creating users index...");
            let task = client.create_index(USERS_INDEX, Some("id")).await?;
            task.wait_for_completion(client, None, None).await?;

            // Apply settings
            tracing::info!("Applying users index settings...");
            let index = client.index(USERS_INDEX);
            let task = index.set_settings(&user_index_settings()).await?;
            task.wait_for_completion(client, None, None).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

/// Handle a single user index job
pub async fn handle_index_user(
    job: IndexUserJob,
    client: &Client,
    db: &DatabaseConnection,
) -> Result<(), anyhow::Error> {
    tracing::info!(
        "Processing user index job: user_id={}, action={:?}",
        job.user_id,
        job.action
    );

    let index = client.index(USERS_INDEX);

    // Ensure index exists and settings are applied
    ensure_index_settings(client).await?;

    match job.action {
        UserIndexAction::Index => {
            // Fetch user from DB
            let user = users::Entity::find_by_id(job.user_id)
                .one(db)
                .await?
                .ok_or_else(|| anyhow::anyhow!("User not found: {}", job.user_id))?;

            // Build search document
            let search_user = build_user_search_json(&user);

            // Add to index (upsert)
            index.add_documents(&[search_user], Some("id")).await?;
            tracing::info!("User {} indexed successfully", job.user_id);
        }
        UserIndexAction::Delete => {
            index.delete_document(&job.user_id.to_string()).await?;
            tracing::info!("User {} deleted from index", job.user_id);
        }
    }

    Ok(())
}
