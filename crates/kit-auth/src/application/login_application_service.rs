use crate::domain::model::Session;
use crate::domain::service::SessionService;
use kit_dto::auth::request::LoginRequest;
use kit_errors::errors::{Errors, ServiceResult};
use crate::domain::repository::UserRepository;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use tracing::info;

/// Login result enum
pub enum LoginResult {
    /// Session created successfully
    SessionCreated {
        session_id: String,
        remember_me: bool,
    },
    /// TOTP verification required
    TotpRequired(String),
}

/// Application service for login operations
#[derive(Clone)]
pub struct LoginApplicationService {
    user_repository: std::sync::Arc<dyn UserRepository>,
}

impl LoginApplicationService {
    /// Create a new login application service instance
    pub fn new(user_repository: std::sync::Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Login with email and password
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `redis` - Redis connection manager
    /// * `payload` - Login request payload
    /// * `user_agent` - User agent string
    /// * `ip_address` - IP address
    pub async fn login(
        &self,
        _db: &DatabaseConnection,
        redis: &RedisClient,
        payload: LoginRequest,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> ServiceResult<LoginResult> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&payload.email)
            .await?
            .ok_or(Errors::InvalidCredentials)?;

        // Verify password
        let password_hash = user.password.ok_or(Errors::InvalidCredentials)?;
        Self::verify_password(&payload.password, &password_hash)?;

        // Check if TOTP is enabled
        if user.totp_enabled_at.is_some() {
            // Create temp token for TOTP verification
            let temp_token = crate::domain::service::totp_service::TotpTempToken::create(
                redis,
                user.id,
                user_agent,
                ip_address,
                payload.remember_me,
            )
            .await?;

            info!(user_id = %user.id, "Login requires TOTP");
            return Ok(LoginResult::TotpRequired(temp_token.token));
        }

        // Create session
        let session = SessionService::create_session(
            redis,
            user.id.to_string(),
            user_agent,
            ip_address,
        )
        .await?;

        info!(user_id = %user.id, "Login successful");

        Ok(LoginResult::SessionCreated {
            session_id: session.session_id,
            remember_me: payload.remember_me,
        })
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
            .map_err(|_| Errors::InvalidCredentials)
    }
}
