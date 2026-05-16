use crate::domain::model::Session;
use crate::domain::service::totp_service::TotpTempToken;
use crate::domain::service::SessionService;
use kit_dto::auth::response::{
    TotpBackupCodesResponse, TotpEnableResponse, TotpSetupResponse, TotpStatusResponse,
};
use kit_errors::errors::{Errors, ServiceResult};
use crate::domain::repository::{UserRepository, UserUpdateParams};
use rand::RngExt;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use totp_rs::{Algorithm, Secret, TOTP};
use tracing::info;
use uuid::Uuid;

const ISSUER: &str = "Sevenwiki";
const BACKUP_CODE_COUNT: usize = 10;
const BACKUP_CODE_LENGTH: usize = 8;
const BACKUP_CODE_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Application service for TOTP operations
#[derive(Clone)]
pub struct TotpApplicationService {
    user_repository: std::sync::Arc<dyn UserRepository>,
}

impl TotpApplicationService {
    /// Create a new TOTP application service instance
    pub fn new(user_repository: std::sync::Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Setup TOTP for a user
    /// Generates a new TOTP secret and returns QR code
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    pub async fn setup(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> ServiceResult<TotpSetupResponse> {
        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Check TOTP not already enabled
        if user.totp_enabled_at.is_some() {
            return Err(Errors::TotpAlreadyEnabled);
        }

        // Generate secret
        let (secret_bytes, secret_base32) = {
            let mut rng = rand::rng();
            let bytes: [u8; 20] = rng.random();
            let secret = Secret::Raw(bytes.to_vec());
            (bytes, secret.to_encoded().to_string())
        };

        // Create TOTP instance
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,  // digits
            1,  // skew
            30, // step
            secret_bytes.to_vec(),
            Some(ISSUER.to_string()),
            user.email.clone(),
        )
        .map_err(|_| Errors::TotpSecretGenerationFailed)?;

        // Generate QR code
        let qr_code_uri = totp.get_url();
        let qr_code_png_base64 = totp
            .get_qr_base64()
            .map_err(|_| Errors::TotpQrGenerationFailed)?;

        // Store secret (not yet enabled)
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    totp_secret: Some(Some(secret_base32)),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        info!(user_id = %user_id, "TOTP setup initiated");

        Ok(TotpSetupResponse {
            qr_code_base64: qr_code_png_base64,
            qr_code_uri,
        })
    }

    /// Enable TOTP for a user
    /// Verifies the code and enables TOTP, returning backup codes
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    /// * `code` - TOTP code to verify
    pub async fn enable(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
        code: &str,
    ) -> ServiceResult<TotpEnableResponse> {
        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Check TOTP not already enabled
        if user.totp_enabled_at.is_some() {
            return Err(Errors::TotpAlreadyEnabled);
        }

        // Get secret
        let secret_base32 = user.totp_secret.clone().ok_or(Errors::TotpNotEnabled)?;

        // Verify code
        if !Self::verify_totp_code(&secret_base32, &user.email, code)? {
            return Err(Errors::TotpInvalidCode);
        }

        // Generate backup codes
        let backup_codes = Self::generate_backup_codes();
        let hashed_codes = Self::hash_backup_codes(&backup_codes);

        // Enable TOTP and store backup codes
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    totp_enabled_at: Some(Some(chrono::Utc::now())),
                    totp_backup_codes: Some(Some(hashed_codes)),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        info!(user_id = %user_id, "TOTP enabled");

        Ok(TotpEnableResponse { backup_codes })
    }

    /// Verify TOTP code and create session
    /// Used during login when TOTP is required
    ///
    /// # Arguments
    /// * `redis` - Redis connection manager
    /// * `token` - TOTP temp token from login
    /// * `code` - TOTP code or backup code
    pub async fn verify(
        &self,
        db: &DatabaseConnection,
        redis: &RedisClient,
        token: &str,
        code: &str,
    ) -> ServiceResult<Session> {
        // Get and delete temp token
        let token_data = TotpTempToken::get_and_delete(redis, token)
            .await?
            .ok_or(Errors::TotpTempTokenInvalid)?;

        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(token_data.user_id).await?;

        // Check TOTP is enabled
        if user.totp_enabled_at.is_none() {
            return Err(Errors::TotpNotEnabled);
        }

        let secret_base32 = user.totp_secret.clone().ok_or(Errors::TotpNotEnabled)?;
        let backup_codes = user.totp_backup_codes.clone().unwrap_or_default();

        // Verify code (6-digit TOTP or 8-digit backup code)
        if code.len() == 6 {
            if !Self::verify_totp_code(&secret_base32, &user.email, code)? {
                return Err(Errors::TotpInvalidCode);
            }
        } else if code.len() == 8 {
            if backup_codes.is_empty() {
                return Err(Errors::TotpBackupCodeExhausted);
            }

            if let Some(idx) = Self::verify_backup_code(code, &backup_codes) {
                // Remove used backup code
                let mut new_codes = backup_codes.clone();
                new_codes.remove(idx);

                self.user_repository
                    .update(
                        token_data.user_id,
                        UserUpdateParams {
                            totp_backup_codes: Some(Some(new_codes)),
                            ..Default::default()
                        },
                    )
                    .await?;
            } else {
                return Err(Errors::TotpInvalidCode);
            }
        } else {
            return Err(Errors::TotpInvalidCode);
        }

        txn.commit().await?;

        // Create session
        let session = SessionService::create_session(
            redis,
            token_data.user_id.to_string(),
            token_data.user_agent,
            token_data.ip_address,
        )
        .await?;

        info!(user_id = %token_data.user_id, "TOTP verified");

        Ok(session)
    }

    /// Disable TOTP for a user
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    /// * `code` - TOTP code or backup code for verification
    pub async fn disable(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
        code: &str,
    ) -> ServiceResult<()> {
        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Check TOTP is enabled
        if user.totp_enabled_at.is_none() {
            return Err(Errors::TotpNotEnabled);
        }

        let secret_base32 = user.totp_secret.clone().ok_or(Errors::TotpNotEnabled)?;
        let backup_codes = user.totp_backup_codes.clone().unwrap_or_default();

        // Verify code (6-digit TOTP or 8-digit backup code)
        if code.len() == 6 {
            if !Self::verify_totp_code(&secret_base32, &user.email, code)? {
                return Err(Errors::TotpInvalidCode);
            }
        } else if code.len() == 8 {
            if Self::verify_backup_code(code, &backup_codes).is_none() {
                return Err(Errors::TotpInvalidCode);
            }
        } else {
            return Err(Errors::TotpInvalidCode);
        }

        // Disable TOTP
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    totp_secret: Some(None),
                    totp_enabled_at: Some(None),
                    totp_backup_codes: Some(None),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        info!(user_id = %user_id, "TOTP disabled");

        Ok(())
    }

    /// Get TOTP status for a user
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    pub async fn status(
        &self,
        _db: &DatabaseConnection,
        user_id: Uuid,
    ) -> ServiceResult<TotpStatusResponse> {
        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        let enabled = user.totp_enabled_at.is_some();

        Ok(TotpStatusResponse {
            enabled,
            enabled_at: user.totp_enabled_at,
            backup_codes_remaining: if enabled {
                Some(user.totp_backup_codes.map(|c| c.len()).unwrap_or(0))
            } else {
                None
            },
        })
    }

    /// Regenerate backup codes for a user
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    /// * `code` - TOTP code for verification
    pub async fn regenerate_backup_codes(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
        code: &str,
    ) -> ServiceResult<TotpBackupCodesResponse> {
        let txn = db.begin().await?;

        // Get user
        let user = self.user_repository.get_by_id(user_id).await?;

        // Check TOTP is enabled
        if user.totp_enabled_at.is_none() {
            return Err(Errors::TotpNotEnabled);
        }

        let secret_base32 = user.totp_secret.clone().ok_or(Errors::TotpNotEnabled)?;

        // Verify code
        if !Self::verify_totp_code(&secret_base32, &user.email, code)? {
            return Err(Errors::TotpInvalidCode);
        }

        // Generate new backup codes
        let backup_codes = Self::generate_backup_codes();
        let hashed_codes = Self::hash_backup_codes(&backup_codes);

        // Store new backup codes
        self.user_repository
            .update(
                user_id,
                UserUpdateParams {
                    totp_backup_codes: Some(Some(hashed_codes)),
                    ..Default::default()
                },
            )
            .await?;

        txn.commit().await?;

        Ok(TotpBackupCodesResponse { backup_codes })
    }

    /// Verify a TOTP code against a secret
    fn verify_totp_code(secret_base32: &str, email: &str, code: &str) -> ServiceResult<bool> {
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

    /// Generate a set of backup codes
    fn generate_backup_codes() -> Vec<String> {
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

    /// Hash backup codes using Blake3
    fn hash_backup_codes(codes: &[String]) -> Vec<String> {
        codes
            .iter()
            .map(|code| blake3::hash(code.as_bytes()).to_string())
            .collect()
    }

    /// Verify a backup code against hashed codes
    /// Returns the index of the matched code if found
    fn verify_backup_code(code: &str, hashed_codes: &[String]) -> Option<usize> {
        let code_hash = blake3::hash(code.as_bytes()).to_string();
        hashed_codes.iter().position(|h| h == &code_hash)
    }
}
