use serde::Deserialize;
use utoipa::ToSchema;
pub mod google;
pub mod github;
pub mod unlink;
pub mod complete_signup;
pub use google::GoogleLoginRequest;
pub use github::GithubLoginRequest;
pub use unlink::UnlinkOAuthRequest;
pub use complete_signup::CompleteOAuthSignupRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum OAuthAuthorizeFlow { Login, Link }
