use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Session context for request handling.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub user_id: Uuid,
    pub session_id: String,
}

/// User session stored in Redis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub max_expires_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

impl Session {
    /// Create a new session.
    pub fn new(user_id: String, sliding_ttl_hours: i64, max_lifetime_hours: i64) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::hours(sliding_ttl_hours);
        let max_expires_at = now + Duration::hours(max_lifetime_hours);

        Self {
            session_id: Uuid::now_v7().to_string(),
            user_id,
            created_at: now,
            expires_at,
            max_expires_at,
            user_agent: None,
            ip_address: None,
        }
    }

    /// Add client information to the session.
    pub fn with_client_info(
        mut self,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Self {
        self.user_agent = user_agent;
        self.ip_address = ip_address;
        self
    }

    /// Check if the session can be refreshed (within max lifetime).
    pub fn can_refresh(&self) -> bool {
        Utc::now() < self.max_expires_at
    }

    /// Check if the session needs refresh based on threshold.
    pub fn needs_refresh(&self, threshold_percent: u8, sliding_ttl_hours: i64) -> bool {
        let now = Utc::now();
        let remaining = (self.expires_at - now).num_seconds();

        if remaining <= 0 {
            return false;
        }

        let sliding_ttl_seconds = Duration::hours(sliding_ttl_hours).num_seconds();
        if sliding_ttl_seconds <= 0 {
            return false;
        }

        let threshold_percent = threshold_percent.min(100) as i64;
        let threshold_seconds = (sliding_ttl_seconds * threshold_percent) / 100;

        remaining <= threshold_seconds
    }
}
