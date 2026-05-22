use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::model::role::Role;
use crate::domain::model::user_role::{
    ActiveModel as UserRoleActiveModel, Column as UserRoleColumn, Entity as UserRoleEntity,
    Model as UserRoleModel,
};
use crate::domain::repository::user_role_repository::UserRoleRepository;
use kit_errors::errors::Errors;

/// User role repository implementation backed by a DatabaseConnection
pub struct UserRoleRepositoryImpl {
    conn: Arc<DatabaseConnection>,
}

impl UserRoleRepositoryImpl {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRoleRepository for UserRoleRepositoryImpl {
    async fn create_user_role(
        &self,
        user_id: Uuid,
        role: Role,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<UserRoleModel, Errors> {
        let new_role = UserRoleActiveModel {
            id: Default::default(),
            user_id: Set(user_id),
            role: Set(role),
            granted_at: Set(Utc::now()),
            expires_at: Set(expires_at),
        };

        let result = new_role.insert(self.conn.as_ref()).await?;
        Ok(result)
    }

    async fn find_user_roles(&self, user_id: Uuid) -> Result<Vec<Role>, Errors> {
        let now = Utc::now();

        let mut roles = UserRoleEntity::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(
                UserRoleColumn::ExpiresAt
                    .is_null()
                    .or(UserRoleColumn::ExpiresAt.gt(now)),
            )
            .all(self.conn.as_ref())
            .await?
            .into_iter()
            .map(|entry| entry.role)
            .collect::<Vec<_>>();

        roles.sort_by_key(|role| std::cmp::Reverse(role.display_priority()));

        Ok(roles)
    }

    async fn delete_user_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors> {
        let result = UserRoleEntity::delete_many()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::Role.eq(role))
            .exec(self.conn.as_ref())
            .await?;

        Ok(result.rows_affected)
    }

    async fn delete_expired_user_role(&self, user_id: Uuid, role: Role) -> Result<u64, Errors> {
        let now = Utc::now();

        let result = UserRoleEntity::delete_many()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::Role.eq(role))
            .filter(UserRoleColumn::ExpiresAt.is_not_null())
            .filter(UserRoleColumn::ExpiresAt.lte(now))
            .exec(self.conn.as_ref())
            .await?;

        Ok(result.rows_affected)
    }
}
