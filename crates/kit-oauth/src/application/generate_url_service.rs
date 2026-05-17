use crate::application::types::OAuthStateData;
use crate::infrastructure::adapter::oauth_client::generate_auth_url;
use crate::infrastructure::config::oauth_config::OAuthProviderConfig;
use crate::api::dto::request::OAuthAuthorizeFlow;
use crate::api::dto::response::OAuthUrlResponse;
use kit_entity::common::OAuthProvider;
use kit_errors::errors::ServiceResult;
use redis::aio::ConnectionManager;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GenerateUrlService;
impl GenerateUrlService {
    pub fn new() -> Self { Self }
    pub async fn generate_url<P: OAuthProviderConfig>(&self, redis_conn: &ConnectionManager, anonymous_user_id: &str, flow: OAuthAuthorizeFlow, provider: OAuthProvider) -> ServiceResult<OAuthUrlResponse> {
        let state = Uuid::now_v7().to_string();
        let (auth_url, _state, pkce_verifier) = generate_auth_url::<P>(state.clone())?;
        let state_data = OAuthStateData { pkce_verifier, flow, provider, anonymous_user_id: anonymous_user_id.to_string() };
        let mut conn = redis_conn.clone();
        let state_key = format!("oauth:state:{}", state);
        let ttl_seconds = 600;
        let json = serde_json::to_string(&state_data).map_err(|e| kit_errors::errors::Errors::SysInternalError(e.to_string()))?;
        redis::cmd("SETEX").arg(&state_key).arg(ttl_seconds).arg(json).query_async::<_, ()>(&mut conn).await.map_err(|e| kit_errors::errors::Errors::SysInternalError(e.to_string()))?;
        Ok(OAuthUrlResponse { auth_url })
    }
}
impl Default for GenerateUrlService { fn default() -> Self { Self::new() } }
