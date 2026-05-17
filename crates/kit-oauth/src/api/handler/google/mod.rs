use crate::api::dto::request::{GoogleLoginRequest, GoogleLinkRequest};
use crate::api::dto::response::OAuthUrlResponse;
use crate::application::{GenerateUrlService, GoogleOAuthService, GoogleSignInInput, GoogleLinkInput};
use crate::infrastructure::config::GoogleProvider;
use axum::extract::Query;
use axum::response::Response;
use axum::{extract::State, Json};
use kit_entity::common::OAuthProvider;
use kit_errors::errors::Errors;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct AuthorizeQuery { pub anonymous_user_id: String }

#[derive(Debug, Clone)]
pub struct OAuthHandlerState { pub db: sea_orm::DatabaseConnection, pub redis: redis::aio::ConnectionManager, pub http_client: reqwest::Client }

pub async fn google_authorize(State(state): State<Arc<OAuthHandlerState>>, Query(query): Query<AuthorizeQuery>) -> Result<Json<OAuthUrlResponse>, Errors> {
    let service = GenerateUrlService::new();
    let response = service.generate_url::<GoogleProvider>(&state.redis, &query.anonymous_user_id, crate::api::dto::request::OAuthAuthorizeFlow::Login, OAuthProvider::Google).await?;
    Ok(Json(response))
}

pub async fn google_login(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<GoogleLoginRequest>) -> Result<Response, Errors> {
    let anonymous_user_id = "temp".to_string();
    let service = GoogleOAuthService::new();
    let input = GoogleSignInInput { code: payload.code, state: payload.state, anonymous_user_id, user_agent: None, ip_address: None };
    let result = service.sign_in(&state.db, &state.redis, &state.http_client, input).await?;
    crate::api::dto::response::OAuthSignInResponse::from_result(result).into_response_result()
}

pub async fn google_link(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<GoogleLinkRequest>) -> Result<axum::http::StatusCode, Errors> {
    let user_id = uuid::Uuid::new_v4();
    let anonymous_user_id = "temp".to_string();
    let service = GoogleOAuthService::new();
    let input = GoogleLinkInput { user_id, code: payload.code, state: payload.state, anonymous_user_id };
    service.link_oauth(&state.db, &state.redis, &state.http_client, input).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn google_one_tap(State(_state): State<Arc<OAuthHandlerState>>) -> Result<axum::http::StatusCode, Errors> {
    Ok(axum::http::StatusCode::NOT_IMPLEMENTED)
}
