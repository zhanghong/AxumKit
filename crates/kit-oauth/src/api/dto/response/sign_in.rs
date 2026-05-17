use axum::http::StatusCode;
use axum::{Json, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;
use crate::application::types::SignInResult;
use kit_errors::errors::Errors;

#[derive(Debug, Serialize, ToSchema)]
#[schema(description = "Response body returned when OAuth sign-in requires profile completion.")]
pub struct OAuthPendingSignupResponse { pub pending_token: String, pub email: String }

impl IntoResponse for OAuthPendingSignupResponse {
    fn into_response(self) -> axum::response::Response { (StatusCode::OK, Json(self)).into_response() }
}

pub enum OAuthSignInResponse {
    Success { session_id: String },
    PendingSignup(OAuthPendingSignupResponse),
}

impl OAuthSignInResponse {
    pub fn from_result(result: SignInResult) -> Self {
        match result {
            SignInResult::Success(session_id) => OAuthSignInResponse::Success { session_id },
            SignInResult::PendingSignup { pending_token, email } => OAuthSignInResponse::PendingSignup(OAuthPendingSignupResponse { pending_token, email }),
        }
    }
    pub fn into_response_result(self) -> Result<axum::response::Response, Errors> {
        match self {
            OAuthSignInResponse::Success { session_id: _ } => Ok(StatusCode::NO_CONTENT.into_response()),
            OAuthSignInResponse::PendingSignup(response) => Ok(response.into_response()),
        }
    }
}
