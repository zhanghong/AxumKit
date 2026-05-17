use crate::domain::repository::OAuthRepository;
use crate::infrastructure::repository::OAuthRepositoryImpl;
use crate::api::dto::response::{OAuthConnectionListResponse, OAuthConnectionResponse};
use kit_errors::errors::ServiceResult;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ListConnectionsService { repository: OAuthRepositoryImpl }
impl ListConnectionsService {
    pub fn new() -> Self { Self { repository: OAuthRepositoryImpl::new() } }
    pub async fn list_connections<C>(&self, conn: &C, user_id: Uuid) -> ServiceResult<OAuthConnectionListResponse> where C: ConnectionTrait {
        let connections = self.repository.list_oauth_connections_by_user_id(conn, user_id).await?;
        Ok(OAuthConnectionListResponse { connections: connections.into_iter().map(OAuthConnectionResponse::from).collect() })
    }
}
impl Default for ListConnectionsService { fn default() -> Self { Self::new() } }
