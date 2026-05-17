use crate::api::dto::request::SearchUsersRequest;
use crate::api::dto::response::SearchUsersResponse;
use crate::application::SearchApplicationService;
use crate::infrastructure::adapter::MeilisearchClient;
use axum::extract::State;
use kit_errors::errors::Errors;

/// Validated query extractor for search requests
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> axum::extract::FromRequestParts<S> for ValidatedQuery<T>
where
    T: serde::de::DeserializeOwned + validator::Validate,
    S: Send + Sync,
{
    type Rejection = Errors;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum_extra::extract::Query(value) =
            axum_extra::extract::Query::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| Errors::BadRequestError(e.to_string()))?;
        value
            .validate()
            .map_err(|e| Errors::ValidationError(e.to_string()))?;
        Ok(ValidatedQuery(value))
    }
}

#[utoipa::path(
    get,
    path = "/v0/search/users",
    params(SearchUsersRequest),
    responses(
        (status = 200, description = "User search results", body = SearchUsersResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Search"
)]
pub async fn search_users(
    State(client): State<MeilisearchClient>,
    ValidatedQuery(payload): ValidatedQuery<SearchUsersRequest>,
) -> Result<SearchUsersResponse, Errors> {
    let response = SearchApplicationService::search_users(&client, &payload).await?;
    Ok(response)
}
