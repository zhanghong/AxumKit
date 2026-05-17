use crate::api::dto::request::CompleteOAuthSignupRequest;
use crate::application::CompleteSignupService;
use crate::api::handler::google::OAuthHandlerState;
use axum::extract::State;
use axum::Json;
use kit_errors::errors::Errors;
use std::sync::Arc;

pub async fn complete_signup(State(state): State<Arc<OAuthHandlerState>>, Json(payload): Json<CompleteOAuthSignupRequest>) -> Result<Json<serde_json::Value>, Errors> {
    let anonymous_user_id = "temp".to_string();
    let service = CompleteSignupService::new();
    let user_id = service.complete_signup(&state.db, &state.redis, &payload.pending_token, &payload.handle, &payload.display_name, &anonymous_user_id).await?;
    Ok(Json(serde_json::json!({ "user_id": user_id.to_string() })))
}
