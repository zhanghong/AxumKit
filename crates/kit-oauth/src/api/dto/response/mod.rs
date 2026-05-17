use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use kit_entity::common::OAuthProvider;
use kit_entity::user_oauth_connections::Model as OAuthConnectionModel;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

pub mod sign_in;
pub use sign_in::{OAuthPendingSignupResponse, OAuthSignInResponse};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OAuthUrlResponse {
    pub auth_url: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OAuthConnectionResponse { pub provider: OAuthProvider, pub created_at: DateTime<Utc> }

impl From<OAuthConnectionModel> for OAuthConnectionResponse {
    fn from(model: OAuthConnectionModel) -> Self { Self { provider: model.provider, created_at: model.created_at } }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthConnectionListResponse { pub connections: Vec<OAuthConnectionResponse> }

impl IntoResponse for OAuthConnectionListResponse {
    fn into_response(self) -> Response { (StatusCode::OK, Json(self)).into_response() }
}
