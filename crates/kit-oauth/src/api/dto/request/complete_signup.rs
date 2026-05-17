use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CompleteOAuthSignupRequest {
    #[validate(length(min = 1, message = "Pending token is required"))]
    pub pending_token: String,
    #[validate(length(min = 3, max = 30, message = "Handle must be 3-30 characters"))]
    pub handle: String,
    #[validate(length(min = 1, max = 50, message = "Display name must be 1-50 characters"))]
    pub display_name: String,
}
