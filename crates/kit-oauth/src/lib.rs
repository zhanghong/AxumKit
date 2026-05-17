//! kit-oauth: OAuth domain module for AxumKit
//!
//! This crate provides third-party authentication functionality including
//! Google and GitHub OAuth login, account linking, and connection management.

// Domain layer
pub mod domain {
    //! Domain layer containing business logic, entities, and repository interfaces
    
    /// Domain models (entities and value objects)
    pub mod model {
        pub use crate::domain::model::oauth_connection::{OAuthConnection, OAuthConnectionId};
        pub use crate::domain::model::oauth_provider::OAuthProviderType;
    }
    
    /// Domain services
    pub mod service {
        pub use crate::domain::service::oauth_service::OAuthDomainService;
    }
    
    /// Repository interfaces
    pub mod repository {
        pub use crate::domain::repository::oauth_repository::OAuthRepository;
    }
}

// Application layer
pub mod application {
    //! Application layer containing use cases and application services
    
    /// Internal types for OAuth operations
    pub mod types {
        pub use crate::application::types::{
            OAuthStateData, OAuthUserResult, PendingSignupData, SignInResult,
        };
    }
    
    pub use crate::application::{
        CompleteSignupService, FindOrCreateUserService, GenerateUrlService,
        GithubOAuthService, GithubSignInInput, GithubLinkInput,
        GoogleOAuthService, GoogleSignInInput, GoogleLinkInput,
        ListConnectionsService, OAuthApplicationService, UnlinkService,
    };
}

// Infrastructure layer
pub mod infrastructure {
    //! Infrastructure layer containing implementations and external integrations
    
    /// OAuth provider configurations
    pub mod config {
        pub use crate::infrastructure::config::oauth_config::{
            GithubProvider, GoogleProvider, OAuthProviderConfig,
        };
    }
    
    /// External API adapters
    pub mod adapter {
        pub use crate::infrastructure::adapter::{
            exchange_code, fetch_github_user_emails, fetch_github_user_info,
            fetch_google_user_info, generate_auth_url, GithubEmail, GithubUserInfo, GoogleUserInfo,
        };
    }
    
    /// Repository implementations
    pub mod repository {
        pub use crate::infrastructure::repository::oauth_repository_impl::OAuthRepositoryImpl;
    }
}

// API layer
pub mod api {
    //! API layer containing HTTP handlers, routes, and DTOs
    
    /// Request/Response DTOs
    pub mod dto {
        pub use crate::api::dto::{
            CompleteOAuthSignupRequest, GithubLinkRequest, GithubLoginRequest,
            GoogleLinkRequest, GoogleLoginRequest, OAuthAuthorizeFlow, OAuthConnectionListResponse,
            OAuthConnectionResponse, OAuthPendingSignupResponse, OAuthSignInResponse,
            OAuthUrlResponse, UnlinkOAuthRequest,
        };
    }
    
    /// HTTP handlers
    pub mod handler {
        pub use crate::api::handler::{
            complete_signup, github_authorize, github_link, github_login, google_authorize,
            google_link, google_login, google_one_tap, list_connections, unlink_connection,
        };
        
        /// Handler state for dependency injection
        pub mod state {
            pub use crate::api::handler::google::OAuthHandlerState;
        }
    }
    
    /// Route definitions
    pub mod route {
        pub use crate::api::route::oauth_routes::oauth_routes;
    }
}

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

// Module declarations
mod domain {
    pub mod model;
    pub mod repository;
    pub mod service;
}

mod infrastructure {
    pub mod adapter;
    pub mod config;
    pub mod repository;
}

mod application {
    pub mod types;
    pub mod complete_signup_service;
    pub mod find_or_create_user_service;
    pub mod generate_url_service;
    pub mod github_service;
    pub mod google_service;
    pub mod list_connections_service;
    pub mod oauth_service;
    pub mod unlink_service;
}

mod api {
    pub mod dto;
    pub mod handler;
    pub mod route;
}
