use crate::application::types::{PendingSignupData, SignInResult};
use crate::application::oauth_service::OAuthApplicationService;
use crate::infrastructure::adapter::{exchange_code, fetch_google_user_info, GoogleProvider};
use crate::infrastructure::repository::OAuthRepositoryImpl;
use crate::api::dto::request::OAuthAuthorizeFlow;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use reqwest::Client as HttpClient;
use sea_orm::ConnectionTrait;
use tracing::info;

#[derive(Debug, Clone)]
pub struct GoogleSignInInput { pub code: String, pub state: String, pub anonymous_user_id: String, pub user_agent: Option<String>, pub ip_address: Option<String> }

#[derive(Debug, Clone)]
pub struct GoogleLinkInput { pub user_id: uuid::Uuid, pub code: String, pub state: String, pub anonymous_user_id: String }

#[derive(Debug, Clone)]
pub struct GoogleOAuthService { repository: OAuthRepositoryImpl, app_service: OAuthApplicationService }

impl GoogleOAuthService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new(), app_service: OAuthApplicationService::new() } }
    pub async fn sign_in<C>(&self, conn: &C, redis_conn: &ConnectionManager, http_client: &HttpClient, input: GoogleSignInInput) -> ServiceResult<SignInResult> where C: ConnectionTrait {
        let GoogleSignInInput { code, state, anonymous_user_id, user_agent: _, ip_address: _ } = input;
        let state_data = self.app_service.get_and_validate_state(redis_conn, &state, OAuthProvider::Google, OAuthAuthorizeFlow::Login, &anonymous_user_id).await?;
        let access_token = exchange_code::<GoogleProvider>(http_client, &code, &state_data.pkce_verifier).await?;
        let user_info = fetch_google_user_info(http_client, &access_token).await?;
        if !user_info.verified_email { return Err(Errors::OauthEmailNotVerified); }
        if let Some(existing_user) = self.repository.find_user_by_oauth(conn, OAuthProvider::Google, &user_info.id).await? {
            info!(user_id = %existing_user.id, "Google OAuth sign-in successful");
            return Ok(SignInResult::Success(existing_user.id.to_string()));
        }
        let pending_data = PendingSignupData { provider: OAuthProvider::Google, provider_user_id: user_info.id, anonymous_user_id: anonymous_user_id.to_string(), email: user_info.email.clone(), profile_image: Some(user_info.picture) };
        let pending_token = self.app_service.store_pending_signup(redis_conn, &pending_data).await?;
        Ok(SignInResult::PendingSignup { pending_token, email: user_info.email })
    }
    pub async fn link_oauth(&self, conn: &sea_orm::DatabaseConnection, redis_conn: &ConnectionManager, http_client: &HttpClient, input: GoogleLinkInput) -> ServiceResult<()> {
        let GoogleLinkInput { user_id, code, state, anonymous_user_id } = input;
        let state_data = self.app_service.get_and_validate_state(redis_conn, &state, OAuthProvider::Google, OAuthAuthorizeFlow::Link, &anonymous_user_id).await?;
        let access_token = exchange_code::<GoogleProvider>(http_client, &code, &state_data.pkce_verifier).await?;
        let user_info = fetch_google_user_info(http_client, &access_token).await?;
        if !user_info.verified_email { return Err(Errors::OauthEmailNotVerified); }
        let txn = conn.begin().await?;
        if self.repository.find_user_by_oauth(&txn, OAuthProvider::Google, &user_info.id).await?.is_some() { return Err(Errors::OauthAccountAlreadyLinked); }
        if self.repository.find_oauth_connection(&txn, user_id, OAuthProvider::Google).await?.is_some() { return Err(Errors::OauthAccountAlreadyLinked); }
        self.repository.create_oauth_connection(&txn, &user_id, OAuthProvider::Google, &user_info.id).await?;
        txn.commit().await?;
        info!(user_id = %user_id, "Google OAuth linked successfully");
        Ok(())
    }
}
impl Default for GoogleOAuthService { fn default() -> Self { Self::new() } }
