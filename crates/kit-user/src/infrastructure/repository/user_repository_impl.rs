use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, Set,
};
use uuid::Uuid;

use crate::domain::model::user::{
    ActiveModel as UserActiveModel, Column as UsersColumn, Entity as UserEntity,
    Model as UserModel,
};
use crate::domain::repository::user_repository::{
    UserRepository, UserUpdateParams,
};
use kit_errors::errors::Errors;

/// User repository implementation with connection
pub struct UserRepositoryImplWithConn<'a, C: ConnectionTrait> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> UserRepositoryImplWithConn<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a, C: ConnectionTrait + Send + Sync> UserRepository for UserRepositoryImplWithConn<'a, C> {
    async fn create_user(
        &self,
        _email: String,
        _handle: String,
        _display_name: String,
        _password: String,
    ) -> Result<UserModel, Errors> {
        unimplemented!("Use create_user_with_password_hash")
    }

    async fn create_user_with_password_hash(
        &self,
        email: String,
        handle: String,
        display_name: String,
        password_hash: String,
    ) -> Result<UserModel, Errors> {
        let new_user = UserActiveModel {
            id: Default::default(),
            display_name: Set(display_name),
            handle: Set(handle),
            bio: Set(None),
            email: Set(email),
            password: Set(Some(password_hash)),
            verified_at: Set(None),
            profile_image: Set(None),
            banner_image: Set(None),
            totp_secret: Set(None),
            totp_enabled_at: Set(None),
            totp_backup_codes: Set(None),
            created_at: Default::default(),
        };

        let user = new_user.insert(self.conn).await?;
        Ok(user)
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, Errors> {
        let user = UserEntity::find_by_id(id).one(self.conn).await?;
        Ok(user)
    }

    async fn find_user_by_email(&self, email: String) -> Result<Option<UserModel>, Errors> {
        let user = UserEntity::find()
            .filter(UsersColumn::Email.eq(email))
            .one(self.conn)
            .await?;
        Ok(user)
    }

    async fn find_user_by_handle(&self, handle: String) -> Result<Option<UserModel>, Errors> {
        let user = UserEntity::find()
            .filter(UsersColumn::Handle.eq(handle))
            .one(self.conn)
            .await?;
        Ok(user)
    }

    async fn find_users_by_ids(&self, ids: &[Uuid]) -> Result<Vec<UserModel>, Errors> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let users = UserEntity::find()
            .filter(UsersColumn::Id.is_in(ids.to_vec()))
            .all(self.conn)
            .await?;

        Ok(users)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserModel, Errors> {
        let user = UserEntity::find_by_id(id).one(self.conn).await?;
        user.ok_or(Errors::UserNotFound)
    }

    async fn get_user_by_id_for_update(&self, id: Uuid) -> Result<UserModel, Errors> {
        let user = UserEntity::find_by_id(id)
            .lock_exclusive()
            .one(self.conn)
            .await?;
        user.ok_or(Errors::UserNotFound)
    }

    async fn get_user_by_email(&self, email: String) -> Result<UserModel, Errors> {
        let user = UserEntity::find()
            .filter(UsersColumn::Email.eq(email))
            .one(self.conn)
            .await?;
        user.ok_or(Errors::UserNotFound)
    }

    async fn get_user_by_handle(&self, handle: String) -> Result<UserModel, Errors> {
        let user = UserEntity::find()
            .filter(UsersColumn::Handle.eq(handle))
            .one(self.conn)
            .await?;
        user.ok_or(Errors::UserNotFound)
    }

    async fn update_user(&self, user_id: Uuid, params: UserUpdateParams) -> Result<UserModel, Errors> {
        let user = UserEntity::find_by_id(user_id)
            .one(self.conn)
            .await?
            .ok_or(Errors::UserNotFound)?;

        let mut user_active: UserActiveModel = user.into_active_model();

        if let Some(email) = params.email {
            user_active.email = Set(email);
        }
        if let Some(display_name) = params.display_name {
            user_active.display_name = Set(display_name);
        }
        if let Some(bio) = params.bio {
            user_active.bio = Set(bio);
        }
        if let Some(password) = params.password {
            user_active.password = Set(password);
        }
        if let Some(verified_at) = params.verified_at {
            user_active.verified_at = Set(verified_at);
        }
        if let Some(profile_image) = params.profile_image {
            user_active.profile_image = Set(profile_image);
        }
        if let Some(banner_image) = params.banner_image {
            user_active.banner_image = Set(banner_image);
        }
        if let Some(totp_secret) = params.totp_secret {
            user_active.totp_secret = Set(totp_secret);
        }
        if let Some(totp_enabled_at) = params.totp_enabled_at {
            user_active.totp_enabled_at = Set(totp_enabled_at);
        }
        if let Some(totp_backup_codes) = params.totp_backup_codes {
            user_active.totp_backup_codes = Set(totp_backup_codes);
        }

        let updated_user = user_active.update(self.conn).await?;
        Ok(updated_user)
    }
}
