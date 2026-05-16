use kit_config::ServerConfig;
use kit_dto::auth::request::ChangeEmailRequest;
use kit_errors::errors::{Errors, ServiceResult};
use crate::domain::repository::{UserRepository, UserUpdateParams};
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

/// Email change data stored in Redis
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailChangeData {
    pub user_id: String,
    pub new_email: String,
}

/// Application service for email operations
#[derive(Clone)]
pub struct EmailApplicationService {
    user_repository: std::sync::Arc<dyn UserRepository>,
}

impl EmailApplicationService {
    /// Create a new email application service instance
    pub fn new(user_repository: std::sync::Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Change email for a user
    /// Sends verification email to the new address
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `worker` - Worker client for sending email
    /// * `user_id` - User ID
    /// * `payload` - Change email request
    pub async fn change_email(
        &self,
        _db: &DatabaseConnection,
        redis: &RedisClient,
        worker: &crate::infrastructure::adapter::worker_client::WorkerClient,
        user_id: Uuid,
        payload: ChangeEmailRequest,
    ) -> ServiceResult<()> {
        let config = ServerConfig::get();

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Verify password
        let password_hash = user.password.ok_or(Errors::UserPasswordNotSet)?;
        Self::verify_password(&payload.password, &password_hash)?;

        // Check new email is different
        if user.email == payload.new_email {
            return Err(Errors::BadRequestError(
                "New email must be different from current email.".to_string(),
            ));
        }

        // Check new email is not already taken
        if self
            .user_repository
            .find_by_email(&payload.new_email)
            .await?
            .is_some()
        {
            return Err(Errors::UserEmailAlreadyExists);
        }

        // Create email change data
        let change_data = EmailChangeData {
            user_id: user.id.to_string(),
            new_email: payload.new_email.clone(),
        };

        // Generate token and store in Redis
        let ttl_seconds = (config.auth_email_change_token_expire_time * 60) as u64;
        let token = Self::generate_secure_token();
        let token_key = kit_constants::email_change_key(&token);

        let json = serde_json::to_string(&change_data).map_err(|e| {
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

        // Send email change verification
        crate::infrastructure::adapter::worker_client::send_email_change_verification(
            worker,
            &payload.new_email,
            &user.handle,
            &token,
            config.auth_email_change_token_expire_time as u64,
        )
        .await?;

        info!(user_id = %user_id, "Email change verification sent");

        Ok(())
    }

    /// Confirm email change using token
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `token` - Email change token
    pub async fn confirm_email_change(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        token: &str,
    ) -> ServiceResult<()> {
        let token_key = kit_constants::email_change_key(token);

        // Get and delete token atomically
        let change_data: EmailChangeData = Self::get_json_and_delete(redis, &token_key)
            .await?
            .ok_or(Errors::TokenInvalidEmailChange)?;

        let user_id =
            Uuid::parse_str(&change_data.user_id).map_err(|_| Errors::TokenInvalidEmailChange)?;

        let txn = db.begin().await?;

        // Check new email is not already taken by another user
        if let Some(existing) = self
            .user_repository
            .find_by_email(&change_data.new_email)
            .await?
        {
            if existing.id != user_id {
                return Err(Errors::UserEmailAlreadyExists);
            }
        }

        // Update email and set verified_at
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    email: Some(change_data.new_email.clone()),
                    verified_at: Some(Some(chrono::Utc::now())),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        info!(user_id = %user_id, "Email changed");

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
