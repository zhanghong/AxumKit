use crate::api::dto::response::{SearchUsersResponse, UserSearchItem};
use crate::api::dto::request::SearchUsersRequest;
use kit_errors::errors::ServiceResult;

/// Search repository trait for user search operations
pub trait SearchRepository: Send + Sync {
    fn search_users(
        &self,
        request: &SearchUsersRequest,
    ) -> impl std::future::Future<Output = ServiceResult<SearchUsersResponse>> + Send;
}
