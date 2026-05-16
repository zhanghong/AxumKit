use crate::domain::service::SessionService;
use kit_config::ServerConfig;
use kit_dto::auth::request::ChangePasswordRequest;
use kit_errors::errors::{Errors, ServiceResult};
use crate::domain::repository::{UserRepository, UserUpdateParams};
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

/// Password reset data stored in Redis
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetData {
    pub user_id: String,
}

/// Application service for password operations
#[derive(Clone)]
pub struct PasswordApplicationService {
    user_repository: std::sync::Arc<dyn UserRepository>,
}

impl PasswordApplicationService {
    /// Create a new password application service instance
    pub fn new(user_repository: std::sync::Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Change password for a user
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `user_id` - User ID
    /// * `session_id` - Current session ID (to keep)
    /// * `payload` - Change password request
    pub async fn change_password(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        user_id: Uuid,
        session_id: &str,
        payload: ChangePasswordRequest,
    ) -> ServiceResult<()> {
        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Verify current password
        let password_hash = user.password.ok_or(Errors::UserPasswordNotSet)?;
        Self::verify_password(&payload.current_password, &password_hash)?;

        // Check new password is different
        if payload.current_password == payload.new_password {
            return Err(Errors::BadRequestError(
                "New password must be different from current password.".to_string(),
            ));
        }

        // Hash new password
        let new_password_hash = Self::hash_password(&payload.new_password)?;

        // Update password
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    password: Some(Some(new_password_hash)),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        // Delete other sessions
        let deleted_count = SessionService::delete_other_sessions(
            redis,
            &user_id.to_string(),
            session_id,
        )
        .await?;

        info!(
            user_id = %user_id,
            invalidated_sessions = deleted_count,
            "Password changed"
        );

        Ok(())
    }

    /// Request password reset
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `worker` - Worker client for sending email
    /// * `email` - User email address
    pub async fn forgot_password(
        &self,
        _db: &DatabaseConnection,
        redis: &RedisClient,
        worker: &crate::infrastructure::adapter::worker_client::WorkerClient,
        email: &str,
    ) -> ServiceResult<()> {
        let config = ServerConfig::get();

        // Find user by email
        let user = self.user_repository.find_by_email(email).await?;

        let user = match user {
            Some(u) => u,
            None => {
                info!("Password reset requested for non-existent email");
                return Ok(());
            }
        };

        // Check user has a password set
        if user.password.is_none() {
            info!("Password reset requested for user without password");
            return Ok(());
        }

        // Create reset data
        let reset_data = PasswordResetData {
            user_id: user.id.to_string(),
        };

        // Generate token and store in Redis
        let ttl_seconds = (config.auth_password_reset_token_expire_time * 60) as u64;
        let token = Self::generate_secure_token();
        let token_key = kit_constants::password_reset_key(&token);

        let json = serde_json::to_string(&reset_data).map_err(|e| {
            Errors::SysInternalError(format!("JSON serialization failed: {}", e))
        })?;

        let mut conn = redis.clone();
        redis::cmd("SETEX")
            .arg(&token_key)
            .arg(ttl_seconds)
            .arg(json)
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| Errors::SysInternalError(format!("Redis SETEX failed: {}", e)))?;

        // Send password reset email
        crate::infrastructure::adapter::worker_client::send_password_reset_email(
            worker,
            &user.email,
            &user.handle,
            &token,
            config.auth_password_reset_token_expire_time as u64,
        )
        .await?;

        info!("Password reset email sent");

        Ok(())
    }

    /// Reset password using token
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `token` - Reset token
    /// * `new_password` - New password
    pub async fn reset_password(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        token: &str,
        new_password: &str,
    ) -> ServiceResult<()> {
        let token_key = kit_constants::password_reset_key(token);

        // Get and delete token atomically
        let reset_data: PasswordResetData = Self::get_json_and_delete(redis, &token_key)
            .await?
            .ok_or(Errors::TokenInvalidReset)?;

        let user_id =
            Uuid::parse_str(&reset_data.user_id).map_err(|_| Errors::TokenInvalidReset)?;

        // Hash new password
        let password_hash = Self::hash_password(new_password)?;

        // Update password
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    password: Some(Some(password_hash)),
                    ..Default::default()
                },
            )
            .await?;

        // Delete all user sessions
        let deleted_count =
            SessionService::delete_all_user_sessions(redis, &user_id.to_string()).await?;

        info!(
            user_id = %user_id,
            invalidated_sessions = deleted_count,
            "Password reset completed"
        );

        Ok(())
    }

    /// Verify password against hash
    fn verify_password(password: &str, hash: &str) -> ServiceResult<()> {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};

        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash).map_err(|e| {
            Errors::SysInternalError(format!("Password hash parsing failed: {}", e))
        })?;

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Errors::PasswordIncorrect)
    }

    /// Hash password using Argon2
    fn hash_password(password: &str) -> ServiceResult<String> {
        use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
        use rand::Rng;

        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut rand::rng());
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Errors::SysInternalError(format!("Password hashing failed: {}", e)))?;

        Ok(password_hash.to_string())
    }

    /// Generate a secure random token
    fn generate_secure_token() -> String {
        use rand::Rng;
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    /// Get JSON value and delete key atomically
    async fn get_json_and_delete<T: serde::de::DeserializeOwned>(
        redis: &RedisClient,
        key: &str,
    ) -> ServiceResult<Option<T>> {
        let mut conn = redis.clone();

        // Use GETDEL if available (Redis 6.2+)
        let value: Option<String> = redis::cmd("GETDEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| Errors::SysInternalError(format!("Redis GETDEL failed: {}", e)))?;

        match value {
            Some(data) => {
                let parsed = serde_json::from_str(&data).map_err(|e| {
                    Errors::SysInternalError(format!("JSON deserialization failed: {}", e))
                })?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }
}
