use crate::application::{LoginApplicationService, LoginResult};
use crate::api::dto::response::create_login_response;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum_extra::{TypedHeader, headers::UserAgent};
use kit_dto::auth::request::LoginRequest;
use kit_dto::auth::response::TotpRequiredResponse;
use kit_dto::validator::json_validator::ValidatedJson;
use kit_errors::errors::Errors;
use std::net::SocketAddr;

/// Extract user agent string from header
fn extract_user_agent(user_agent: Option<TypedHeader<UserAgent>>) -> String {
    user_agent
        .map(|ua| ua.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Extract IP address from headers or connection info
fn extract_ip_address(headers: &HeaderMap, addr: SocketAddr) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

/// Login handler
///
/// Authenticates a user with email and password.
/// Returns a session cookie on success, or a TOTP challenge if 2FA is enabled.
#[utoipa::path(
    post,
    path = "/v0/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 204, description = "Login successful"),
        (status = 202, description = "TOTP required", body = TotpRequiredResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid credentials or password not set"),
        (status = 404, description = "Not Found - User not found"),
        (status = 500, description = "Internal Server Error - Database or Redis error")
    ),
    tag = "Auth"
)]
pub async fn auth_login(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(login_service): State<LoginApplicationService>,
    State(redis): State<redis::aio::ConnectionManager>,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<Response, Errors> {
    let user_agent = extract_user_agent(user_agent);
    let ip_address = extract_ip_address(&headers, addr);

    let result = login_service
        .login(
            &redis,
            &redis,
            payload,
            Some(user_agent),
            Some(ip_address),
        )
        .await?;

    match result {
        LoginResult::SessionCreated {
            session_id,
            remember_me,
        } => create_login_response(session_id, remember_me),
        LoginResult::TotpRequired(temp_token) => {
            Ok(TotpRequiredResponse { temp_token }.into_response())
        }
    }
}
