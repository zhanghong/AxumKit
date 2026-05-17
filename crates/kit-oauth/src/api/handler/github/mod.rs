use crate::api::dto::request::{GithubLoginRequest, GithubLinkRequest};
use crate::api::dto::response::OAuthUrlResponse;
use crate::application::{GenerateUrlService, GithubOAuthService, GithubSignInInput, GithubLinkInput};
use crate::infrastructure::config::GithubProvider;
use crate::api::handler::google::{AuthorizeQuery, OAuthHandlerState};
use axum::extract::Query;
use axum::response::Response;
use axum::{extract::State, Json};
use kit_entity::common::OAuthProvider;
use kit_errors::errors::Errors;
use std::sync::Arc;

pub async fn github_authorize(State(state): State<Arc<OAuthHandlerState>>, Query(query): Query<AuthorizeQuery>) -> Result<Json<OAuthUrlResponse>, Errors> {
    let service = GenerateUrlService::new();
    let response = service.generate_url::<GithubProvider>(&state.redis, &query.anonymous_user_id, crate::api::dto::request::OAuthAuthorizeFlow::Login, OAuthProvider::Github).await?;
    Ok(Json(response))
}

pub async fn github_login(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<GithubLoginRequest>) -> Result<Response, Errors> {
    let anonymous_user_id = "temp".to_string();
    let service = GithubOAuthService::new();
    let input = GithubSignInInput { code: payload.code, state: payload.state, anonymous_user_id, user_agent: None, ip_address: None };
    let result = service.sign_in(&state.db, &state.redis, &state.http_client, input).await?;
    crate::api::dto::response::OAuthSignInResponse::from_result(result).into_response_result()
}

pub async fn github_link(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<GithubLinkRequest>) -> Result<axum::http::StatusCode, Errors> {
    let user_id = uuid::Uuid::new_v4();
    let anonymous_user_id = "temp".to_string();
    let service = GithubOAuthService::new();
    let input = GithubLinkInput { user_id, code: payload.code, state: payload.state, anonymous_user_id };
    service.link_oauth(&state.db, &state.redis, &state.http_client, input).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
