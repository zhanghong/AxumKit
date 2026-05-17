use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GoogleLoginRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "State is required"))]
    pub state: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GoogleLinkRequest {
    #[validate(length(min = 1, message = "Code is required"))]
    pub code: String,
    #[validate(length(min = 1, message = "State is required"))]
    pub state: String,
}
