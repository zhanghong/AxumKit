use async_trait::async_trait;
use crate::domain::model::Session;
use kit_errors::Errors;
use uuid::Uuid;

/// 会话仓库接口
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// 创建会话
    async fn create(&self, session: &Session) -> Result<(), Errors>;

    /// 根据会话 ID 获取会话
    async fn get(&self, session_id: &str) -> Result<Option<Session>, Errors>;

    /// 删除指定会话
    async fn delete(&self, session_id: &str) -> Result<(), Errors>;

    /// 删除用户的所有会话
    async fn delete_all_by_user(&self, user_id: Uuid) -> Result<(), Errors>;

    /// 删除用户的其他会话（保留当前会话）
    async fn delete_other_by_user(
        &self,
        user_id: Uuid,
        current_session_id: &str,
    ) -> Result<(), Errors>;
}
