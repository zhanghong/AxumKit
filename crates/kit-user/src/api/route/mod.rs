use axum::{
    Router,
    routing::{get, patch, post},
};
use sea_orm::DatabaseConnection;

use crate::api::handler::management;
use crate::api::handler::profile;

/// Create user routes
pub fn user_routes() -> Router<DatabaseConnection> {
    Router::new()
        // Profile routes
        .route("/user/me", get(profile::get_my_profile))
        .route("/user/me", patch(profile::update_my_profile))
        .route("/users/profile", get(profile::get_user_profile_by_handle))
        .route("/users/profile/id", get(profile::get_user_profile_by_id))
        .route("/users/handle/{handle}/available", get(profile::check_handle_available))
        // Management routes
        .route("/users/ban", post(management::ban_user))
        .route("/users/unban", post(management::unban_user))
        .route("/users/roles/grant", post(management::grant_role))
        .route("/users/roles/revoke", post(management::revoke_role))
}
