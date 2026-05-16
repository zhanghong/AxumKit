use async_trait::async_trait;
use kit_errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use redis::AsyncCommands;
use std::collections::HashSet;
use uuid::Uuid;

use crate::domain::model::Session;
use crate::domain::repository::SessionRepository;

/// SessionRepository 实现，使用 Redis 存储会话
pub struct SessionRepositoryImpl {
    redis: RedisClient,
}

impl SessionRepositoryImpl {
    /// 创建新的 SessionRepositoryImpl 实例
    pub fn new(redis: RedisClient) -> Self {
        Self { redis }
    }

    /// Session payload key
    fn session_key(session_id: &str) -> String {
        format!("session:{}", session_id)
    }

    /// Per-session TTL-synced index key
    fn user_session_index_key(user_id: &str, session_id: &str) -> String {
        format!("user_session_idx:{}:{}", user_id, session_id)
    }

    /// User session index prefix
    fn user_session_index_prefix(user_id: &str) -> String {
        format!("user_session_idx:{}:", user_id)
    }

    /// 计算会话的 TTL 秒数
    fn calculate_ttl(session: &Session) -> u64 {
        let now = chrono::Utc::now();
        let ttl_seconds = (session.expires_at - now).num_seconds().max(0) as u64;
        ttl_seconds
    }

    /// 收集用户的所有活跃会话 ID
    async fn collect_user_session_ids(&self, user_id: &str) -> Result<Vec<String>, Errors> {
        let mut conn = self.redis.clone();
        let key_prefix = Self::user_session_index_prefix(user_id);
        let scan_pattern = format!("{}*", key_prefix);
        let mut cursor = 0_u64;
        let mut session_ids = HashSet::new();

        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&scan_pattern)
                .arg("COUNT")
                .arg(200_u32)
                .query_async(&mut conn)
                .await
                .map_err(|e| {
                    Errors::SysInternalError(format!(
                        "Failed to scan user session index '{}': {}",
                        scan_pattern, e
                    ))
                })?;

            for key in keys {
                if let Some(session_id) = key.strip_prefix(&key_prefix) {
                    if !session_id.is_empty() {
                        session_ids.insert(session_id.to_string());
                    }
                }
            }

            if next_cursor == 0 {
                break;
            }
            cursor = next_cursor;
        }

        Ok(session_ids.into_iter().collect())
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    /// 创建会话
    async fn create(&self, session: &Session) -> Result<(), Errors> {
        let ttl_seconds = self.calculate_ttl(session);

        if ttl_seconds == 0 {
            return Err(Errors::SysInternalError(
                "Session already expired".to_string(),
            ));
        }

        let json = serde_json::to_string(session).map_err(|e| {
            Errors::SysInternalError(format!("Session serialization failed: {}", e))
        })?;

        let mut conn = self.redis.clone();
        let session_key = Self::session_key(&session.session_id);
        let index_key = Self::user_session_index_key(&session.user_id, &session.session_id);

        redis::pipe()
            .set_ex(&session_key, json, ttl_seconds)
            .ignore()
            .set_ex(&index_key, "1", ttl_seconds)
            .ignore()
            .query_async::<()>(&mut conn)
            .await
            .map_err(|e| Errors::SysInternalError(format!("Failed to create session: {}", e)))?;

        Ok(())
    }

    /// 根据会话 ID 获取会话
    async fn get(&self, session_id: &str) -> Result<Option<Session>, Errors> {
        let mut conn = self.redis.clone();
        let key = Self::session_key(session_id);

        let session_data: Option<String> = conn.get(&key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis session retrieval failed: {}", e))
        })?;

        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data).map_err(|e| {
                    Errors::SysInternalError(format!("Session deserialization failed: {}", e))
                })?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    /// 删除指定会话
    async fn delete(&self, session_id: &str) -> Result<(), Errors> {
        let mut conn = self.redis.clone();
        let key = Self::session_key(session_id);

        // 从存储的会话 payload 中读取 user_id
        let session_data: Option<String> = conn.get(&key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis session retrieval failed: {}", e))
        })?;

        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data).map_err(|e| {
                    Errors::SysInternalError(format!("Session deserialization failed: {}", e))
                })?;

                let index_key = Self::user_session_index_key(&session.user_id, session_id);
                // 删除会话 payload + 索引 key
                redis::pipe()
                    .del(&key)
                    .ignore()
                    .del(&index_key)
                    .ignore()
                    .query_async::<()>(&mut conn)
                    .await
                    .map_err(|e| {
                        Errors::SysInternalError(format!("Redis session deletion failed: {}", e))
                    })?;
            }
            None => {
                // 会话已过期/删除
            }
        }

        Ok(())
    }

    /// 删除用户的所有会话
    async fn delete_all_by_user(&self, user_id: Uuid) -> Result<(), Errors> {
        let mut conn = self.redis.clone();
        let user_id_str = user_id.to_string();

        let session_ids = self.collect_user_session_ids(&user_id_str).await?;

        let mut pipe = redis::pipe();

        for session_id in &session_ids {
            pipe.del(Self::session_key(session_id)).ignore();
            pipe.del(Self::user_session_index_key(&user_id_str, session_id))
                .ignore();
        }

        pipe.query_async::<()>(&mut conn).await.map_err(|e| {
            Errors::SysInternalError(format!("Failed to delete user sessions: {}", e))
        })?;

        Ok(())
    }

    /// 删除用户的其他会话（保留当前会话）
    async fn delete_other_by_user(
        &self,
        user_id: Uuid,
        current_session_id: &str,
    ) -> Result<(), Errors> {
        let mut conn = self.redis.clone();
        let user_id_str = user_id.to_string();

        let session_ids = self.collect_user_session_ids(&user_id_str).await?;

        // 保留当前会话，删除其他会话
        let other_session_ids: Vec<&String> = session_ids
            .iter()
            .filter(|id| id.as_str() != current_session_id)
            .collect();

        if !other_session_ids.is_empty() {
            let mut pipe = redis::pipe();
            for session_id in &other_session_ids {
                pipe.del(Self::session_key(session_id)).ignore();
                pipe.del(Self::user_session_index_key(&user_id_str, session_id.as_str()))
                    .ignore();
            }

            pipe.query_async::<()>(&mut conn).await.map_err(|e| {
                Errors::SysInternalError(format!("Failed to delete other sessions: {}", e))
            })?;
        }

        Ok(())
    }
}
