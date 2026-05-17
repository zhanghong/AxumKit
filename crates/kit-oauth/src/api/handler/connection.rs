use crate::api::dto::request::UnlinkOAuthRequest;
use crate::api::dto::response::OAuthConnectionListResponse;
use crate::application::{ListConnectionsService, UnlinkService};
use crate::api::handler::google::OAuthHandlerState;
use axum::extract::State;
use axum::Json;
use kit_errors::errors::Errors;
use std::sync::Arc;
use uuid::Uuid;

pub async fn list_connections(State(state): State<Arc<OAuthHandlerState>>) -> Result<OAuthConnectionListResponse, Errors> {
    let user_id = Uuid::new_v4();
    let service = ListConnectionsService::new();
    let connections = service.list_connections(&state.db, user_id).await?;
    Ok(connections)
}

pub async fn unlink_connection(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<UnlinkOAuthRequest>) -> Result<axum::http::StatusCode, Errors> {
    let user_id = Uuid::new_v4();
    let service = UnlinkService::new();
    service.unlink_oauth(&state.db, user_id, payload.provider).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}
