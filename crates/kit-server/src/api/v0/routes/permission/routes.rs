use axum::{routing::post, Router};

use super::handler::*;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/add", post(add_permission))
        .route("/remove", post(remove_permission))
        .route("/role", post(get_role_permissions))
}
