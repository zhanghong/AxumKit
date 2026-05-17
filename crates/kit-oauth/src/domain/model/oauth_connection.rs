use crate::domain::model::oauth_provider::OAuthProviderType;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OAuthConnectionId(pub Uuid);

impl OAuthConnectionId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OAuthConnectionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct OAuthConnection {
    pub id: OAuthConnectionId,
    pub user_id: Uuid,
    pub provider: OAuthProviderType,
    pub provider_user_id: String,
    pub created_at: DateTime<Utc>,
}

impl OAuthConnection {
    pub fn new(user_id: Uuid, provider: OAuthProviderType, provider_user_id: String) -> Self {
        Self { id: OAuthConnectionId::new(), user_id, provider, provider_user_id, created_at: Utc::now() }
    }
}
