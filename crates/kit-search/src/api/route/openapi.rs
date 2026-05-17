use crate::api::dto::request::{SearchUsersRequest, SortOrder};
use crate::api::dto::response::{SearchUsersResponse, UserSearchItem};
use crate::api::handler::search_users;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        search_users,
    ),
    components(
        schemas(
            SortOrder,
            SearchUsersRequest,
            SearchUsersResponse,
            UserSearchItem,
        )
    ),
    tags(
        (name = "Search", description = "Search endpoints")
    )
)]
pub struct SearchApiDoc;
