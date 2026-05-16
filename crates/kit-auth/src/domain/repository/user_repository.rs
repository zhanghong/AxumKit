use async_trait::async_trait;
use kit_entity::users::Model as User;
use kit_errors::Errors;
use uuid::Uuid;

/// 创建用户所需参数
pub struct NewUser {
    pub email: String,
    pub handle: String,
    pub display_name: String,
    pub password: String,
}

/// 用户更新参数
/// - `Option<T>`: None = 不修改, Some(value) = 修改为指定值
/// - `Option<Option<T>>`: None = 不修改, Some(None) = 设置为 NULL, Some(Some(value)) = 设置为指定值
pub struct UserUpdateParams {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<Option<String>>,
    pub password: Option<Option<String>>,
    pub verified_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub profile_image: Option<Option<String>>,
    pub banner_image: Option<Option<String>>,
    pub totp_secret: Option<Option<String>>,
    pub totp_enabled_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub totp_backup_codes: Option<Option<Vec<String>>>,
}

impl Default for UserUpdateParams {
    fn default() -> Self {
        Self {
            email: None,
            display_name: None,
            bio: None,
            password: None,
            verified_at: None,
            profile_image: None,
            banner_image: None,
            totp_secret: None,
            totp_enabled_at: None,
            totp_backup_codes: None,
        }
    }
}

/// 用户仓库接口
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Errors>;

    /// 根据 handle 查找用户
    async fn find_by_handle(&self, handle: &str) -> Result<Option<User>, Errors>;

    /// 根据 ID 查找用户
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Errors>;

    /// 根据 ID 获取用户，不存在时返回错误
    async fn get_by_id(&self, id: Uuid) -> Result<User, Errors>;

    /// 创建新用户
    async fn create(&self, user: &NewUser) -> Result<User, Errors>;

    /// 更新用户信息
    async fn update(&self, id: Uuid, params: UserUpdateParams) -> Result<User, Errors>;
}
