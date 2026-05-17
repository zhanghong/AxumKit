use crate::api::handler::{complete_signup, github_authorize, github_link, github_login, google_authorize, google_link, google_login, google_one_tap, list_connections, unlink_connection};
use crate::api::handler::google::OAuthHandlerState;
use axum::{routing::{get, post}, Router};
use std::sync::Arc;

pub fn oauth_routes(state: Arc<OAuthHandlerState>) -> Router {
    Router::new()
        .route("/v0/auth/oauth/google/authorize", get(google_authorize))
        .route("/v0/auth/oauth/google/login", post(google_login))
        .route("/v0/auth/oauth/google/link", post(google_link))
        .route("/v0/auth/oauth/google/one-tap", post(google_one_tap))
        .route("/v0/auth/oauth/github/authorize", get(github_authorize))
        .route("/v0/auth/oauth/github/login", post(github_login))
        .route("/v0/auth/oauth/github/link", post(github_link))
        .route("/v0/auth/oauth/connections", get(list_connections))
        .route("/v0/auth/oauth/connections/unlink", post(unlink_connection))
        .route("/v0/auth/oauth/complete-signup", post(complete_signup))
        .with_state(state)
}
