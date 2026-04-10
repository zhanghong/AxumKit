use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Deserialize,
    Serialize,
    ToSchema,
    Hash,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "permission")]
pub enum Permission {
    #[sea_orm(string_value = "user:manage")]
    #[serde(rename = "user:manage")]
    UserManage,
    
    #[sea_orm(string_value = "user:ban")]
    #[serde(rename = "user:ban")]
    UserBan,
    
    #[sea_orm(string_value = "user:role:manage")]
    #[serde(rename = "user:role:manage")]
    UserRoleManage,
    
    #[sea_orm(string_value = "moderation:manage")]
    #[serde(rename = "moderation:manage")]
    ModerationManage,
    
    #[sea_orm(string_value = "action_log:view")]
    #[serde(rename = "action_log:view")]
    ActionLogView,
    
    #[sea_orm(string_value = "settings:manage")]
    #[serde(rename = "settings:manage")]
    SettingsManage,
    
    #[sea_orm(string_value = "oauth:manage")]
    #[serde(rename = "oauth:manage")]
    OAuthManage,
    
    #[sea_orm(string_value = "search:manage")]
    #[serde(rename = "search:manage")]
    SearchManage,
}

impl Permission {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserManage => "user:manage",
            Self::UserBan => "user:ban",
            Self::UserRoleManage => "user:role:manage",
            Self::ModerationManage => "moderation:manage",
            Self::ActionLogView => "action_log:view",
            Self::SettingsManage => "settings:manage",
            Self::OAuthManage => "oauth:manage",
            Self::SearchManage => "search:manage",
        }
    }

    pub const ALL: &'static [Permission] = &[
        Self::UserManage,
        Self::UserBan,
        Self::UserRoleManage,
        Self::ModerationManage,
        Self::ActionLogView,
        Self::SettingsManage,
        Self::OAuthManage,
        Self::SearchManage,
    ];
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user:manage" => Ok(Self::UserManage),
            "user:ban" => Ok(Self::UserBan),
            "user:role:manage" => Ok(Self::UserRoleManage),
            "moderation:manage" => Ok(Self::ModerationManage),
            "action_log:view" => Ok(Self::ActionLogView),
            "settings:manage" => Ok(Self::SettingsManage),
            "oauth:manage" => Ok(Self::OAuthManage),
            "search:manage" => Ok(Self::SearchManage),
            _ => Err(format!("Unknown permission: {}", s)),
        }
    }
}
