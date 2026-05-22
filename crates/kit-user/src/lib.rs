//! Kit User - User domain module for AxumKit
//!
//! This crate provides user management functionality following DDD architecture.
//!
//! # Architecture Layers
//!
//! - **Domain Layer**: Core business logic (entities, value objects, services, repository interfaces)
//! - **Application Layer**: Application services coordinating domain operations
//! - **API Layer**: HTTP handlers, routes, and DTOs
//! - **Infrastructure Layer**: Repository implementations and external adapters

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types from domain
pub use domain::model::{
    role::Role,
    permission::Permission,
    user::{self, Model as UserModel},
    user_role::{self, Model as UserRoleModel},
    user_ban::{self, Model as UserBanModel},
};

// Re-export repository traits
pub use domain::repository::{
    user_repository::{UserRepository, UserUpdateParams},
    user_role_repository::UserRoleRepository,
    user_ban_repository::UserBanRepository,
};

// Re-export domain services
pub use domain::service::{
    profile_service::ProfileService,
    user_management_service::UserManagementService,
};

// Re-export application services
pub use application::{
    profile_application_service::ProfileApplicationService,
    user_management_application_service::UserManagementApplicationService,
};

// Re-export API types
pub use api::{
    dto::{
        request::*,
        response::*,
    },
    handler::profile::AuthenticatedUser,
    route::user_routes,
};

// Re-export infrastructure implementations
pub use infrastructure::repository::{
    user_repository_impl::UserRepositoryImpl,
    user_role_repository_impl::UserRoleRepositoryImpl,
    user_ban_repository_impl::UserBanRepositoryImpl,
};

use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Service factory for creating user domain services with default implementations
pub struct UserServiceFactory;

impl UserServiceFactory {
    /// Create a profile application service
    pub fn create_profile_service(
        conn: &DatabaseConnection,
    ) -> ProfileApplicationService {
        let user_repo = UserRepositoryImpl::new(Arc::new(conn.clone()));
        let profile_service = ProfileService::new(Arc::new(user_repo));
        ProfileApplicationService::new(profile_service)
    }

    /// Create a user management application service
    pub fn create_user_management_service(
        conn: &DatabaseConnection,
    ) -> UserManagementApplicationService {
        let user_ban_repo = UserBanRepositoryImpl::new(Arc::new(conn.clone()));
        let user_role_repo = UserRoleRepositoryImpl::new(Arc::new(conn.clone()));
        let user_management_service = UserManagementService::new(
            Arc::new(user_ban_repo),
            Arc::new(user_role_repo),
        );
        UserManagementApplicationService::new(user_management_service)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_as_str() {
        assert_eq!(Role::Mod.as_str(), "mod");
        assert_eq!(Role::Admin.as_str(), "admin");
    }

    #[test]
    fn test_permission_as_str() {
        assert_eq!(Permission::UserManage.as_str(), "user:manage");
        assert_eq!(Permission::UserBan.as_str(), "user:ban");
    }
}
