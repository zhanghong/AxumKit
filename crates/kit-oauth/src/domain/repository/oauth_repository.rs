use kit_entity::common::OAuthProvider;
use kit_entity::user_oauth_connections::Model as OAuthConnectionModel;
use kit_entity::users::Model as UserModel;
use kit_errors::errors::Errors;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait OAuthRepository<C>: Send + Sync where C: ConnectionTrait {
    async fn create_oauth_connection(&self, conn: &C, user_id: &Uuid, provider: OAuthProvider, provider_user_id: &str) -> Result<(), Errors>;
    async fn find_user_by_oauth(&self, conn: &C, provider: OAuthProvider, provider_user_id: &str) -> Result<Option<UserModel>, Errors>;
    async fn find_oauth_connection(&self, conn: &C, user_id: Uuid, provider: OAuthProvider) -> Result<Option<OAuthConnectionModel>, Errors>;
    async fn list_oauth_connections_by_user_id(&self, conn: &C, user_id: Uuid) -> Result<Vec<OAuthConnectionModel>, Errors>;
    async fn count_oauth_connections(&self, conn: &C, user_id: Uuid) -> Result<u64, Errors>;
    async fn delete_oauth_connection(&self, conn: &C, user_id: Uuid, provider: OAuthProvider) -> Result<(), Errors>;
    async fn create_oauth_user(&self, conn: &C, email: &str, display_name: &str, handle: &str, profile_image: Option<String>) -> Result<UserModel, Errors>;
}
