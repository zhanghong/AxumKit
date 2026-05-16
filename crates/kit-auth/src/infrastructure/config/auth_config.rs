/// 认证相关配置
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// 会话滑动 TTL（小时）
    /// 用户活跃时会话自动延长的时间
    pub session_ttl_hours: i64,

    /// 会话最大生命周期（小时）
    /// 无论用户是否活跃，会话在此时间后强制过期
    pub session_max_lifetime_hours: i64,

    /// 会话刷新阈值（百分比）
    /// 当剩余 TTL 低于此百分比时触发刷新
    pub session_refresh_threshold: u8,

    /// 邮箱验证令牌过期时间（分钟）
    pub email_verification_token_expire_time: i64,

    /// 密码重置令牌过期时间（分钟）
    pub password_reset_token_expire_time: i64,

    /// 邮箱更改令牌过期时间（分钟）
    pub email_change_token_expire_time: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            session_ttl_hours: 168,        // 7 天
            session_max_lifetime_hours: 720, // 30 天
            session_refresh_threshold: 50,   // 50%
            email_verification_token_expire_time: 60, // 1 小时
            password_reset_token_expire_time: 15,     // 15 分钟
            email_change_token_expire_time: 15,       // 15 分钟
        }
    }
}

impl AuthConfig {
    /// 从环境变量创建配置
    pub fn from_env() -> Self {
        use std::env;

        Self {
            session_ttl_hours: env::var("AUTH_SESSION_SLIDING_TTL_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(168)
                .max(0),

            session_max_lifetime_hours: env::var("AUTH_SESSION_MAX_LIFETIME_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(720)
                .max(0),

            session_refresh_threshold: env::var("AUTH_SESSION_REFRESH_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50)
                .min(100),

            email_verification_token_expire_time: env::var(
                "AUTH_EMAIL_VERIFICATION_TOKEN_EXPIRE_TIME",
            )
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60)
            .max(0),

            password_reset_token_expire_time: env::var(
                "AUTH_PASSWORD_RESET_TOKEN_EXPIRE_TIME",
            )
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(15)
            .max(0),

            email_change_token_expire_time: env::var("AUTH_EMAIL_CHANGE_TOKEN_EXPIRE_TIME")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(15)
                .max(0),
        }
    }

    /// 计算会话滑动 TTL 的秒数
    pub fn session_ttl_seconds(&self) -> u64 {
        (self.session_ttl_hours * 3600).max(0) as u64
    }

    /// 计算会话最大生命周期的秒数
    pub fn session_max_lifetime_seconds(&self) -> u64 {
        (self.session_max_lifetime_hours * 3600).max(0) as u64
    }

    /// 计算邮箱验证令牌过期时间的秒数
    pub fn email_verification_token_expire_seconds(&self) -> u64 {
        (self.email_verification_token_expire_time * 60).max(0) as u64
    }

    /// 计算密码重置令牌过期时间的秒数
    pub fn password_reset_token_expire_seconds(&self) -> u64 {
        (self.password_reset_token_expire_time * 60).max(0) as u64
    }

    /// 计算邮箱更改令牌过期时间的秒数
    pub fn email_change_token_expire_seconds(&self) -> u64 {
        (self.email_change_token_expire_time * 60).max(0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AuthConfig::default();
        assert_eq!(config.session_ttl_hours, 168);
        assert_eq!(config.session_max_lifetime_hours, 720);
        assert_eq!(config.session_refresh_threshold, 50);
        assert_eq!(config.email_verification_token_expire_time, 60);
        assert_eq!(config.password_reset_token_expire_time, 15);
        assert_eq!(config.email_change_token_expire_time, 15);
    }

    #[test]
    fn test_session_ttl_seconds() {
        let config = AuthConfig {
            session_ttl_hours: 1,
            ..Default::default()
        };
        assert_eq!(config.session_ttl_seconds(), 3600);
    }

    #[test]
    fn test_token_expire_seconds() {
        let config = AuthConfig {
            email_verification_token_expire_time: 30,
            password_reset_token_expire_time: 10,
            email_change_token_expire_time: 5,
            ..Default::default()
        };
        assert_eq!(config.email_verification_token_expire_seconds(), 1800);
        assert_eq!(config.password_reset_token_expire_seconds(), 600);
        assert_eq!(config.email_change_token_expire_seconds(), 300);
    }
}
