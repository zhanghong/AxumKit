use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::model::user_ban::{
    ActiveModel as UserBanActiveModel, Column as UserBanColumn, Entity as UserBanEntity,
    Model as UserBanModel,
};
use crate::domain::repository::user_ban_repository::UserBanRepository;
use kit_errors::errors::Errors;

/// User ban repository implementation backed by a DatabaseConnection
pub struct UserBanRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl UserBanRepositoryImpl {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserBanRepository for UserBanRepositoryImpl {
    async fn create_user_ban(
        &self,
        user_id: Uuid,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserBanModel, Errors> {
        let new_ban = UserBanActiveModel {
            id: Default::default(),
            user_id: Set(user_id),
            expires_at: Set(expires_at),
            created_at: Set(Utc::now()),
        };

        let result = new_ban.insert(self.conn.as_ref()).await?;
        Ok(result)
    }

    async fn find_user_ban(&self, user_id: Uuid) -> Result<Option<UserBanModel>, Errors> {
        let now = Utc::now();

        let ban = UserBanEntity::find()
            .filter(UserBanColumn::UserId.eq(user_id))
            .filter(
                UserBanColumn::ExpiresAt
                    .is_null()
                    .or(UserBanColumn::ExpiresAt.gt(now)),
            )
            .one(self.conn.as_ref())
            .await?;

        Ok(ban)
    }

    async fn is_user_banned(&self, user_id: Uuid) -> Result<bool, Errors> {
        let ban = self.find_user_ban(user_id).await?;
        Ok(ban.is_some())
    }

    async fn delete_user_ban(&self, user_id: Uuid) -> Result<u64, Errors> {
        let result = UserBanEntity::delete_many()
            .filter(UserBanColumn::UserId.eq(user_id))
            .exec(self.conn.as_ref())
            .await?;

        Ok(result.rows_affected)
    }

    async fn delete_expired_user_ban(&self, user_id: Uuid) -> Result<u64, Errors> {
        let now = Utc::now();

        let result = UserBanEntity::delete_many()
            .filter(UserBanColumn::UserId.eq(user_id))
            .filter(UserBanColumn::ExpiresAt.is_not_null())
            .filter(UserBanColumn::ExpiresAt.lte(now))
            .exec(self.conn.as_ref())
            .await?;

        Ok(result.rows_affected)
    }
}
