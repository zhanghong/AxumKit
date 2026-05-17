use crate::application::types::PendingSignupData;
use crate::infrastructure::repository::OAuthRepositoryImpl;
use kit_config::ServerConfig;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OAuthApplicationService { repository: OAuthRepositoryImpl }
impl OAuthApplicationService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new() } }
    pub async fn get_and_validate_state(&self, redis_conn: &ConnectionManager, state: &str, expected_provider: OAuthProvider, expected_flow: crate::api::dto::request::OAuthAuthorizeFlow, anonymous_user_id: &str) -> ServiceResult<OAuthStateData> {
        let mut conn = redis_conn.clone();
        let state_key = format!("oauth:state:{}", state);
        let json: Option<String> = redis::cmd("GETDEL").arg(&state_key).query_async(&mut conn).await.map_err(|e| Errors::SysInternalError(e.to_string()))?;
        let json = json.ok_or(Errors::OauthInvalidState)?;
        let state_data: OAuthStateData = serde_json::from_str(&json).map_err(|_| Errors::OauthInvalidState)?;
        if state_data.provider != expected_provider || state_data.flow != expected_flow || state_data.anonymous_user_id != anonymous_user_id {
            return Err(Errors::OauthInvalidState);
        }
        Ok(state_data)
    }
    pub async fn store_pending_signup(&self, redis_conn: &ConnectionManager, pending_data: &PendingSignupData) -> ServiceResult<String> {
        let config = ServerConfig::get();
        let pending_token = Uuid::new_v4().to_string();
        let pending_key = format!("oauth:pending:{}", pending_token);
        let ttl_seconds = (config.oauth_pending_signup_ttl_minutes * 60) as u64;
        let json = serde_json::to_string(pending_data).map_err(|e| Errors::SysInternalError(e.to_string()))?;
        let mut conn = redis_conn.clone();
        redis::cmd("SETEX").arg(&pending_key).arg(ttl_seconds).arg(json).query_async::<_, ()>(&mut conn).await.map_err(|e| Errors::SysInternalError(e.to_string()))?;
        Ok(pending_token)
    }
    pub async fn get_pending_signup(&self, redis_conn: &ConnectionManager, pending_token: &str) -> ServiceResult<PendingSignupData> {
        let mut conn = redis_conn.clone();
        let pending_key = format!("oauth:pending:{}", pending_token);
        let json: Option<String> = redis::cmd("GET").arg(&pending_key).query_async(&mut conn).await.map_err(|e| Errors::SysInternalError(e.to_string()))?;
        let json = json.ok_or(Errors::UserTokenExpired)?;
        let pending_data: PendingSignupData = serde_json::from_str(&json).map_err(|_| Errors::UserInvalidToken)?;
        Ok(pending_data)
    }
    pub async fn delete_pending_signup(&self, redis_conn: &ConnectionManager, pending_token: &str) -> ServiceResult<()> {
        let mut conn = redis_conn.clone();
        let pending_key = format!("oauth:pending:{}", pending_token);
        redis::cmd("DEL").arg(&pending_key).query_async::<_, ()>(&mut conn).await.map_err(|e| Errors::SysInternalError(e.to_string()))?;
        Ok(())
    }
}
impl Default for OAuthApplicationService { fn default() -> Self { Self::new() } }
