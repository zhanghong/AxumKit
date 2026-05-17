use crate::domain::model::oauth_provider::OAuthProviderType;
use kit_entity::common::OAuthProvider;

#[derive(Debug, Clone)]
pub struct OAuthDomainService;

impl OAuthDomainService {
    pub fn new() -> Self { Self }
    pub fn to_entity_provider(&self, provider: OAuthProviderType) -> OAuthProvider {
        match provider { OAuthProviderType::Google => OAuthProvider::Google, OAuthProviderType::Github => OAuthProvider::Github }
    }
    pub fn to_domain_provider(&self, provider: OAuthProvider) -> OAuthProviderType {
        match provider { OAuthProvider::Google => OAuthProviderType::Google, OAuthProvider::Github => OAuthProviderType::Github }
    }
}

impl Default for OAuthDomainService {
    fn default() -> Self { Self::new() }
}
