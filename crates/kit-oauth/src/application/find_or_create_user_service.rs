use crate::application::types::OAuthUserResult;
use crate::domain::repository::OAuthRepository;
use crate::infrastructure::repository::OAuthRepositoryImpl;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use tracing::info;

#[derive(Debug, Clone)]
pub struct FindOrCreateUserService { repository: OAuthRepositoryImpl }
impl FindOrCreateUserService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new() } }
    pub async fn find_or_create_user<C>(&self, conn: &C, provider: OAuthProvider, provider_user_id: &str, email: &str, display_name: &str, handle: Option<&str>, profile_image: Option<String>) -> ServiceResult<OAuthUserResult> where C: ConnectionTrait {
        if let Some(user) = self.repository.find_user_by_oauth(conn, provider.clone(), provider_user_id).await? {
            return Ok(OAuthUserResult { user, is_new_user: false });
        }
        let handle = handle.ok_or(Errors::OauthHandleRequired)?;
        let new_user = self.repository.create_oauth_user(conn, email, display_name, handle, profile_image).await?;
        self.repository.create_oauth_connection(conn, &new_user.id, provider.clone(), provider_user_id).await?;
        info!(user_id = %new_user.id, provider = ?provider, "OAuth user created");
        Ok(OAuthUserResult { user: new_user, is_new_user: true })
    }
}
impl Default for FindOrCreateUserService { fn default() -> Self { Self::new() } }
