use crate::application::types::{PendingSignupData, SignInResult};
use crate::application::oauth_service::OAuthApplicationService;
use crate::infrastructure::adapter::{exchange_code, fetch_github_user_emails, fetch_github_user_info, GithubProvider};
use crate::infrastructure::repository::OAuthRepositoryImpl;
use crate::api::dto::request::OAuthAuthorizeFlow;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use reqwest::Client as HttpClient;
use sea_orm::ConnectionTrait;
use tracing::info;

#[derive(Debug, Clone)]
pub struct GithubSignInInput { pub code: String, pub state: String, pub anonymous_user_id: String, pub user_agent: Option<String>, pub ip_address: Option<String> }

#[derive(Debug, Clone)]
pub struct GithubLinkInput { pub user_id: uuid::Uuid, pub code: String, pub state: String, pub anonymous_user_id: String }

#[derive(Debug, Clone)]
pub struct GithubOAuthService { repository: OAuthRepositoryImpl, app_service: OAuthApplicationService }

impl GithubOAuthService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new(), app_service: OAuthApplicationService::new() } }
    pub async fn sign_in<C>(&self, conn: &C, redis_conn: &ConnectionManager, http_client: &HttpClient, input: GithubSignInInput) -> ServiceResult<SignInResult> where C: ConnectionTrait {
        let GithubSignInInput { code, state, anonymous_user_id, user_agent: _, ip_address: _ } = input;
        let state_data = self.app_service.get_and_validate_state(redis_conn, &state, OAuthProvider::Github, OAuthAuthorizeFlow::Login, &anonymous_user_id).await?;
        let access_token = exchange_code::<GithubProvider>(http_client, &code, &state_data.pkce_verifier).await?;
        let user_info = fetch_github_user_info(http_client, &access_token).await?;
        let email = if let Some(email) = user_info.email { email } else {
            let emails = fetch_github_user_emails(http_client, &access_token).await?;
            emails.into_iter().find(|e| e.primary && e.verified).map(|e| e.email).ok_or(Errors::OauthUserInfoParseFailed("No verified primary email found".to_string()))?
        };
        if let Some(existing_user) = self.repository.find_user_by_oauth(conn, OAuthProvider::Github, &user_info.id.to_string()).await? {
            info!(user_id = %existing_user.id, "GitHub OAuth sign-in successful");
            return Ok(SignInResult::Success(existing_user.id.to_string()));
        }
        let pending_data = PendingSignupData { provider: OAuthProvider::Github, provider_user_id: user_info.id.to_string(), anonymous_user_id: anonymous_user_id.to_string(), email: email.clone(), profile_image: Some(user_info.avatar_url) };
        let pending_token = self.app_service.store_pending_signup(redis_conn, &pending_data).await?;
        Ok(SignInResult::PendingSignup { pending_token, email })
    }
    pub async fn link_oauth(&self, conn: &sea_orm::DatabaseConnection, redis_conn: &ConnectionManager, http_client: &HttpClient, input: GithubLinkInput) -> ServiceResult<()> {
        let GithubLinkInput { user_id, code, state, anonymous_user_id } = input;
        let state_data = self.app_service.get_and_validate_state(redis_conn, &state, OAuthProvider::Github, OAuthAuthorizeFlow::Link, &anonymous_user_id).await?;
        let access_token = exchange_code::<GithubProvider>(http_client, &code, &state_data.pkce_verifier).await?;
        let user_info = fetch_github_user_info(http_client, &access_token).await?;
        let txn = conn.begin().await?;
        if self.repository.find_user_by_oauth(&txn, OAuthProvider::Github, &user_info.id.to_string()).await?.is_some() { return Err(Errors::OauthAccountAlreadyLinked); }
        if self.repository.find_oauth_connection(&txn, user_id, OAuthProvider::Github).await?.is_some() { return Err(Errors::OauthAccountAlreadyLinked); }
        self.repository.create_oauth_connection(&txn, &user_id, OAuthProvider::Github, &user_info.id.to_string()).await?;
        txn.commit().await?;
        info!(user_id = %user_id, "GitHub OAuth linked successfully");
        Ok(())
    }
}
impl Default for GithubOAuthService { fn default() -> Self { Self::new() } }
