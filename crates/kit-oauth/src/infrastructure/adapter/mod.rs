pub mod google_adapter;
pub mod github_adapter;
pub mod oauth_client;
pub use google_adapter::{fetch_google_user_info, GoogleUserInfo};
pub use github_adapter::{fetch_github_user_info, fetch_github_user_emails, GithubUserInfo, GithubEmail};
pub use oauth_client::{generate_auth_url, exchange_code};
