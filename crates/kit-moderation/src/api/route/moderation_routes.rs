use crate::api::handler::list_moderation_logs;
use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

pub fn moderation_routes() -> Router<DatabaseConnection> {
    Router::new().route("/moderation/logs", get(list_moderation_logs))
}
