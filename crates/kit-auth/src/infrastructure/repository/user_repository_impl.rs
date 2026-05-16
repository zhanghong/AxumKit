use async_trait::async_trait;
use kit_entity::users::{ActiveModel as UserActiveModel, Entity as UserEntity, Model as User};
use kit_errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::domain::repository::{
    NewUser, UserRepository, UserUpdateParams,
};

/// 密码哈希函数
fn hash_password(password: &str) -> Result<String, Errors> {
    use argon2::password_hash::SaltString;
    use argon2::password_hash::rand_core::OsRng;
    use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};

    // OWASP - Password Storage Cheat Sheet
    // Use Argon2id with a minimum configuration of 19 MiB of memory,
    // an iteration count of 2, and 1 degree of parallelism.
    let params = Params::new(
        19 * 1024, // 19 MiB memory (in KB)
        2,         // iterations
        1,         // parallelism
        None,      // output length default (32 bytes)
    )
    .map_err(|e| Errors::HashingError(e.to_string()))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Errors::HashingError(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

/// UserRepository 实现
pub struct UserRepositoryImpl<DB: ConnectionTrait> {
    db: DB,
}

impl<DB: ConnectionTrait> UserRepositoryImpl<DB> {
    /// 创建新的 UserRepositoryImpl 实例
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<DB: ConnectionTrait + Send + Sync> UserRepository for UserRepositoryImpl<DB> {
    /// 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Errors> {
        use kit_entity::users::Column as UsersColumn;

        let user = UserEntity::find()
            .filter(UsersColumn::Email.eq(email))
            .one(&self.db)
            .await?;

        Ok(user)
    }

    /// 根据 handle 查找用户
    async fn find_by_handle(&self, handle: &str) -> Result<Option<User>, Errors> {
        use kit_entity::users::Column as UsersColumn;

        let user = UserEntity::find()
            .filter(UsersColumn::Handle.eq(handle))
            .one(&self.db)
            .await?;

        Ok(user)
    }

    /// 根据 ID 查找用户
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Errors> {
        let user = UserEntity::find_by_id(id).one(&self.db).await?;

        Ok(user)
    }

    /// 根据 ID 获取用户，不存在时返回错误
    async fn get_by_id(&self, id: Uuid) -> Result<User, Errors> {
        let user = UserEntity::find_by_id(id).one(&self.db).await?;

        user.ok_or(Errors::UserNotFound)
    }

    /// 创建新用户
    async fn create(&self, user: &NewUser) -> Result<User, Errors> {
        let hashed_password = hash_password(&user.password)?;

        let new_user = UserActiveModel {
            id: Default::default(),
            display_name: Set(user.display_name.clone()),
            handle: Set(user.handle.clone()),
            bio: Set(None),
            email: Set(user.email.clone()),
            password: Set(Some(hashed_password)),
            verified_at: Set(None),
            profile_image: Set(None),
            banner_image: Set(None),
            totp_secret: Set(None),
            totp_enabled_at: Set(None),
            totp_backup_codes: Set(None),
            created_at: Default::default(),
        };

        let user = new_user.insert(&self.db).await?;

        Ok(user)
    }

    /// 更新用户信息
    async fn update(&self, id: Uuid, params: UserUpdateParams) -> Result<User, Errors> {
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(Errors::UserNotFound)?;

        let mut user_active: UserActiveModel = user.into();

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

        let updated_user = user_active.update(&self.db).await?;
        Ok(updated_user)
    }
}
