use crate::application::types::PendingSignupData;
use crate::application::oauth_service::OAuthApplicationService;
use crate::domain::repository::OAuthRepository;
use crate::infrastructure::repository::OAuthRepositoryImpl;
use kit_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CompleteSignupService { repository: OAuthRepositoryImpl, app_service: OAuthApplicationService }
impl CompleteSignupService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new(), app_service: OAuthApplicationService::new() } }
    pub async fn complete_signup<C>(&self, conn: &C, redis_conn: &ConnectionManager, pending_token: &str, handle: &str, display_name: &str, anonymous_user_id: &str) -> ServiceResult<Uuid> where C: ConnectionTrait + TransactionTrait {
        let pending_data = self.app_service.get_pending_signup(redis_conn, pending_token).await?;
        if pending_data.anonymous_user_id != anonymous_user_id { return Err(Errors::UserInvalidToken); }
        let provider = pending_data.provider.clone();
        let provider_user_id = pending_data.provider_user_id.clone();
        let email = pending_data.email.clone();
        let txn = conn.begin().await?;
        if self.repository.find_user_by_oauth(&txn, provider.clone(), &provider_user_id).await?.is_some() {
            return Err(Errors::OauthAccountAlreadyLinked);
        }
        let new_user = self.repository.create_oauth_user(&txn, &email, display_name, handle, pending_data.profile_image).await?;
        self.repository.create_oauth_connection(&txn, &new_user.id, provider.clone(), &provider_user_id).await?;
        txn.commit().await?;
        if let Err(err) = self.app_service.delete_pending_signup(redis_conn, pending_token).await {
            warn!(pending_token = %pending_token, error = ?err, "Failed to delete OAuth pending signup token");
        }
        info!(user_id = %new_user.id, provider = ?provider, "OAuth signup completed");
        Ok(new_user.id)
    }
}
impl Default for CompleteSignupService { fn default() -> Self { Self::new() } }
