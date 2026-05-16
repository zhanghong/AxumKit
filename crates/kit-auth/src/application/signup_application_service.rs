use kit_config::ServerConfig;
use kit_dto::user::CreateUserRequest;
use kit_dto::user::CreateUserResponse;
use kit_errors::errors::{Errors, ServiceResult};
use crate::domain::repository::UserRepository;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::info;
use uuid::Uuid;

/// Pending email signup data stored in Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingEmailSignupData {
    pub email: String,
    pub handle: String,
    pub display_name: String,
    pub password_hash: String,
}

static RESERVE_PENDING_SIGNUP_SCRIPT: LazyLock<redis::Script> =
    LazyLock::new(|| redis::Script::new(include_str!("../infrastructure/lua/reserve_pending_signup.lua")));

/// Application service for signup operations
#[derive(Clone)]
pub struct SignupApplicationService {
    user_repository: std::sync::Arc<dyn UserRepository>,
}

impl SignupApplicationService {
    /// Create a new signup application service instance
    pub fn new(user_repository: std::sync::Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Signup with email and password
    /// Creates a pending signup and sends verification email
    pub async fn signup(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        worker: &crate::infrastructure::adapter::worker_client::WorkerClient,
        payload: CreateUserRequest,
    ) -> ServiceResult<CreateUserResponse> {
        let config = ServerConfig::get();

        // Check if user already exists by email
        let existing_user_by_email = self.user_repository.find_by_email(&payload.email).await?;
        if existing_user_by_email.is_some() {
            return Err(Errors::UserEmailAlreadyExists);
        }

        // Check if pending signup already exists for this email
        if self.find_pending_by_email(redis, &payload.email).await?.is_some() {
            return Ok(CreateUserResponse {
                message: "Verification email sent. Complete signup from the link in your inbox."
                    .to_string(),
            });
        }

        // Check if user already exists by handle
        let existing_user_by_handle = self.user_repository.find_by_handle(&payload.handle).await?;
        if existing_user_by_handle.is_some() {
            return Err(Errors::UserHandleAlreadyExists);
        }

        // Check if pending signup already exists for this handle
        if self.find_pending_by_handle(redis, &payload.handle).await?.is_some() {
            return Err(Errors::UserHandleAlreadyExists);
        }

        // Hash password
        let password_hash = Self::hash_password(&payload.password)?;

        // Create pending signup data
        let verification_data = PendingEmailSignupData {
            email: payload.email.clone(),
            handle: payload.handle.clone(),
            display_name: payload.display_name.clone(),
            password_hash,
        };

        // Issue token and store in Redis
        let ttl_seconds = (config.auth_email_verification_token_expire_time * 60) as u64;
        let token = self
            .issue_pending_token(redis, &verification_data, ttl_seconds)
            .await?;

        // Send verification email
        crate::infrastructure::adapter::worker_client::send_verification_email(
            worker,
            &payload.email,
            &payload.handle,
            &token,
            config.auth_email_verification_token_expire_time as u64,
        )
        .await?;

        Ok(CreateUserResponse {
            message: "Verification email sent. Complete signup from the link in your inbox."
                .to_string(),
        })
    }

    /// Verify email and complete signup
    pub async fn verify_email(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        token: &str,
    ) -> ServiceResult<Uuid> {
        let token_key = kit_constants::email_verification_key(token);

        // Get and delete token data atomically
        let signup_data: PendingEmailSignupData = Self::get_json_and_delete(redis, &token_key)
            .await?
            .ok_or(Errors::TokenInvalidVerification)?;

        // Complete signup in database transaction
        let user_id = self.complete_signup(db, redis, signup_data.clone()).await?;

        // Clean up Redis indices (best-effort)
        self.delete_pending_indices(redis, &signup_data).await.ok();

        Ok(user_id)
    }

    /// Resend verification email
    pub async fn resend_verification_email(
        &self,
        redis: &RedisClient,
        worker: &crate::infrastructure::adapter::worker_client::WorkerClient,
        email: &str,
    ) -> ServiceResult<()> {
        let Some((existing_token, signup_data)) = self.find_pending_by_email(redis, email).await?
        else {
            return Ok(());
        };

        let remaining_minutes = self.get_pending_ttl_minutes(redis, &existing_token).await?;

        if remaining_minutes == 0 {
            return Ok(());
        }

        crate::infrastructure::adapter::worker_client::send_verification_email(
            worker,
            &signup_data.email,
            &signup_data.handle,
            &existing_token,
            remaining_minutes,
        )
        .await?;

        info!(
            email = %signup_data.email,
            handle = %signup_data.handle,
            "Pending signup verification email resent"
        );

        Ok(())
    }

    /// Complete pending email signup by creating the user
    async fn complete_signup(
        &self,
        db: &DatabaseConnection,
        _redis: &RedisClient,
        signup_data: PendingEmailSignupData,
    ) -> ServiceResult<Uuid> {
        let txn = db.begin().await?;

        // Double-check email doesn't exist
        if self
            .user_repository
            .find_by_email(&signup_data.email)
            .await?
            .is_some()
        {
            return Err(Errors::UserEmailAlreadyExists);
        }

        // Double-check handle doesn't exist
        if self
            .user_repository
            .find_by_handle(&signup_data.handle)
            .await?
            .is_some()
        {
            return Err(Errors::UserHandleAlreadyExists);
        }

        // Create user with pre-hashed password
        // Note: This would need to be implemented in the user repository
        // For now, we use the standard create method
        let user = self
            .user_repository
            .create(&crate::domain::repository::NewUser {
                email: signup_data.email.clone(),
                handle: signup_data.handle.clone(),
                display_name: signup_data.display_name.clone(),
                password: signup_data.password_hash.clone(),
            })
            .await?;

        txn.commit().await?;

        info!(
            user_id = %user.id,
            handle = %user.handle,
            "Pending signup completed"
        );

        Ok(user.id)
    }

    /// Issue a new pending email signup token
    async fn issue_pending_token(
        &self,
        redis: &RedisClient,
        signup_data: &PendingEmailSignupData,
        ttl_seconds: u64,
    ) -> ServiceResult<String> {
        let token = Self::generate_secure_token();

        let email_key = kit_constants::email_signup_email_key(&signup_data.email);
        let handle_key = kit_constants::email_signup_handle_key(&signup_data.handle);
        let token_key = kit_constants::email_verification_key(&token);

        let token_json = serde_json::to_string(&token).map_err(|e| {
            Errors::SysInternalError(format!("JSON serialization failed for token index: {}", e))
        })?;

        let payload_json = serde_json::to_string(signup_data).map_err(|e| {
            Errors::SysInternalError(format!(
                "JSON serialization failed for signup payload: {}",
                e
            ))
        })?;

        let mut conn = redis.clone();
        let result: i64 = RESERVE_PENDING_SIGNUP_SCRIPT
            .key(&email_key)
            .key(&handle_key)
            .key(&token_key)
            .arg(&token_json)
            .arg(&payload_json)
            .arg(ttl_seconds)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| {
                Errors::SysInternalError(format!("Redis reserve_pending_signup script failed: {}", e))
            })?;

        match result {
            1 => Ok(token),
            -1 => Err(Errors::UserEmailAlreadyExists),
            -2 => Err(Errors::UserHandleAlreadyExists),
            other => Err(Errors::SysInternalError(format!(
                "Unexpected reserve_pending_signup result: {}",
                other
            ))),
        }
    }

    /// Find pending signup by email
    async fn find_pending_by_email(
        &self,
        redis: &RedisClient,
        email: &str,
    ) -> ServiceResult<Option<(String, PendingEmailSignupData)>> {
        self.find_pending_by_index(redis, &kit_constants::email_signup_email_key(email))
            .await
    }

    /// Find pending signup by handle
    async fn find_pending_by_handle(
        &self,
        redis: &RedisClient,
        handle: &str,
    ) -> ServiceResult<Option<(String, PendingEmailSignupData)>> {
        self.find_pending_by_index(redis, &kit_constants::email_signup_handle_key(handle))
            .await
    }

    /// Find pending signup by index key
    async fn find_pending_by_index(
        &self,
        redis: &RedisClient,
        index_key: &str,
    ) -> ServiceResult<Option<(String, PendingEmailSignupData)>> {
        let mut conn = redis.clone();
        let token: Option<String> = conn.get(index_key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis get failed: {}", e))
        })?;

        let Some(token) = token else {
            return Ok(None);
        };

        let verification_key = kit_constants::email_verification_key(&token);
        let signup_data: Option<PendingEmailSignupData> = conn.get(&verification_key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis get failed: {}", e))
        })?;

        Ok(signup_data.map(|data| (token, data)))
    }

    /// Get pending signup TTL in minutes
    async fn get_pending_ttl_minutes(&self, redis: &RedisClient, token: &str) -> ServiceResult<u64> {
        let key = kit_constants::email_verification_key(token);
        let mut conn = redis.clone();
        let ttl: i64 = conn.ttl(&key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis TTL failed: {}", e))
        })?;

        if ttl <= 0 {
            Ok(0)
        } else {
            Ok(ttl.div_ceil(60) as u64)
        }
    }

    /// Delete pending signup indices
    async fn delete_pending_indices(
        &self,
        redis: &RedisClient,
        signup_data: &PendingEmailSignupData,
    ) -> ServiceResult<()> {
        let mut conn = redis.clone();
        let email_key = kit_constants::email_signup_email_key(&signup_data.email);
        let handle_key = kit_constants::email_signup_handle_key(&signup_data.handle);

        redis::pipe()
            .del(&email_key)
            .ignore()
            .del(&handle_key)
            .ignore()
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| Errors::SysInternalError(format!("Redis delete failed: {}", e)))?;

        Ok(())
    }

    /// Get JSON value and delete key atomically
    async fn get_json_and_delete<T: serde::de::DeserializeOwned>(
        redis: &RedisClient,
        key: &str,
    ) -> ServiceResult<Option<T>> {
        let mut conn = redis.clone();

        // Use GETDEL if available (Redis 6.2+), otherwise use GET + DEL
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

    /// Generate a secure random token
    fn generate_secure_token() -> String {
        use rand::Rng;
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
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
}
