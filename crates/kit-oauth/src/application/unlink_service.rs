use crate::domain::repository::OAuthRepository;
use crate::infrastructure::repository::OAuthRepositoryImpl;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UnlinkService { repository: OAuthRepositoryImpl }
impl UnlinkService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new() } }
    pub async fn unlink_oauth(&self, conn: &DatabaseConnection, user_id: Uuid, provider: OAuthProvider) -> ServiceResult<()> {
        let txn = conn.begin().await?;
        let connections = self.repository.list_oauth_connections_by_user_id(&txn, user_id).await?;
        let oauth_count = connections.len();
        let has_target_provider = connections.iter().any(|c| c.provider == provider);
        if !has_target_provider { return Err(Errors::OauthConnectionNotFound); }
        if oauth_count <= 1 { return Err(Errors::OauthCannotUnlinkLastConnection); }
        self.repository.delete_oauth_connection(&txn, user_id, provider.clone()).await?;
        txn.commit().await?;
        info!(user_id = %user_id, provider = ?provider, "OAuth connection unlinked");
        Ok(())
    }
}
impl Default for UnlinkService { fn default() -> Self { Self::new() } }
