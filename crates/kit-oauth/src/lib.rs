//! kit-oauth: OAuth domain module for AxumKit
//!
//! This crate provides third-party authentication functionality including
//! Google and GitHub OAuth login, account linking, and connection management.

pub mod domain;
pub mod infrastructure;
pub mod application;
pub mod api;

// Re-export commonly used items
pub use domain::model::{OAuthConnection, OAuthConnectionId, OAuthProviderType};
pub use domain::repository::OAuthRepository;
pub use domain::service::OAuthDomainService;

pub use application::{
    CompleteSignupService, FindOrCreateUserService, GenerateUrlService, GithubLinkInput,
    GithubOAuthService, GithubSignInInput, GoogleLinkInput, GoogleOAuthService, GoogleSignInInput,
    ListConnectionsService, OAuthApplicationService, SignInResult, UnlinkService,
};

pub use infrastructure::{
    adapter::{
        exchange_code, fetch_github_user_emails, fetch_github_user_info, fetch_google_user_info,
        generate_auth_url, GithubEmail, GithubUserInfo, GoogleUserInfo,
    },
    config::{GithubProvider, GoogleProvider, OAuthProviderConfig},
    repository::OAuthRepositoryImpl,
};

pub use api::{
    dto::{
        CompleteOAuthSignupRequest, GithubLinkRequest, GithubLoginRequest, GoogleLinkRequest,
        GoogleLoginRequest, OAuthAuthorizeFlow, OAuthConnectionListResponse,
        OAuthConnectionResponse, OAuthPendingSignupResponse, OAuthSignInResponse,
        OAuthUrlResponse, UnlinkOAuthRequest,
    },
    handler::state::OAuthHandlerState,
    route::oauth_routes,
};
