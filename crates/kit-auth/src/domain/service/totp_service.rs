use kit_errors::errors::{Errors, ServiceResult};
use chrono::{DateTime, Utc};
use rand::Rng;
use rand::RngExt;
use redis::AsyncCommands;
use redis::aio::ConnectionManager as RedisClient;
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, Secret, TOTP};
use uuid::Uuid;

const ISSUER: &str = "Sevenwiki";
const BACKUP_CODE_COUNT: usize = 10;
const BACKUP_CODE_LENGTH: usize = 8;
const BACKUP_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const TEMP_TOKEN_TTL_SECONDS: u64 = 120; // 2 minutes

/// Verify a TOTP code against a secret.
pub fn verify_totp_code(secret_base32: &str, email: &str, code: &str) -> ServiceResult<bool> {
    let secret = Secret::Encoded(secret_base32.to_string())
        .to_bytes()
        .map_err(|_| Errors::TotpInvalidCode)?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
        Some(ISSUER.to_string()),
        email.to_string(),
    )
    .map_err(|_| Errors::TotpInvalidCode)?;

    Ok(totp.check_current(code).unwrap_or(false))
}

/// Generate a set of backup codes.
pub fn generate_backup_codes() -> Vec<String> {
    let mut rng = rand::rng();
    (0..BACKUP_CODE_COUNT)
        .map(|_| {
            (0..BACKUP_CODE_LENGTH)
                .map(|_| {
                    let idx = rng.random_range(0..BACKUP_CODE_CHARSET.len());
                    BACKUP_CODE_CHARSET[idx] as char
                })
                .collect()
        })
        .collect()
}

/// Temporary token for TOTP verification flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpTempToken {
    pub token: String,
    pub user_id: Uuid,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub remember_me: bool,
    pub created_at: DateTime<Utc>,
}

impl TotpTempToken {
    /// Create a new temporary token.
    pub fn new(
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
        remember_me: bool,
    ) -> Self {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        let token = hex::encode(bytes);

        Self {
            token,
            user_id,
            user_agent,
            ip_address,
            remember_me,
            created_at: Utc::now(),
        }
    }

    /// Get the Redis key for this token.
    pub fn redis_key(&self) -> String {
        format!("totp_temp:{}", self.token)
    }

    /// Create and store a new temporary token in Redis.
    pub async fn create(
        redis: &RedisClient,
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
        remember_me: bool,
    ) -> Result<Self, Errors> {
        let temp_token = Self::new(user_id, user_agent, ip_address, remember_me);

        let json = serde_json::to_string(&temp_token).map_err(|e| {
            Errors::SysInternalError(format!("TOTP temp token serialization failed: {}", e))
        })?;

        let mut conn = redis.clone();
        conn.set_ex::<_, _, ()>(&temp_token.redis_key(), json, TEMP_TOKEN_TTL_SECONDS)
            .await
            .map_err(|e| {
                Errors::SysInternalError(format!(
                    "Redis write failed for TOTP temp token: {}",
                    e
                ))
            })?;

        Ok(temp_token)
    }

    /// Get and delete a temporary token from Redis.
    pub async fn get_and_delete(redis: &RedisClient, token: &str) -> Result<Option<Self>, Errors> {
        let key = format!("totp_temp:{}", token);

        let mut conn = redis.clone();
        let data: Option<String> = conn.get_del(&key).await.map_err(|e| {
            Errors::SysInternalError(format!(
                "Redis read-and-delete failed for TOTP temp token '{}': {}",
                key, e
            ))
        })?;

        match data {
            Some(json) => {
                let value = serde_json::from_str(&json).map_err(|e| {
                    Errors::SysInternalError(format!(
                        "TOTP temp token deserialization failed: {}",
                        e
                    ))
                })?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

/// TOTP service for managing TOTP operations.
pub struct TotpService;

impl TotpService {
    /// Verify a TOTP code.
    pub fn verify_code(secret_base32: &str, email: &str, code: &str) -> ServiceResult<bool> {
        verify_totp_code(secret_base32, email, code)
    }

    /// Generate backup codes.
    pub fn generate_backup_codes() -> Vec<String> {
        generate_backup_codes()
    }
}
