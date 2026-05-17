use crate::domain::repository::OAuthRepository;
use chrono::Utc;
use kit_entity::common::OAuthProvider;
use kit_entity::user_oauth_connections::{ActiveModel as OAuthConnectionActiveModel, Column as OAuthConnectionsColumn, Entity as OAuthConnectionsEntity, Model as OAuthConnectionModel};
use kit_entity::users::{ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel, Relation as UserRelation};
use kit_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, Order, QueryFilter, QueryOrder, RelationTrait, Set};
use sea_orm::PaginatorTrait;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OAuthRepositoryImpl;

impl OAuthRepositoryImpl {
    pub fn new() -> Self { Self }
}

impl Default for OAuthRepositoryImpl {
    fn default() -> Self { Self::new() }
}

#[async_trait::async_trait]
impl<C> OAuthRepository<C> for OAuthRepositoryImpl where C: ConnectionTrait + Send + Sync {
    async fn create_oauth_connection(&self, conn: &C, user_id: &Uuid, provider: OAuthProvider, provider_user_id: &str) -> Result<(), Errors> {
        let oauth_connection = OAuthConnectionActiveModel {
            id: Default::default(), user_id: Set(*user_id), provider: Set(provider),
            provider_user_id: Set(provider_user_id.to_string()), created_at: Default::default(),
        };
        oauth_connection.insert(conn).await.map_err(|e| { error!("Failed to create OAuth connection: {:?}", e); Errors::DatabaseError(e.to_string()) })?;
        Ok(())
    }
    async fn find_user_by_oauth(&self, conn: &C, provider: OAuthProvider, provider_user_id: &str) -> Result<Option<UserModel>, Errors> {
        let user = UserEntity::find().join(JoinType::InnerJoin, UserRelation::OAuthConnections.def())
            .filter(OAuthConnectionsColumn::Provider.eq(provider)).filter(OAuthConnectionsColumn::ProviderUserId.eq(provider_user_id))
            .one(conn).await?;
        Ok(user)
    }
    async fn find_oauth_connection(&self, conn: &C, user_id: Uuid, provider: OAuthProvider) -> Result<Option<OAuthConnectionModel>, Errors> {
        let connection = OAuthConnectionsEntity::find().filter(OAuthConnectionsColumn::UserId.eq(user_id)).filter(OAuthConnectionsColumn::Provider.eq(provider)).one(conn).await?;
        Ok(connection)
    }
    async fn list_oauth_connections_by_user_id(&self, conn: &C, user_id: Uuid) -> Result<Vec<OAuthConnectionModel>, Errors> {
        let connections = OAuthConnectionsEntity::find().filter(OAuthConnectionsColumn::UserId.eq(user_id)).order_by(OAuthConnectionsColumn::Id, Order::Asc).all(conn).await?;
        Ok(connections)
    }
    async fn count_oauth_connections(&self, conn: &C, user_id: Uuid) -> Result<u64, Errors> {
        let count = OAuthConnectionsEntity::find().filter(OAuthConnectionsColumn::UserId.eq(user_id)).count(conn).await?;
        Ok(count)
    }
    async fn delete_oauth_connection(&self, conn: &C, user_id: Uuid, provider: OAuthProvider) -> Result<(), Errors> {
        let result = OAuthConnectionsEntity::delete_many().filter(OAuthConnectionsColumn::UserId.eq(user_id)).filter(OAuthConnectionsColumn::Provider.eq(provider)).exec(conn).await?;
        if result.rows_affected == 0 { return Err(Errors::OauthConnectionNotFound); }
        Ok(())
    }
    async fn create_oauth_user(&self, conn: &C, email: &str, display_name: &str, handle: &str, profile_image: Option<String>) -> Result<UserModel, Errors> {
        let new_user = UserActiveModel {
            id: Default::default(), display_name: Set(display_name.to_string()), handle: Set(handle.to_string()),
            bio: Set(None), email: Set(email.to_string()), password: Set(None), verified_at: Set(Some(Utc::now())),
            profile_image: Set(profile_image), banner_image: Set(None), totp_secret: Set(None),
            totp_enabled_at: Set(None), totp_backup_codes: Set(None), created_at: Default::default(),
        };
        let user = new_user.insert(conn).await?;
        Ok(user)
    }
}
